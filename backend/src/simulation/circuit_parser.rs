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
    let mut return_list: ParsedCircuit = ParsedCircuit { circuit: Vec::new() };

    for step in 0..grid.circuit[0].len() {
        let mut current_gates: Vec<QuantumGateWrapper> = Vec::new();

        for (i, qubit_line) in grid.circuit.iter().enumerate() {
            let gate: QuantumGate = parse_gate(qubit_line[step].as_str());
            let mut operands: Vec<usize> = vec![i];
            if step != 0 {
                operands = find_qubits_that_are_entangled_to_qubit(i, return_list.circuit[step - 1].clone())
                gate = expand_gate_to_entangled_qubits();
            }
            current_gates.push(QuantumGateWrapper { qubits: operands, gate: gate });
        }

        let mut time_step: Vec<QuantumGateWrapper> = Vec::new();

        return_list.circuit.push(current_gates);
    }

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
