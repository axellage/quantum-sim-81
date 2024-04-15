use crate::simulation::quantum_gate::{QuantumGate, QuantumGateWrapper};
use ndarray::{arr2};
use num::Complex;

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
    pub circuit: Vec<Vec<QuantumGateWrapper>>,
}

pub fn build_circuit_from_data(grid: UnparsedCircuit) -> ParsedCircuit {
    println!("{:?}", grid);

    // Step 1: Parse gates
    let mut initial_gates: Vec<Vec<QuantumGateWrapper>> = Vec::new();
    for step in 0..grid.circuit[0].len() {
        println!("Step: {}", step);

        let mut current_gates: Vec<QuantumGateWrapper> = Vec::new();
        for qubit_no in 0..grid.circuit.len() {
            let gate = grid.circuit[qubit_no][step].as_str();
            let parsed_gate = parse_gate(gate);

            // If size is 0 then qubit is part of a multi qubit gate

            if parsed_gate.size == 0 {
                let mut prev_gate = current_gates.pop().unwrap();
                let mut operands = prev_gate.qubits;
                operands.push(qubit_no);

                prev_gate.qubits = operands;

                current_gates.push(prev_gate);
            } else {
                current_gates.push(QuantumGateWrapper { qubits: vec![qubit_no], gate: parsed_gate });
            }
        }

        println!("Current gate {:?}", current_gates);
        initial_gates.push(current_gates);
    }

    // Step 2: Look up what gates to combine
    // INPUT: Entangled qubits in previous step and gates in current step
    // OUTPUT: [[Gates to be combined, Gates to be combined, ...], [Gates to be combined, Gates to be combined, ...], ...]
    println!("COMBINING GATES");

    let updated_steps: Vec<Vec<QuantumGateWrapper>> = vec![initial_gates[0].clone()];

    for (step_no, step) in initial_gates.iter().enumerate().skip(1) {
        let entangled_qubits: Vec<Vec<usize>> = updated_steps[step_no - 1].iter().map(|gate| gate.qubits.clone()).collect();

        println!("Entangled Qubits {:?}", entangled_qubits);

        let mut to_be_merged: Vec<Vec<(QuantumGate, Vec<usize>)>> = vec![vec![(initial_gates[step_no][0].clone().gate, initial_gates[step_no][0].clone().qubits)]];

        for gate in initial_gates[step_no].iter().skip(1) {
            let gate_qubits = gate.qubits.clone();
        }
    }

    // Step 3: Combine the gates in each list to get final circuit

    let mut return_list: ParsedCircuit = ParsedCircuit { circuit: Vec::new() };

    return_list
}

fn find_qubits_that_are_entangled_to_qubit(qubit: usize, gates_with_operands_in_previous_step: Vec<QuantumGateWrapper>) -> Vec<usize> {
    for (index, gate_with_operands) in gates_with_operands_in_previous_step.iter().enumerate() {
        if gate_with_operands.qubits.contains(&qubit) {
            return gate_with_operands.qubits.clone();
        }
    }
    vec![]
}

fn expand_gate_to_entangled_qubits() -> QuantumGate {
    QuantumGate {
        matrix: arr2(&[[Complex::new(1.0_f64, 0.0_f64)]]),
        size: 0,
    }
}

fn parse_gate(gate_string: &str) -> QuantumGate {
    // Multi qubit gates are only applied once, so we can ignore the subsequent parts
    match gate_string {
        "I" => QuantumGate::i_gate(),
        "H" => QuantumGate::h_gate(),
        "X" => QuantumGate::x_gate(),
        "Y" => QuantumGate::y_gate(),
        "Z" => QuantumGate::z_gate(),
        "T" => QuantumGate::t_gate(),
        "S" => QuantumGate::s_gate(),
        "CZ" => QuantumGate::cz_gate(),
        "SWAP-1" => QuantumGate::swap_gate(),
        "CCNOT-1" => QuantumGate::ccnot_gate(),
        "CNOT-1" => QuantumGate::cnot_gate(),
        "CNOT-2" | "CCNOT-2" | "CCNOT-3" | "SWAP-2" => QuantumGate {
            matrix: arr2(&[[Complex::new(1.0_f64, 0.0_f64)]]),
            size: 0,
        },
        _ => panic!("Invalid gate"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::quantum_state::QuantumState;
    use ndarray::Array2;

    #[test]
    fn testing() {
        let q0 = vec!["H", "CNOT-1", "I"];
        let q1 = vec!["I", "CNOT-2", "X"];

        let grid = vec![q0, q1];

        let circuit = build_circuit_from_data(UnparsedCircuit::from(grid));
    }

    #[test]
    fn x_gate_circuit_test() {
        let q0 = vec!["X"];
        let grid = vec![q0];

        let circuit: ParsedCircuit = build_circuit_from_data(UnparsedCircuit::from(grid));


        let expected_result: ParsedCircuit = ParsedCircuit {
            circuit: vec![vec![QuantumGateWrapper { gate: QuantumGate::x_gate(), qubits: vec![0] }]]
        };

        assert_eq!(circuit, expected_result);
    }

    #[test]
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
    }

    #[test]
    fn bell_state_circuit_test() {
        let q0 = vec!["H", "CNOT-1"];
        let q1 = vec!["I", "CNOT-2"];

        let grid = vec![q0, q1];

        let circuit = build_circuit_from_data(UnparsedCircuit::from(grid));

        let expected_result = ParsedCircuit {
            circuit: vec![
                vec![QuantumGateWrapper { gate: QuantumGate::h_gate(), qubits: vec![0] },
                     QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![1] }],
                vec![QuantumGateWrapper { gate: QuantumGate::cnot_gate(), qubits: vec![0, 1] },
                ],
            ]
        };

        assert_eq!(circuit, expected_result);
    }

    #[test]
    fn ghz_state_circuit_test() {
        let grid = vec![
            vec!["H", "CNOT-1", "I"],
            vec!["I", "CNOT-2", "CNOT-1"],
            vec!["I", "I", "CNOT-2"],
        ];

        let circuit = build_circuit_from_data(UnparsedCircuit::from(grid));

        let expected_result = ParsedCircuit {
            circuit:
            vec![
                vec![QuantumGateWrapper { gate: QuantumGate::h_gate(), qubits: vec![0] },
                     QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![1] },
                     QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![2] }],
                vec![QuantumGateWrapper { gate: QuantumGate::cnot_gate(), qubits: vec![0, 1] },
                     QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![2] }],
                vec![QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![0] },
                     QuantumGateWrapper { gate: QuantumGate::cnot_gate(), qubits: vec![1, 2] }],
            ]
        };

        println!("{:?}", circuit);

        assert_eq!(circuit, expected_result);
    }
}
