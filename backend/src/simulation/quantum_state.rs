use crate::simulation::quantum_gate::QuantumGate;

use ndarray::Array2;
use ndarray::linalg::kron;
use num::{Complex, ToPrimitive};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuantumStep {
    pub states: Vec<QuantumStateWrapper>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuantumStateWrapper {
    pub qubits: Vec<usize>,
    pub state: QuantumState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuantumState {
    pub col: Array2<Complex<f64>>,
}

impl QuantumState {
    pub fn new(bits: &[usize]) -> QuantumState {
        let no_of_qubits = bits.len();

        if !(1..=6).contains(&no_of_qubits) {
            panic!("Number of qubits must be between 1 and 6, inclusive");
        }

        let mut index = 0_usize;
        for (i, &bit) in bits.iter().enumerate() {
            if bit != 0 && bit != 1 {
                panic!("Bits must be 0 or 1");
            }
            index += bit << (no_of_qubits - i - 1);
        }

        let mut col: Array2<Complex<f64>> =
            Array2::<Complex<f64>>::zeros((2_usize.pow(no_of_qubits as u32), 1));
        col[[index, 0]] = Complex::new(1.0, 0.0);

        QuantumState { col }
    }

    // Calculate the number of qubits in the QuantumState
    pub fn size(&self) -> usize {
        self.col.len().ilog2().to_usize().unwrap()
    }

    // Apply a QuantumGate to a QuantumState, panic if gate and state are not of the same size, if the gate size is 0, return the state unchanged
    pub fn apply_gate(self, gate: QuantumGate) -> QuantumState {
        if gate.size == 0 {
            return self;
        }

        if self.size() != gate.size {
            panic!(
                "Trying to apply a gate for {} qubits to a state with {} qubits",
                gate.size,
                self.size()
            )
        }

        let col = gate.matrix.dot(&self.clone().col);

        QuantumState { col }
    }

    pub fn kronecker(&self, other: QuantumState) -> QuantumState {
        let new_col = kron(&self.col, &other.col);
        QuantumState { col: new_col }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    // Test that a simple state is correctly initialized
    #[test]
    fn test_simple_state() {
        let state = QuantumState::new(&[0]);
        let expected_state = arr2(&[[Complex::new(1.0, 0.0)], [Complex::new(0.0, 0.0)]]);

        assert_eq!(state.col, expected_state);
    }

    // Test that a larger state is correctly initialized
    #[test]
    fn test_large_state() {
        let state = QuantumState::new(&[0, 0, 0]);
        let expected_state = arr2(&[
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);

        assert_eq!(state.col, expected_state);
    }

    // Test advanced state initialization
    #[test]
    fn test_advanced_state() {
        let state = QuantumState::new(&[1, 1, 0]);
        let expected_state = arr2(&[
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);

        assert_eq!(state.col, expected_state);
    }

    // Test that the size of a state is correct
    #[test]
    fn test_size() {
        let state = QuantumState::new(&[0, 0, 0, 0, 0]);
        assert_eq!(state.size(), 5);
    }
}
