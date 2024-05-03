
use crate::simulation::circuit_parser::build_circuit_from_data;
use crate::simulation::circuit_validator::{validate_grid_input, QuantumCircuitError};
use crate::simulation::quantum_gate::{QuantumGate, QuantumGateWrapper};
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

fn initialize_states(circuit: ParsedCircuit) -> Vec<QuantumStep> {
    let mut first_step: QuantumStep = QuantumStep {
        states: vec![],
    };
    for i in 0..circuit.circuit[0].gates.len() {
        first_step.states.push(QuantumStateWrapper {
            state: QuantumState::new(&[0]),
            qubits: vec![i],
        });
    }
    vec![first_step]
}



fn simulate_circuit(circuit: ParsedCircuit) -> Vec<QuantumStep> {
    
    let mut state_list: Vec<QuantumStep> = initialize_states(circuit.clone()); 

    for gates_in_step in circuit.circuit.into_iter() {
        let mut states_in_step: Vec<QuantumStateWrapper> = vec![];

        for gate in gates_in_step.gates {
            let states_in_prev_step = QuantumStep {states: state_list.last().unwrap().states.clone()};
            let state_after_gate = calculate_qubits_state_after_gate(states_in_prev_step, gate);
            
            states_in_step.push(state_after_gate);
        }

        state_list.push(QuantumStep {
            states: states_in_step,
        });
    }
    state_list
}

fn calculate_qubits_state_after_gate(states_in_prev_step: QuantumStep, gate_wrapper: QuantumGateWrapper) -> QuantumStateWrapper {
    let qubits_in_gate = gate_wrapper.qubits.clone();

    let states_in_prev_step_gate_acts_on: Vec<QuantumStateWrapper> = states_in_prev_step.states
        .into_iter()
        .filter(|state| state.qubits.iter().any(|qubit| qubits_in_gate.contains(qubit)))
        .collect();

    let mut state_into_gate = combine_states(states_in_prev_step_gate_acts_on.clone());
    let state_after_gate = state_into_gate.apply_gate(gate_wrapper.gate.clone());

    let state_after_gate_wrapped = QuantumStateWrapper {
        state: state_after_gate,
        qubits: qubits_in_gate,
    };
    state_after_gate_wrapped
}

fn combine_states(states: Vec<QuantumStateWrapper>) -> QuantumState {
    let mut current_state: QuantumState = states[0].state.clone();
    for entagled_group in states.iter().skip(1) {
        current_state = current_state.kronecker(entagled_group.state.clone());
    }
    println!("Combining states:\n {:?}\ninto\n{:?}\n\n", states.clone(), current_state.clone());
    current_state
}

fn combine_states_for_frontend(simulated_states: Vec<QuantumStep>) -> Vec<QuantumState> {
    let mut combined_states: Vec<QuantumState> = vec![];
    for step in simulated_states.iter(){
        let combined_states_in_step = combine_states(step.states.clone());
        combined_states.push(to_little_endian(&combined_states_in_step));
        
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
