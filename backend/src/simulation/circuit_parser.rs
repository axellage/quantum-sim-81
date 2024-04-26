use crate::simulation::quantum_gate::{QuantumGate, QuantumGateWrapper, GatesInTimeStep};
use ndarray::{arr2};
use num::Complex;
use rocket::Either;
use rocket::Either::{Left, Right};

#[derive(Debug, Clone, PartialEq)]
pub struct UnparsedCircuit {
    pub circuit: Vec<Vec<String>>,
}

impl From<Vec<Vec<&str>>> for UnparsedCircuit {
    fn from(circuit: Vec<Vec<&str>>) -> Self {
        let mut new_circuit: Vec<Vec<String>> = Vec::new();
        for row in circuit {
            let mut new_row: Vec<String> = Vec::new();
            for gate in row {
                new_row.push(gate.to_string());
            }
            new_circuit.push(new_row);
        }
        UnparsedCircuit { circuit: new_circuit }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedCircuit {
    pub circuit: Vec<GatesInTimeStep>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntangledQubitGroup {
    pub qubits: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntangledQubitGroupsInTimeStep {
    pub groups: Vec<EntangledQubitGroup>,
}

impl EntangledQubitGroupsInTimeStep {
    pub fn combine_entangled_groups(self, mut group1: EntangledQubitGroup, group2: EntangledQubitGroup) -> EntangledQubitGroupsInTimeStep {
        let mut index = 0;
        let mut return_list: EntangledQubitGroupsInTimeStep = EntangledQubitGroupsInTimeStep { groups: vec![] };
        while index < self.groups.len() {
            let group = self.groups[index].clone();
            if group == group1 {
                if self.groups[index + 1] != group2 {
                    panic!("invalid circuit");
                }
                group1.qubits.append(&mut group2.qubits.clone());
                return_list.groups.push(group1.clone());
                index += 1;
            } else {
                return_list.groups.push(group);
            }
            index += 1;
        }
        return_list
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PartOfMultiGate { control_down, swap }

pub fn build_circuit_from_data(grid: UnparsedCircuit) -> ParsedCircuit {
    let gates_parsed_individually: ParsedCircuit = parse_gates_individually(grid);
    let entangled_groups: Vec<EntangledQubitGroupsInTimeStep> = entangle_qubits(gates_parsed_individually.clone());
    combine_gates_where_necessary(gates_parsed_individually.clone(), entangled_groups)
}

fn parse_gates_individually(unparsed_circuit: UnparsedCircuit) -> ParsedCircuit {
    let mut initial_gates = vec![];
    for step in 0..unparsed_circuit.circuit[0].len() {
        let gates_in_time_step = parse_time_step_individual_gates(unparsed_circuit.clone(), step);
        initial_gates.push(gates_in_time_step);
    }
    ParsedCircuit { circuit: initial_gates }
}

fn entangle_qubits(input: ParsedCircuit) -> Vec<EntangledQubitGroupsInTimeStep> {
    let mut qubit_groups: Vec<EntangledQubitGroupsInTimeStep> = vec![EntangledQubitGroupsInTimeStep { groups: input.circuit[0].gates.iter().map(|gate| EntangledQubitGroup { qubits: gate.qubits.clone() }).collect() }];

    for step in 1..input.circuit.len() {
        let previous_entangled_qubits: EntangledQubitGroupsInTimeStep = qubit_groups[step - 1].clone();
        let mut current_entangled_qubits: EntangledQubitGroupsInTimeStep = previous_entangled_qubits.clone();

        let mut gate_index = 0;
        while gate_index < input.circuit[step].gates.len() {
            let gate: QuantumGateWrapper = input.circuit[step].gates[gate_index].clone();
            if gate.qubits.len() == 2 {
                let first_qubit_group = find_qubits_that_are_entangled_to_qubit(gate.qubits[0], previous_entangled_qubits.clone());
                let second_qubit_group = find_qubits_that_are_entangled_to_qubit(gate.qubits[1], previous_entangled_qubits.clone());

                if first_qubit_group != second_qubit_group {
                    current_entangled_qubits = current_entangled_qubits.combine_entangled_groups(first_qubit_group.clone(), second_qubit_group.clone());
                }
            }
            gate_index += 1;
        }
        qubit_groups.push(current_entangled_qubits);
    }
    qubit_groups
}

fn combine_gates_where_necessary(preparsed_circuit: ParsedCircuit, entangled_groups: Vec<EntangledQubitGroupsInTimeStep>) -> ParsedCircuit {
    let mut updated_steps = vec![preparsed_circuit.circuit[0].clone()];

    for (step_no, step) in preparsed_circuit.circuit.iter().enumerate().skip(1) {
        let entangled_groups = entangled_groups[step_no].clone();
        updated_steps.push(combine_gates_in_time_step(step.clone(), entangled_groups));
    }
    ParsedCircuit { circuit: updated_steps }
}

fn parse_time_step_individual_gates(unparsed_circuit: UnparsedCircuit, step: usize) -> GatesInTimeStep {
    let mut current_gates: GatesInTimeStep = GatesInTimeStep { gates: Vec::new() };
    let mut qubit_no = 0;
    while qubit_no < unparsed_circuit.circuit.len() {
        let unparsed_gate = unparsed_circuit.circuit[qubit_no][step].as_str();
        let parsed_gate_or_part_of_multigate: Either<QuantumGate, PartOfMultiGate> = parse_gate(unparsed_gate);

        if parsed_gate_or_part_of_multigate.is_left() {
            let parsed_gate = parsed_gate_or_part_of_multigate.unwrap_left();
            current_gates.gates.push(QuantumGateWrapper { qubits: vec![qubit_no], gate: parsed_gate });
        } 
        else 
        {
            let gate_part = parsed_gate_or_part_of_multigate.unwrap_right();
            if gate_part == PartOfMultiGate::control_down {
                let gate_underneath = parse_gate(unparsed_circuit.circuit[qubit_no + 1][step].as_str()).unwrap_left();
                let controlled_gate = QuantumGate::c_down(gate_underneath);
                current_gates.gates.push(QuantumGateWrapper { qubits: vec![qubit_no, qubit_no + 1], gate: controlled_gate });
                qubit_no += 1;
            } else if gate_part == PartOfMultiGate::swap {
                let object_underneath = parse_gate(unparsed_circuit.circuit[qubit_no + 1][step].as_str());
                if(object_underneath.is_right()){
                    current_gates.gates.push(QuantumGateWrapper { qubits: vec![qubit_no, qubit_no + 1], gate: QuantumGate::swap_gate() });
                    qubit_no += 1;
                } else {
                    // Found incomplete swap gate, replacing with identity gate
                    current_gates.gates.push(QuantumGateWrapper { qubits: vec![qubit_no], gate: QuantumGate::i_gate() });
                }
            }
        }
        qubit_no += 1;
    }
    current_gates
}

fn combine_gates_in_time_step(step: GatesInTimeStep, entangled_groups: EntangledQubitGroupsInTimeStep) -> GatesInTimeStep {
    let mut gate_index = 1;
    let mut current_step: Vec<QuantumGateWrapper> = vec![];

    let mut previous_entangled_group_of_operand = find_qubits_that_are_entangled_to_qubit(step.gates[0].qubits[0], entangled_groups.clone());
    let mut previous_gate = step.gates[0].clone();
    current_step.push(previous_gate.clone());

    while gate_index < step.gates.len() {
        let gate = step.gates[gate_index].clone();
        let operand_in_gate = gate.qubits[0];
        let entangled_group_of_operand = find_qubits_that_are_entangled_to_qubit(operand_in_gate, entangled_groups.clone());

        let gate_to_push;
        if entangled_group_of_operand == previous_entangled_group_of_operand {
            previous_gate.qubits.append(&mut gate.qubits.clone());
            let large_gate = QuantumGateWrapper { gate: previous_gate.gate.kronecker(gate.gate), qubits: previous_gate.qubits };
            current_step.pop();
            gate_to_push = large_gate;
        } else {
            gate_to_push = gate.clone();
        }
        current_step.push(gate_to_push.clone());
        previous_gate = gate_to_push.clone();
        previous_entangled_group_of_operand = find_qubits_that_are_entangled_to_qubit(gate_to_push.qubits[0], entangled_groups.clone());

        gate_index += 1;
    }

    GatesInTimeStep { gates: current_step }
}

fn find_qubits_that_are_entangled_to_qubit(qubit: usize, entangled_qubit_groups: EntangledQubitGroupsInTimeStep) -> EntangledQubitGroup {
    for (_index, entangled_group) in entangled_qubit_groups.groups.iter().enumerate() {
        if entangled_group.qubits.contains(&qubit) {
            return entangled_group.clone();
        }
    }
    panic!("Qubit not found");
}

fn find_gate_that_acts_upon_qubit(qubit: usize, gates_in_time_step: GatesInTimeStep) -> QuantumGateWrapper {
    for (_gate_no, gate) in gates_in_time_step.gates.iter().enumerate() {
        for (_operand_no, operand) in gate.qubits.iter().enumerate() {
            if operand == &qubit {
                return gate.clone();
            }
        }
    }
    panic!("Qubit not found in any of the GateWrappers");
}

fn parse_gate(gate_string: &str) -> Either<QuantumGate, PartOfMultiGate> {
    match gate_string {
        "I" => Left(QuantumGate::i_gate()),
        "H" => Left(QuantumGate::h_gate()),
        "X" => Left(QuantumGate::x_gate()),
        "Y" => Left(QuantumGate::y_gate()),
        "Z" => Left(QuantumGate::z_gate()),
        "T" => Left(QuantumGate::t_gate()),
        "S" => Left(QuantumGate::s_gate()),
        "Swap" => Right(PartOfMultiGate::swap),
        "C_down" => Right(PartOfMultiGate::control_down),
        _ => panic!("Invalid gate"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::quantum_state::QuantumState;
    use ndarray::Array2;

    #[test]
    fn entangle_testing() {
        let q0 = vec!["H", "C_down", "I"];
        let q1 = vec!["I", "X", "X"];

        let grid = vec![q0, q1];

        let circuit = parse_gates_individually(UnparsedCircuit::from(grid));
        let entangled_groups = entangle_qubits(circuit);

        let expected_result = vec![EntangledQubitGroupsInTimeStep {
            groups: vec![
                EntangledQubitGroup { qubits: vec![0] },
                EntangledQubitGroup { qubits: vec![1] },
            ]
        }, EntangledQubitGroupsInTimeStep {
            groups: vec![
                EntangledQubitGroup { qubits: vec![0, 1] },
            ]
        }, EntangledQubitGroupsInTimeStep {
            groups: vec![
                EntangledQubitGroup { qubits: vec![0, 1] },
            ]
        }];

        assert_eq!(entangled_groups, expected_result);
    }

    #[test]
    fn testing() {
        let q0 = vec!["H", "C_down", "I"];
        let q1 = vec!["I", "X", "X"];

        let grid = vec![q0, q1];

        let circuit = build_circuit_from_data(UnparsedCircuit::from(grid));
    }

    #[test]
    fn parse_swap_circuit() {
        let incoming_data = vec![vec!["X", "Swap"], vec!["I", "Swap"]];
        let result = build_circuit_from_data(UnparsedCircuit::from(incoming_data));

        let expected_result = ParsedCircuit {
            circuit: vec![  GatesInTimeStep { gates: vec![QuantumGateWrapper { gate: QuantumGate::x_gate(), qubits: vec![0] },
                                                          QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![1] }] },
                            GatesInTimeStep { gates: vec![QuantumGateWrapper { gate: QuantumGate::swap_gate(), qubits: vec![0,1] }] }]
        };

        assert_eq!(result, expected_result);
    }

    #[test]
    fn x_gate_circuit_test() {
        let q0 = vec!["X"];
        let grid = vec![q0];

        let circuit: ParsedCircuit = build_circuit_from_data(UnparsedCircuit::from(grid));


        let expected_result: ParsedCircuit = ParsedCircuit {
            circuit: vec![GatesInTimeStep { gates: vec![QuantumGateWrapper { gate: QuantumGate::x_gate(), qubits: vec![0] }] }]
        };

        assert_eq!(circuit, expected_result);
    }

    // Disablar det här för flera gates kan inte agera på samma qubit i samma time step
    /*#[test]
    fn one_qubit_multiple_gates_test() {
        let q0 = vec!["X", "H"];
        let grid = vec![q0];

        let circuit = build_circuit_from_data(UnparsedCircuit::from(grid));
        let expected_result = ParsedCircuit {
            circuit: vec![
                vec![QuantumGateWrapper { gate: QuantumGate::x_gate(), qubits: vec![0] }],
                vec![QuantumGateWrapper { gate: QuantumGate::h_gate(), qubits: vec![0] }],
            ]
        };

        assert_eq!(circuit, expected_result);
    }*/

    #[test]
    fn preparse_bell_state_circuit_test() {
        let q0 = vec!["H", "C_down"];
        let q1 = vec!["I", "X"];

        let grid = vec![q0, q1];

        let circuit = parse_gates_individually(UnparsedCircuit::from(grid));

        let expected_result = ParsedCircuit {
            circuit: vec![
                GatesInTimeStep {
                    gates: vec![
                        QuantumGateWrapper { gate: QuantumGate::h_gate(), qubits: vec![0] },
                        QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![1] }]
                },
                GatesInTimeStep {
                    gates: vec![
                        QuantumGateWrapper { gate: QuantumGate::c_down(QuantumGate::x_gate()), qubits: vec![0, 1] }]
                }]
        };

        assert_eq!(circuit, expected_result);
    }

    #[test]
    fn entangle_bell_state_circuit_test() {
        let q0 = vec!["H", "C_down"];
        let q1 = vec!["I", "X"];

        let grid = vec![q0, q1];

        let circuit = parse_gates_individually(UnparsedCircuit::from(grid));
        let entangled_groups = entangle_qubits(circuit);
        let expected_result = vec![EntangledQubitGroupsInTimeStep {
            groups: vec![
                EntangledQubitGroup { qubits: vec![0] },
                EntangledQubitGroup { qubits: vec![1] },
            ]
        }, EntangledQubitGroupsInTimeStep {
            groups: vec![
                EntangledQubitGroup { qubits: vec![0, 1] },
            ]
        }];

        assert_eq!(entangled_groups, expected_result);
    }

    #[test]
    fn parse_bell_state_circuit_test() {
        let q0 = vec!["H", "C_down"];
        let q1 = vec!["I", "X"];

        let grid = vec![q0, q1];

        let circuit = build_circuit_from_data(UnparsedCircuit::from(grid));

        let expected_result = ParsedCircuit {
            circuit: vec![
                GatesInTimeStep {
                    gates: vec![QuantumGateWrapper { gate: QuantumGate::h_gate(), qubits: vec![0] },
                                QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![1] }]
                },
                GatesInTimeStep { gates: vec![QuantumGateWrapper { gate: QuantumGate::c_down(QuantumGate::x_gate()), qubits: vec![0, 1] }] },
            ]
        };

        assert_eq!(circuit, expected_result);
    }

    #[test]
    fn preparse__ghz_state_circuit_test() {
        let grid = vec![
            vec!["H", "C_down", "I"],
            vec!["I", "X", "C_down"],
            vec!["I", "I", "X"],
        ];

        let circuit = parse_gates_individually(UnparsedCircuit::from(grid));

        let expected_result = ParsedCircuit {
            circuit:
            vec![
                GatesInTimeStep {
                    gates: vec![QuantumGateWrapper { gate: QuantumGate::h_gate(), qubits: vec![0] },
                                QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![1] },
                                QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![2] }]
                },
                GatesInTimeStep {
                    gates: vec![QuantumGateWrapper { gate: QuantumGate::c_down(QuantumGate::x_gate()), qubits: vec![0, 1] },
                                QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![2] }]
                },
                GatesInTimeStep {
                    gates: vec![QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![0] },
                                QuantumGateWrapper { gate: QuantumGate::c_down(QuantumGate::x_gate()), qubits: vec![1, 2] }]
                },
            ]
        };

        println!("{:?}", circuit);

        assert_eq!(circuit, expected_result);
    }

    #[test]
    fn entangle__ghz_state_circuit_test() {
        let grid = vec![
            vec!["H", "C_down", "I"],
            vec!["I", "X", "C_down"],
            vec!["I", "I", "X"],
        ];

        //let circuit = build_circuit_from_data(UnparsedCircuit::from(grid));
        let entangled_qubits = entangle_qubits(parse_gates_individually(UnparsedCircuit::from(grid)));
        let expected_result = vec![EntangledQubitGroupsInTimeStep {
            groups: vec![
                EntangledQubitGroup { qubits: vec![0] },
                EntangledQubitGroup { qubits: vec![1] },
                EntangledQubitGroup { qubits: vec![2] },
            ]
        }, EntangledQubitGroupsInTimeStep {
            groups: vec![
                EntangledQubitGroup { qubits: vec![0, 1] },
                EntangledQubitGroup { qubits: vec![2] },
            ]
        }, EntangledQubitGroupsInTimeStep {
            groups: vec![
                EntangledQubitGroup { qubits: vec![0, 1, 2] },
            ]
        }];

        println!("{:?}", entangled_qubits);

        assert_eq!(entangled_qubits, expected_result);
    }

    #[test]
    fn parse__ghz_state_circuit_test() {
        let grid = vec![
            vec!["H", "C_down", "I"],
            vec!["I", "X", "C_down"],
            vec!["I", "I", "X"],
        ];

        let circuit = build_circuit_from_data(UnparsedCircuit::from(grid));

        let expected_result = ParsedCircuit {
            circuit:
            vec![
                GatesInTimeStep {
                    gates: vec![QuantumGateWrapper { gate: QuantumGate::h_gate(), qubits: vec![0] },
                                QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![1] },
                                QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![2] }]
                },
                GatesInTimeStep {
                    gates: vec![QuantumGateWrapper { gate: QuantumGate::c_down(QuantumGate::x_gate()), qubits: vec![0, 1] },
                                QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![2] }]
                },
                GatesInTimeStep { gates: vec![QuantumGateWrapper { gate: QuantumGate::i_gate().kronecker(QuantumGate::c_down(QuantumGate::x_gate())), qubits: vec![0, 1, 2] }] },
            ]
        };

        println!("{:?}", circuit);

        assert_eq!(circuit, expected_result);
    }

    #[test]
    fn parse__swap_gate_test() {
        let grid = vec![
            vec!["X", "Swap", "H"],
            vec!["I", "Swap", "Z"],
        ];

        let circuit = build_circuit_from_data(UnparsedCircuit::from(grid));

        let expected_result = ParsedCircuit {
            circuit:
            vec![
                GatesInTimeStep {
                    gates: vec![QuantumGateWrapper { gate: QuantumGate::x_gate(), qubits: vec![0] },
                                QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![1] },]
                },
                GatesInTimeStep {
                    gates: vec![QuantumGateWrapper { gate: QuantumGate::swap_gate(), qubits: vec![0, 1] }]
                },
                GatesInTimeStep { gates: vec![QuantumGateWrapper { gate: QuantumGate::h_gate().kronecker(QuantumGate::z_gate()), qubits: vec![0, 1] }] },
            ]
        };

        println!("{:?}", circuit);

        assert_eq!(circuit, expected_result);
    }

    #[test]
    fn parse__swap_gate__odd_amount() {
        let grid = vec![
            vec!["X", "Swap"],
            vec!["I", "Swap"],
            vec!["H", "Swap"],
            vec!["I", "H"],
        ];

        let circuit = build_circuit_from_data(UnparsedCircuit::from(grid));

        let expected_result = ParsedCircuit {
            circuit:
            vec![
                GatesInTimeStep {
                    gates: vec![QuantumGateWrapper { gate: QuantumGate::x_gate(), qubits: vec![0] },
                                QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![1] },
                                QuantumGateWrapper { gate: QuantumGate::h_gate(), qubits: vec![2] },
                                QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![3] },]
                },
                GatesInTimeStep {
                    gates: vec![QuantumGateWrapper { gate: QuantumGate::swap_gate(), qubits: vec![0,1] },
                                QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![2] },
                                QuantumGateWrapper { gate: QuantumGate::h_gate(), qubits: vec![3] },]},
            ]
        };

        println!("{:?}", circuit);

        assert_eq!(circuit, expected_result);
    }
}
