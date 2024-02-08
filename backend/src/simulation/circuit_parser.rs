use crate::simulation::quantum_gate::QuantumGate;
use ndarray::{arr2, Array1};
use num::Complex;

pub fn build_circuit_from_data(grid: Vec<Vec<String>>) -> Array1<QuantumGate> {
    let mut return_list: Vec<QuantumGate> = Vec::new();

    for step in 0..grid.len() {
        let mut step_gate = parse_gate(&grid[0][step]);

        for qubit in 1..grid[step].len() {
            step_gate = step_gate.kronecker(&parse_gate(&grid[qubit][step]));
        }

        return_list.push(step_gate);
    }

    Array1::from(return_list)
}

fn parse_gate(gate_string: &String) -> QuantumGate {
    match gate_string.as_str() {
        "I" => QuantumGate::i_gate(),
        "H" => QuantumGate::h_gate(),
        "X" => QuantumGate::x_gate(),
        "CNOT-1" => QuantumGate::cnot_gate(),
        _ => QuantumGate {
            matrix: arr2(&[[Complex::new(1.0_f64, 0.0_f64)]]),
            size: 1,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::quantum_state::QuantumState;
    use ndarray::Array2;

    #[test]
    fn entagled_state_circuit_test() {
        let mut q0 = Vec::new();
        q0.push("H".to_string());
        q0.push("CNOT-1".to_string());

        let mut q1 = Vec::new();
        q1.push("I".to_string());
        q1.push("CNOT-2".to_string());

        let mut grid = Vec::new();
        grid.push(q0);
        grid.push(q1);

        let circuit = build_circuit_from_data(grid);

        let expected_result: Array2<Complex<f64>> = arr2(&[
            [Complex::new(1.0_f64 / 2.0_f64.sqrt(), 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(1.0_f64 / 2.0_f64.sqrt(), 0.0_f64)],
        ]);

        let mut state = QuantumState::new(2);

        state = state
            .apply_gate_to_qubit(circuit[0].clone(), 0)
            .apply_gate_to_qubit(circuit[1].clone(), 0);

        assert_eq!(state.col, expected_result);
    }

    #[test]
    fn ghz_state_circuit_test() {
        let mut q0 = Vec::new();
        q0.push("H".to_string());
        q0.push("CNOT-1".to_string());
        q0.push("I".to_string());

        let mut q1 = Vec::new();
        q1.push("I".to_string());
        q1.push("CNOT-2".to_string());
        q1.push("CNOT-1".to_string());

        let mut q2 = Vec::new();
        q2.push("I".to_string());
        q2.push("I".to_string());
        q2.push("CNOT-2".to_string());

        let mut grid = Vec::new();
        grid.push(q0);
        grid.push(q1);
        grid.push(q2);

        let circuit = build_circuit_from_data(grid);

        let expected_result: Array2<Complex<f64>> = arr2(&[
            [Complex::new(1.0_f64 / 2.0_f64.sqrt(), 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(0.0_f64, 0.0_f64)],
            [Complex::new(1.0_f64 / 2.0_f64.sqrt(), 0.0_f64)],
        ]);

        let mut state = QuantumState::new(3);
        state = state
            .apply_gate_to_qubit(circuit[0].clone(), 0)
            .apply_gate_to_qubit(circuit[1].clone(), 0)
            .apply_gate_to_qubit(circuit[2].clone(), 0);

        assert_eq!(state.col, expected_result);
    }
}
