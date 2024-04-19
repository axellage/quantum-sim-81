use ndarray::linalg::kron;
use ndarray::{arr2, Array2};
use num::Complex;


#[derive(Debug, Clone, PartialEq)]
pub struct QuantumGateWrapper {
    pub gate: QuantumGate,
    pub qubits: Vec<usize>,
}

// QuantumGate struct
// Matrix is a 2D array of Complex numbers that represents the gate
// Size is the number of qubits the gate operates on
#[derive(Debug, Clone, PartialEq)]
pub struct QuantumGate {
    pub matrix: Array2<Complex<f64>>,
    pub size: usize,
}

impl QuantumGate {
    pub fn i_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
            ]),
            size: 1,
        }
    }
    pub fn x_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)],
                [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
            ]),
            size: 1,
        }
    }

    pub fn y_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [Complex::new(0.0, 0.0), Complex::new(0.0, -1.0)],
                [Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)],
            ]),
            size: 1,
        }
    }

    pub fn z_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)],
            ]),
            size: 1,
        }
    }

    pub fn h_gate() -> QuantumGate {
        QuantumGate {
            matrix: Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)
                * arr2(&[
                [Complex::new(1.0, 0.0), Complex::new(1.0, 0.0)],
                [Complex::new(1.0, 0.0), Complex::new(-1.0, 0.0)],
            ]),
            size: 1,
        }
    }

    pub fn s_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
                [Complex::new(0.0, 0.0), Complex::new(0.0, 1.0)],
            ]),
            size: 1,
        }
    }

    pub fn t_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(
                        std::f64::consts::FRAC_1_SQRT_2,
                        std::f64::consts::FRAC_1_SQRT_2,
                    ),
                ],
            ]),
            size: 1,
        }
    }

    pub fn cnot_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
            ]),
            size: 2,
        }
    }

    pub fn cz_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(-1.0, 0.0),
                ],
            ]),
            size: 2,
        }
    }

    pub fn swap_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                ],
            ]),
            size: 2,
        }
    }

    pub fn ccnot_gate() -> QuantumGate {
        QuantumGate {
            matrix: arr2(&[
                [
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                ],
                [
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(0.0, 0.0),
                    Complex::new(1.0, 0.0),
                    Complex::new(0.0, 0.0),
                ],
            ]),
            size: 3,
        }
    }

    // Combine two gates using the Kronecker product
    pub fn kronecker(self, other: QuantumGate) -> QuantumGate {
        QuantumGate {
            matrix: kron(&self.matrix, &other.matrix),
            size: self.size + other.size,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::simulation::quantum_state::QuantumState;

    use super::*;

    #[test]
    fn test_x_gate() {
        // X|0> -> |1>
        // X |1> -> |0>
        let state = QuantumState::new(&[0]).apply_gate(QuantumGate::x_gate());
        let final_state = arr2(&[[Complex::new(0.0, 0.0)], [Complex::new(1.0, 0.0)]]);
        assert_eq!(state.col, final_state);

        let state = QuantumState::new(&[1]).apply_gate(QuantumGate::x_gate());
        let final_state = arr2(&[[Complex::new(1.0, 0.0)], [Complex::new(0.0, 0.0)]]);
        assert_eq!(state.col, final_state);
    }

    #[test]
    fn test_i_gate() {
        // I|0> -> |0>
        // I|1> -> |1>
        let state = QuantumState::new(&[0]).apply_gate(QuantumGate::i_gate());
        let final_state = arr2(&[[Complex::new(1.0, 0.0)], [Complex::new(0.0, 0.0)]]);
        assert_eq!(state.col, final_state);

        let state = QuantumState::new(&[1]).apply_gate(QuantumGate::i_gate());
        let final_state = arr2(&[[Complex::new(0.0, 0.0)], [Complex::new(1.0, 0.0)]]);
        assert_eq!(state.col, final_state);
    }

    #[test]
    fn test_y_gate() {
        // Y|0> -> i|1>
        // Y|1> -> -i|0>
        let state = QuantumState::new(&[0]).apply_gate(QuantumGate::y_gate());
        let final_state = arr2(&[[Complex::new(0.0, 0.0)], [Complex::new(0.0, 1.0)]]);
        assert_eq!(state.col, final_state);

        let state = QuantumState::new(&[1]).apply_gate(QuantumGate::y_gate());
        let final_state = arr2(&[[Complex::new(0.0, -1.0)], [Complex::new(0.0, 0.0)]]);
        assert_eq!(state.col, final_state);
    }

    #[test]
    fn test_z_gate() {
        // Z|0> -> |0>
        // Z|1> -> -|1>
        let state = QuantumState::new(&[0]).apply_gate(QuantumGate::z_gate());
        let final_state = arr2(&[[Complex::new(1.0, 0.0)], [Complex::new(0.0, 0.0)]]);
        assert_eq!(state.col, final_state);

        let state = QuantumState::new(&[1]).apply_gate(QuantumGate::z_gate());
        let final_state = arr2(&[[Complex::new(0.0, 0.0)], [Complex::new(-1.0, 0.0)]]);
        assert_eq!(state.col, final_state);
    }

    #[test]
    fn test_h_gate() {
        // H|0> -> (|0> + |1>) / √2
        // H|1> -> (|0> - |1>) / √2
        let state = QuantumState::new(&[0]).apply_gate(QuantumGate::h_gate());
        let final_state = arr2(&[
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
        ]);
        assert_eq!(state.col, final_state);

        let state = QuantumState::new(&[1]).apply_gate(QuantumGate::h_gate());
        let final_state = arr2(&[
            [Complex::new(1.0 / 2.0_f64.sqrt(), 0.0)],
            [Complex::new(-1.0 / 2.0_f64.sqrt(), 0.0)],
        ]);
        assert_eq!(state.col, final_state);
    }

    #[test]
    fn test_cnot_gate() {
        // CNOT|00> -> |00>
        // CNOT|01> -> |01>
        // CNOT|10> -> |11>
        // CNOT|11> -> |10>
        let state = QuantumState::new(&[0, 0]).apply_gate(QuantumGate::cnot_gate());

        let expected_result = arr2(&[
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);
        assert_eq!(state.col, expected_result);

        let state = QuantumState::new(&[0, 1]).apply_gate(QuantumGate::cnot_gate());

        let expected_result = arr2(&[
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);
        assert_eq!(state.col, expected_result);

        let state = QuantumState::new(&[1, 0]).apply_gate(QuantumGate::cnot_gate());

        let expected_result = arr2(&[
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0)],
        ]);

        assert_eq!(state.col, expected_result);

        let state = QuantumState::new(&[1, 1]).apply_gate(QuantumGate::cnot_gate());

        let expected_result = arr2(&[
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);

        assert_eq!(state.col, expected_result);
    }

    #[test]
    fn test_swap_gate() {
        // Swap state of two qubits: |01> should become |10>
        let state = QuantumState::new(&[0, 1]).apply_gate(QuantumGate::swap_gate());

        let expected_result = arr2(&[
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0)],
            [Complex::new(0.0, 0.0)],
        ]);

        assert_eq!(state.col, expected_result);
    }

    #[test]
    fn test_toffoli_gate() {
        // Apply Toffoli gate (CCNOT): |110> should become |111>
        let state = QuantumState::new(&[1, 1, 0]).apply_gate(QuantumGate::ccnot_gate());

        let expected_result = arr2(&[
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(0.0, 0.0)],
            [Complex::new(1.0, 0.0)],
        ]);

        assert_eq!(state.col, expected_result);
    }
}
