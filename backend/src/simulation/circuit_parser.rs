use crate::simulation::quantum_gate::{QuantumGate, QuantumGateWrapper};
use ndarray::{arr2};
use num::Complex;

#[derive(Debug, Clone, PartialEq)]
pub struct UnparsedCircuit {
    pub circuit: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedCircuit {
    pub circuit: Vec<Vec<QuantumGateWrapper>>,
}

pub fn build_circuit_from_data(grid: UnparsedCircuit) -> ParsedCircuit {
    let mut return_list: ParsedCircuit = ParsedCircuit { circuit: Vec::new()};

    for step in 0..grid.circuit[0].len() {
        let mut time_step: Vec<QuantumGateWrapper> = Vec::new();

        for (i, qubit) in grid.circuit.iter().enumerate() {
            let gate = parse_gate(qubit[step].as_str());

            if gate.size > 0 {
                let mut qubits: Vec<usize> = Vec::new();

                for qubit_index in 0..gate.size {
                    qubits.push(i + qubit_index);
                }

                time_step.push(
                    QuantumGateWrapper {
                        gate,
                        qubits,
                    }
                );
            }
        }

        return_list.circuit.push(time_step);
    }

    return_list
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

        let circuit = build_circuit_from_data(grid);


        let expected_result: Vec<Vec<QuantumGateWrapper>> =
            vec![vec![QuantumGateWrapper { gate: QuantumGate::x_gate(), qubits: vec![0] }]];

        assert_eq!(circuit, expected_result);
    }

    #[test]
    fn one_qubit_multiple_gates_test() {
        let q0 = vec!["X", "H"];
        let grid = vec![q0];

        let circuit = build_circuit_from_data(grid);

        let expected_result: Vec<Vec<QuantumGateWrapper>> =
            vec![
                vec![QuantumGateWrapper { gate: QuantumGate::x_gate(), qubits: vec![0] }],
                vec![QuantumGateWrapper { gate: QuantumGate::h_gate(), qubits: vec![0] }],
            ];

        assert_eq!(circuit, expected_result);
    }

    #[test]
    fn bell_state_circuit_test() {
        let q0 = vec!["H", "CNOT-1"];
        let q1 = vec!["I", "CNOT-2"];

        let grid = vec![q0, q1];

        let circuit = build_circuit_from_data(grid);

        let expected_result: Vec<Vec<QuantumGateWrapper>> =
            vec![
                vec![QuantumGateWrapper { gate: QuantumGate::h_gate(), qubits: vec![0] },
                     QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![1] }],
                vec![QuantumGateWrapper { gate: QuantumGate::cnot_gate(), qubits: vec![0, 1] },
                ],
            ];

        assert_eq!(circuit, expected_result);
    }

    #[test]
    fn ghz_state_circuit_test() {
        let grid = vec![
            vec!["H", "CNOT-1", "I"],
            vec!["I", "CNOT-2", "CNOT-1"],
            vec!["I", "I", "CNOT-2"],
        ];

        let circuit = build_circuit_from_data(grid);

        let expected_result: Vec<Vec<QuantumGateWrapper>> =
            vec![
                vec![QuantumGateWrapper { gate: QuantumGate::h_gate(), qubits: vec![0] },
                     QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![1] },
                     QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![2] }],
                vec![QuantumGateWrapper { gate: QuantumGate::cnot_gate(), qubits: vec![0, 1] },
                     QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![2] }],
                vec![QuantumGateWrapper { gate: QuantumGate::i_gate(), qubits: vec![0] },
                     QuantumGateWrapper { gate: QuantumGate::cnot_gate(), qubits: vec![1, 2] }],
            ];

        println!("{:?}", circuit);

        assert_eq!(circuit, expected_result);
    }
}
