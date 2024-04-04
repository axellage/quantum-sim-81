// All rows must be the same length
// All elements must be a valid gate
// If a multi-qubit gate is present, the other parts of that gate must be in the same step
// The number of rows (qubits) in the circuit must be between 1 and 6, inclusive
// Atleast one column must be present

use serde::Serialize;
use crate::simulation::circuit_parser::{UnparsedCircuit};

#[derive(Debug, PartialEq, Serialize)]
pub enum QuantumCircuitError {
    TooManyQubits,
    TooFewQubits,
    InvalidGate,
    InvalidRowLength,
    MultiQubitGateMismatch,
}

// Ensures that all rows are the same length and that there is at least one row
// and that the number of rows is between 1 and 6
pub fn validate_grid_input(grid: &UnparsedCircuit) -> Result<(), QuantumCircuitError> {
    if grid.circuit.is_empty() {
        return Err(QuantumCircuitError::TooFewQubits);
    }

    if grid.circuit.len() > 6 {
        return Err(QuantumCircuitError::TooManyQubits);
    }

    let row_length = grid.circuit[0].len();
    for row in grid.circuit.clone().iter() {
        if row.len() != row_length {
            return Err(QuantumCircuitError::InvalidRowLength);
        }
    }

    // Validate steps (columns)
    for i in 0..row_length {
        let mut col: Vec<String> = Vec::new();
        for row in grid.circuit.clone().iter() {
            col.push(row[i].clone());
        }

        match validate_col(&col) {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

// Ensure that a gate is valid
fn validate_gate(gate: &str) -> bool {
    matches!(
        gate,
        "I" | "H"
            | "X"
            | "Y"
            | "Z"
            | "T"
            | "S"
            | "CZ"
            | "SWAP-1"
            | "CCNOT-1"
            | "CNOT-1"
            | "CNOT-2"
            | "CCNOT-2"
            | "CCNOT-3"
            | "SWAP-2"
    )
}

// If a multi-qubit gate, return the other parts of the gate which must be in the same step
fn is_multi_qubit_gate(gate: &str) -> &str {
    match gate {
        "CNOT-1" => "CNOT-2",
        "CCNOT-1" => "CCNOT-2",
        "CCNOT-2" => "CCNOT-3",
        "SWAP-1" => "SWAP-2",
        _ => "",
    }
}

fn is_sub_gate(gate: &str) -> bool {
    matches!(gate, "CNOT-2" | "CCNOT-2" | "CCNOT-3" | "SWAP-2")
}

fn ensure_multi_qubit_gate(gate: &str, prev_gate: &str) -> Result<(), QuantumCircuitError> {
    if is_sub_gate(gate) && prev_gate.is_empty() && is_multi_qubit_gate(prev_gate) != gate {
        return Err(QuantumCircuitError::MultiQubitGateMismatch);
    }

    Ok(())
}

// Validate a row of gates
// Go through each gate and check if it is valid
// If a multi-qubit gate is present, check if the other parts of the gate are in the same step
// by adding them to a list and removing them if they are found
fn validate_col(row: &Vec<String>) -> Result<(), QuantumCircuitError> {
    let mut next_multi_qubit_gate = "";

    for gate in row {
        if !validate_gate(gate) {
            return Err(QuantumCircuitError::InvalidGate);
        }

        if !next_multi_qubit_gate.is_empty() && gate != next_multi_qubit_gate {
            println!(
                "gate: {}, next_multi_qubit_gate: {}",
                gate, next_multi_qubit_gate
            );
            return Err(QuantumCircuitError::MultiQubitGateMismatch);
        }

        if ensure_multi_qubit_gate(gate, next_multi_qubit_gate).is_err() {
            return Err(QuantumCircuitError::MultiQubitGateMismatch);
        }

        next_multi_qubit_gate = is_multi_qubit_gate(gate);
    }

    if next_multi_qubit_gate.is_empty() {
        Ok(())
    } else {
        Err(QuantumCircuitError::MultiQubitGateMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_grid_input() {
        let valid_grid = vec![vec!["I", "H"], vec!["X", "Y"]];
        let invalid_grid = vec![vec!["I", "H"], vec!["X", "Y", "Z"]];

        assert_eq!(validate_grid_input(&UnparsedCircuit::from(valid_grid)), Ok(()));
        assert_eq!(
            validate_grid_input(&UnparsedCircuit::from(invalid_grid)),
            Err(QuantumCircuitError::InvalidRowLength)
        );
    }

    #[test]
    fn test_validate_gate() {
        let valid_gate = "I";
        let invalid_gate = "A";

        assert_eq!(validate_gate(valid_gate), true);
        assert_eq!(validate_gate(invalid_gate), false);
    }

    #[test]
    fn test_is_multi_qubit_gate() {
        let multi_qubit_gate: &str = "CNOT-1";
        let non_multi_qubit_gate: &str = "I";

        assert_eq!(is_multi_qubit_gate(multi_qubit_gate), "CNOT-2");
        assert_eq!(is_multi_qubit_gate(non_multi_qubit_gate), "");
    }

    #[test]
    fn test_valid_multi_qubit_gates_separated() {
        let separated_multi_qubit_gate_grid = vec![
            vec!["CNOT-1", "I", "CNOT-2"], // CNOT-1 and CNOT-2 separated by an I gate
        ];
        assert_eq!(
            validate_grid_input(&UnparsedCircuit::from(separated_multi_qubit_gate_grid)),
            Err(QuantumCircuitError::MultiQubitGateMismatch)
        );
    }

    #[test]
    fn test_multi_qubit_gate_wrong_order_same_step() {
        let grid = vec![
            vec!["CNOT-2", "CNOT-1"], // CNOT-2 and CNOT-1 in alone in a step
        ];
        assert_eq!(
            validate_grid_input(&UnparsedCircuit::from(grid)),
            Err(QuantumCircuitError::MultiQubitGateMismatch)
        );
    }

    #[test]
    fn test_valid_gates_but_exceed_qubit_limit() {
        let grid = vec![
            vec!["I", "H"],
            vec!["X", "Y"],
            vec!["Z", "T"],
            vec!["S", "I"],
            vec!["H", "X"],
            vec!["Y", "Z"],
            vec!["I", "S"],
        ];
        assert_eq!(
            validate_grid_input(&UnparsedCircuit::from(grid)),
            Err(QuantumCircuitError::TooManyQubits)
        );
    }

    #[test]
    fn test_valid_gates_but_missing_multi_qubit_component() {
        let grid = vec![
            vec!["CNOT-1"], // Missing CNOT-2
        ];
        assert_eq!(
            validate_grid_input(&UnparsedCircuit::from(grid)),
            Err(QuantumCircuitError::MultiQubitGateMismatch)
        );
    }

    #[test]
    fn test_valid_circuit_inconsistent_row_lengths() {
        let grid = vec![vec!["I", "H", "X"], vec!["X", "Y"]];
        assert_eq!(
            validate_grid_input(&UnparsedCircuit::from(grid)),
            Err(QuantumCircuitError::InvalidRowLength)
        );
    }

    #[test]
    fn valid_circuit() {
        let grid = vec![vec!["H", "CNOT-1"], vec!["I", "CNOT-2"]];
        assert_eq!(validate_grid_input(&UnparsedCircuit::from(grid)), Ok(()));
    }

    #[test]
    fn valid_circuit_with_single_gate() {
        let grid = vec![vec!["H"]];
        assert_eq!(validate_grid_input(&UnparsedCircuit::from(grid)), Ok(()));
    }

    #[test]
    fn ending_with_multi_qubit_gate() {
        let grid = vec![vec!["H", "CNOT-2"], vec!["I", "CNOT-2"]];
        assert_eq!(
            validate_grid_input(&UnparsedCircuit::from(grid)),
            Err(QuantumCircuitError::MultiQubitGateMismatch)
        );
    }
}
