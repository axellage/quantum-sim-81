use std::collections::HashSet;
use crate::simulation::circuit_parser::build_circuit_from_data;
use crate::simulation::circuit_validator::{validate_grid_input, QuantumCircuitError};
use crate::simulation::quantum_gate::{QuantumGate, QuantumGateWrapper};
use crate::simulation::quantum_state::{QuantumState, QuantumStateWrapper, QuantumStep};
use crate::simulation::circuit_parser::{UnparsedCircuit, ParsedCircuit};
use crate::simulation::utils::{format_to_complex_container, to_little_endian};
use crate::Step;
use ndarray::{arr2, Array1};
use num::Complex;

pub fn simulate_circuit_handler(incoming_data: UnparsedCircuit) -> Result<Vec<QuantumStep>, QuantumCircuitError> {
    let validation_result = validate_grid_input(&incoming_data);
    if validation_result.is_err() {
        return Err(validation_result.unwrap_err());
    }

    let parsed_circuit: ParsedCircuit = build_circuit_from_data(incoming_data);
    println!("Parsed circuit: {:?}", parsed_circuit);
    let simulated_states: Vec<QuantumStep> = simulate_circuit(parsed_circuit);


    Ok(simulated_states)
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

    for (step, step_gate) in circuit.circuit.into_iter().enumerate() {
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


            let mut combined_gate = QuantumGate {
                matrix: arr2(&[[Complex::new(1.0, 0.0)]]),
                size: 0,
            };

            println!("Combined state: {:?}", combined_state);
            println!("For qubits: {:?}", qubits_in_combined_state);
            println!("Gate: {:?}", gate);


            let mut i = 0;

            for qubit in qubits_in_combined_state.clone() {
                if qubits_in_combined_state.clone().len() == combined_gate.size {
                    break;
                }
                if i < gate.qubits.len() && &qubit == gate.qubits.get(i).unwrap() {
                    combined_gate = combined_gate.kronecker(gate.gate.clone());
                    i += gate.gate.size;
                } else {
                    combined_gate = combined_gate.kronecker(QuantumGate::i_gate());
                    i += 1;
                }
            }

            println!("Combined gate: {:?}", combined_state.clone().apply_gate(combined_gate.clone()));

            let new_state_wrapped = QuantumStateWrapper {
                state: combined_state.apply_gate(combined_gate),
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
            QuantumStep {
                states: vec![QuantumStateWrapper {
                    state: QuantumState::new(&[0]),
                    qubits: vec![0],
                }],
            },
            QuantumStep {
                states: vec![QuantumStateWrapper {
                    state: QuantumState::new(&[1]),
                    qubits: vec![0],
                }],
            },
        ];

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_simulate_hadamard() {
        let incoming_data = vec![vec!["H"]];
        let result = simulate_circuit_handler(UnparsedCircuit::from(incoming_data));

        let expected_result = vec![
            QuantumStep {
                states: vec![QuantumStateWrapper {
                    state: QuantumState::new(&[0]),
                    qubits: vec![0],
                }],
            },
            QuantumStep {
                states: vec![QuantumStateWrapper {
                    state: QuantumState {
                        col: arr2(&[
                            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                        ]),
                    },
                    qubits: vec![0],
                }],
            },
        ];

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_simulate_not_on_index() {
        let incoming_data = vec![vec!["I"], vec!["X"]];
        let result = simulate_circuit_handler(UnparsedCircuit::from(incoming_data));

        let expected_result = vec![
            QuantumStep {
                states: vec![
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![0],
                    },
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![1],
                    },
                ],
            },
            QuantumStep {
                states: vec![
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![0],
                    },
                    QuantumStateWrapper {
                        state: QuantumState::new(&[1]),
                        qubits: vec![1],
                    },
                ],
            },
        ];

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_x_gate_on_index() {
        let incoming_data = vec![vec!["X"], vec!["I"]];
        let result = simulate_circuit_handler(UnparsedCircuit::from(incoming_data));

        let expected_result = vec![
            QuantumStep {
                states: vec![
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![0],
                    },
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![1],
                    },
                ],
            },
            QuantumStep {
                states: vec![
                    QuantumStateWrapper {
                        state: QuantumState::new(&[1]),
                        qubits: vec![0],
                    },
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![1],
                    },
                ],
            },
        ];

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_cnot_gate_on_index() {
        let incoming_data = vec![vec!["X", "C_down"], vec!["I", "X"]];
        let result = simulate_circuit_handler(UnparsedCircuit::from(incoming_data));

        let expected_result = vec![
            QuantumStep {
                states: vec![
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![0],
                    },
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![1],
                    },
                ],
            },
            QuantumStep {
                states: vec![
                    QuantumStateWrapper {
                        state: QuantumState::new(&[1]),
                        qubits: vec![0],
                    },
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![1],
                    },
                ],
            },
            QuantumStep {
                states: vec![
                    QuantumStateWrapper {
                        state: QuantumState::new(&[1, 1]),
                        qubits: vec![0, 1],
                    }
                ],
            },
        ];

        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_entanglement_circuit() {
        let incoming_data = UnparsedCircuit::from(vec![vec!["H", "C_down"], vec!["I", "X"]]);
        let result = simulate_circuit_handler(incoming_data);

        let expected_result = vec![
            QuantumStep {
                states: vec![
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![0],
                    },
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![1],
                    },
                ],
            },
            QuantumStep {
                states: vec![
                    QuantumStateWrapper {
                        state: QuantumState {
                            col: arr2(&[
                                [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                                [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                            ]),
                        },
                        qubits: vec![0],
                    },
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![1],
                    },
                ],
            },
            QuantumStep {
                states: vec![
                    QuantumStateWrapper {
                        state: QuantumState {
                            col: arr2(&[
                                [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                                [Complex::new(0.0, 0.0)],
                                [Complex::new(0.0, 0.0)],
                                [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                            ]),
                        },
                        qubits: vec![0, 1],
                    },
                ],
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
            QuantumStep {
                states: vec![
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![0],
                    },
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![1],
                    },
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![2],
                    },
                ],
            },
            QuantumStep {
                states: vec![
                    QuantumStateWrapper {
                        state: QuantumState {
                            col: arr2(&[
                                [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                                [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                            ]),
                        },
                        qubits: vec![0],
                    },
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![1],
                    },
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![2],
                    },
                ],
            },
            QuantumStep {
                states: vec![
                    QuantumStateWrapper {
                        state: QuantumState {
                            col: arr2(&[
                                [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                                [Complex::new(0.0, 0.0)],
                                [Complex::new(0.0, 0.0)],
                                [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
                            ]),
                        },
                        qubits: vec![0, 1],
                    },
                    QuantumStateWrapper {
                        state: QuantumState::new(&[0]),
                        qubits: vec![2],
                    },
                ],
            },
            QuantumStep {
                states: vec![
                    QuantumStateWrapper {
                        state: QuantumState {
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
                        qubits: vec![0, 1, 2],
                    },
                ],
            },
        ];

        let last_step = QuantumStep {
            states: vec![
                QuantumStateWrapper {
                    state: QuantumState {
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
                    qubits: vec![0, 1, 2],
                },
            ],
        };

        println!("{:?}", result.unwrap().last().unwrap().states);

        println!("{:?}", last_step.states);
    }
}
