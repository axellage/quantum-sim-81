
use crate::simulation::circuit_parser::build_circuit_from_data;
use crate::simulation::circuit_validator::{validate_grid_input, QuantumCircuitError};
use crate::simulation::quantum_gate::{QuantumGate};
use crate::simulation::quantum_state::{QuantumState, QuantumStateWrapper, QuantumStep};
use crate::simulation::circuit_parser::{UnparsedCircuit, ParsedCircuit};
use crate::simulation::utils::to_little_endian;
use ndarray::{arr2};
use ndarray::Array2;
use num::Complex;

pub fn simulate_circuit_handler(incoming_data: UnparsedCircuit) -> Result<Vec<QuantumState>, QuantumCircuitError> {
    let validation_result = validate_grid_input(&incoming_data);
    if validation_result.is_err() {
        return Err(validation_result.unwrap_err());
    }

    let parsed_circuit: ParsedCircuit = build_circuit_from_data(incoming_data);

    let simulated_states: Vec<QuantumStep> = simulate_circuit(parsed_circuit);

    let combined_states: Vec<QuantumState> = combine_states_for_frontend(simulated_states);

    Ok(combined_states)
}

fn simulate_circuit(circuit: ParsedCircuit) -> Vec<QuantumStep> {
    let mut states: QuantumStep = QuantumStep {
        states: vec![],
    };

    for i in 0..circuit.circuit[0].gates.len() {
        states.states.push(QuantumStateWrapper {
            state: QuantumState::new(&[0]),
            qubits: vec![i],
        });
    }


    let mut state_list: Vec<QuantumStep> = vec![states];

    for (_step, step_gate) in circuit.circuit.into_iter().enumerate() {
        let mut new_state_list: Vec<QuantumStateWrapper> = vec![];

        for gate in step_gate.gates {
            // Identify qubits that the gate will act on
            let qubits_to_act_on = gate.qubits.clone();
            // Get all previous states
            let all_states = state_list.last().unwrap().states.clone();
            // Collect all states that contain the qubits that the gate will act on
            let filtered_states: Vec<QuantumStateWrapper> = all_states
                .into_iter()
                .filter(|state| state.qubits.iter().any(|qubit| qubits_to_act_on.contains(qubit)))
                .collect();


            // TODO: Create helper function to combine stateWrappers to keep qubit information
            let mut combined_state = QuantumState::new(&[0]);
            let mut qubits_in_combined_state: Vec<usize> = vec![];

            for (i, state) in filtered_states.iter().enumerate() {
                if i == 0 {
                    combined_state = state.state.clone();
                    qubits_in_combined_state = state.qubits.clone();
                } else {
                    combined_state = combined_state.kronecker(state.state.clone());
                    qubits_in_combined_state.extend(state.qubits.clone());
                }
            }

            println!("Applying gate: \n{:?} to state \n{:?}, result: \n{:?}\n\n", gate.gate.clone(), combined_state, combined_state.clone().apply_gate(gate.gate.clone()));

            let new_state_wrapped = QuantumStateWrapper {
                state: combined_state.apply_gate(gate.gate),
                qubits: qubits_in_combined_state,
            };


            new_state_list.push(new_state_wrapped);
        }

        state_list.push(QuantumStep {
            states: new_state_list,
        });
    }
    state_list
}

fn combine_states_for_frontend(simulated_states: Vec<QuantumStep>) -> Vec<QuantumState> {
    let mut combined_states: Vec<QuantumState> = vec![];
    for step in simulated_states.iter(){
        
        let mut current_state: QuantumState = step.states[0].state.clone();
        for entagled_group in step.states.iter().skip(1) {
            current_state = current_state.kronecker(entagled_group.state.clone());
        }
        combined_states.push(to_little_endian(&current_state));
        println!("Combining states:\n {:?}\ninto\n{:?}\n\n", step.states.clone(), current_state.clone());
    }
    combined_states
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;
    use num::Complex;

    #[test]
    fn test_simulate_not() {
        let incoming_data = vec![vec!["X"]];
        let result = simulate_circuit_handler(UnparsedCircuit::from(incoming_data));

        let expected_result = vec![
            QuantumState::new(&[0]),
            QuantumState::new(&[1]),
        ];

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_simulate_hadamard() {
        let incoming_data = vec![vec!["H"]];
        let result = simulate_circuit_handler(UnparsedCircuit::from(incoming_data));

        let expected_result = vec![
            QuantumState::new(&[0]),
            QuantumState {
                col: arr2(&[
                    [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                    [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                ]),
            },
        ];

        assert_eq!(result.unwrap(), expected_result);
    }
    
    #[test]
    fn test_simulate_not_on_index() {
        let incoming_data = vec![vec!["I"], vec!["X"]];
        let result = simulate_circuit_handler(UnparsedCircuit::from(incoming_data));

        let expected_result = vec![
            QuantumState::new(&[0,0]),
            QuantumState::new(&[1,0]),
        ];

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_x_gate_on_index() {
        let incoming_data = vec![vec!["X"], vec!["I"]];
        let result = simulate_circuit_handler(UnparsedCircuit::from(incoming_data));

        let expected_result = vec![
            QuantumState::new(&[0,0]),
            QuantumState::new(&[0,1]),
        ];

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_cnot_gate_on_index() {
        let incoming_data = vec![vec!["X", "C_down"], vec!["I", "X"]];
        let result = simulate_circuit_handler(UnparsedCircuit::from(incoming_data));

        let expected_result = vec![
            QuantumState::new(&[0,0]),
            QuantumState::new(&[0, 1]),
            QuantumState::new(&[1, 1]),
        ];

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_swap_circuit() {
        let incoming_data = vec![vec!["X", "Swap"], vec!["I", "Swap"]];
        let result = simulate_circuit_handler(UnparsedCircuit::from(incoming_data));

        let expected_result = vec![
            QuantumState::new(&[0,0]),
            QuantumState::new(&[0,1]),
            QuantumState::new(&[1,0]),
        ];

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_entanglement_circuit() {
        let incoming_data = UnparsedCircuit::from(vec![vec!["H", "C_down"], vec!["I", "X"]]);
        let result = simulate_circuit_handler(incoming_data);

        let expected_result = vec![
            QuantumState::new(&[0,0]),
            QuantumState::new(&[0]).kronecker(QuantumState {
                col: arr2(&[
                    [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                    [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                ]),
            }),
            QuantumState {
                col: arr2(&[
                    [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                    [Complex::new(0.0, 0.0)],
                    [Complex::new(0.0, 0.0)],
                    [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                ]),
            },
        ];

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_ghz_state_circuit() {
        let incoming_data = UnparsedCircuit::from(
            vec![
                vec!["H", "C_down", "I"],
                vec!["I", "X", "C_down"],
                vec!["I", "I", "X"],
            ]
        );

        let result = simulate_circuit_handler(incoming_data);

        let expected_result = vec![
            QuantumState::new(&[0,0,0]),
            QuantumState::new(&[0,0]).kronecker(QuantumState {
                col: arr2(&[
                    [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                    [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                ]),
            }),
            QuantumState::new(&[0]).kronecker(QuantumState {
                col: arr2(&[
                    [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                    [Complex::new(0.0, 0.0)],
                    [Complex::new(0.0, 0.0)],
                    [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                ]),
            }),
            QuantumState {
                col: arr2(&[
                    [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                    [Complex::new(0.0, 0.0)],
                    [Complex::new(0.0, 0.0)],
                    [Complex::new(0.0, 0.0)],
                    [Complex::new(0.0, 0.0)],
                    [Complex::new(0.0, 0.0)],
                    [Complex::new(0.0, 0.0)],
                    [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                ]),
            },
        ];

        assert_eq!(result.unwrap(), expected_result);
    }
}
