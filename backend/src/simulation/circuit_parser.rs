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

#[derive(Debug, Clone, PartialEq)]
pub struct EntangledQubitGroup {
    pub qubits: Vec<usize>,
}

pub fn build_circuit_from_data(grid: UnparsedCircuit) -> ParsedCircuit {
    println!("{:?}", grid);

    let mut gates_parsed_individually: ParsedCircuit = parse_gates_individually(grid);

    let mut parsed_circuit_accounting_for_combined_gates: ParsedCircuit = combine_gates_where_necessary(gates_parsed_individually);

    parsed_circuit_accounting_for_combined_gates
}

fn parse_gates_individually(unparsed_circuit: UnparsedCircuit) -> ParsedCircuit{
    let mut initial_gates = vec![];
    println!("Preparsing");
    for step in 0..unparsed_circuit.circuit[0].len() {
        println!("Step: {}", step);
        let gates_in_time_step = parse_time_step_individual_gates(unparsed_circuit.clone(), step);
        initial_gates.push(gates_in_time_step);
    }
    ParsedCircuit {circuit: initial_gates}
}

fn parse_time_step_individual_gates(unparsed_circuit: UnparsedCircuit, step: usize) -> Vec<QuantumGateWrapper> {
    let mut current_gates: Vec<QuantumGateWrapper> = Vec::new();
    for qubit_no in 0..unparsed_circuit.circuit.len() {
        let unparsed_gate = unparsed_circuit.circuit[qubit_no][step].as_str();
        let parsed_gate = parse_gate(unparsed_gate);
 
        current_gates.push(QuantumGateWrapper { qubits: vec![qubit_no], gate: parsed_gate });
    }
    current_gates
}

fn combine_gates_where_necessary(preparsed_circuit: ParsedCircuit) -> ParsedCircuit {
    println!("Combining gates");
    let mut updated_steps = vec![preparsed_circuit.circuit[0].clone()]; // this might not account for multi qubit gates in the first step
 
    for (step_no, step) in preparsed_circuit.circuit.iter().enumerate().skip(1) {
        println!("Step: {}", step_no);
        let entangled_qubits: Vec<EntangledQubitGroup> = updated_steps[step_no - 1].iter().map(|gate| EntangledQubitGroup{ qubits: gate.qubits.clone()}).collect();
       
        updated_steps.push(account_for_entangled_qubits(entangled_qubits.clone(), preparsed_circuit.circuit[step_no].clone()));
    }
    ParsedCircuit {circuit: updated_steps }
}

fn account_for_entangled_qubits(entangled_qubits_before: Vec<EntangledQubitGroup>, preparsed_gates: Vec<QuantumGateWrapper>) -> Vec<QuantumGateWrapper> {
    let mut new_combined_gates: Vec<QuantumGateWrapper> = vec![];
    let mut gate_index = 0;
    while gate_index < preparsed_gates.clone().len(){
        println!("Iteration {}", gate_index);
        let gate = preparsed_gates[gate_index].clone();

        let mut prev_entangled_group = EntangledQubitGroup {qubits: vec![]};
        for (operand_no, operand) in gate.qubits.iter().enumerate() {
            println!("Operand_no: {}", operand_no);
            
            let entangled_group: EntangledQubitGroup = find_qubits_that_are_entangled_to_qubit(operand.clone(), entangled_qubits_before.clone());
            if(entangled_group == prev_entangled_group){
                continue;
            }
            prev_entangled_group = entangled_group.clone();
            if(entangled_group.qubits[0] == operand.clone()){
                let mut large_gate: QuantumGate = gate.gate.clone();
                for (entangled_qubit_no, entangled_qubit) in entangled_group.qubits.iter().enumerate().skip(1){
                    large_gate = large_gate.kronecker(find_gate_that_acts_upon_qubit(entangled_qubit.clone(), preparsed_gates.clone()).gate);
                    gate_index += 1;
                }
                new_combined_gates.push(QuantumGateWrapper { qubits: entangled_group.qubits.clone(), gate: large_gate});
                prev_entangled_group = entangled_group;
            }
        }
        gate_index += 1;
    }
    new_combined_gates
}

fn find_qubits_that_are_entangled_to_qubit(qubit: usize, entangled_qubit_groups: Vec<EntangledQubitGroup>) -> EntangledQubitGroup {
    for (index, entangled_group) in entangled_qubit_groups.iter().enumerate() {
        if entangled_group.qubits.contains(&qubit) {
            return entangled_group.clone();
        }
    }
    panic!("Qubit not found");
}

fn find_gate_that_acts_upon_qubit(qubit: usize, gates_in_time_step: Vec<QuantumGateWrapper>) -> QuantumGateWrapper {
    for (gate_no, gate) in gates_in_time_step.iter().enumerate() {
        for (operand_no, operand) in gate.qubits.iter().enumerate() {
            if(operand == &qubit){
                return gate.clone();
            }
        }
    }
    panic!("Qubit not found in any of the GateWrappers");
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
    fn test__account_for_entangled_qubits__first_two_entangled(){
        let entangled_groups = vec![
            EntangledQubitGroup { qubits: vec![0,1] }, 
            EntangledQubitGroup { qubits: vec![2] }];

        let gates_in_time_step = vec![
            QuantumGateWrapper { qubits: vec![0], gate: QuantumGate::h_gate()}, 
            QuantumGateWrapper { qubits: vec![1], gate: QuantumGate::x_gate()}, 
            QuantumGateWrapper { qubits: vec![2], gate: QuantumGate::i_gate()}];
        
        let new_parsed_gates: Vec<QuantumGateWrapper> = account_for_entangled_qubits(entangled_groups, gates_in_time_step);
        let expected_result: Vec<QuantumGateWrapper> = vec![
            QuantumGateWrapper { qubits: vec![0, 1], gate: QuantumGate::h_gate().kronecker(QuantumGate::x_gate())}, 
            QuantumGateWrapper { qubits: vec![2], gate: QuantumGate::i_gate()}];
    }

    #[test]
    fn test__account_for_entangled_qubits__last_two_entangled(){
        let entangled_groups = vec![
            EntangledQubitGroup { qubits: vec![0] }, 
            EntangledQubitGroup { qubits: vec![1,2] }];

        let gates_in_time_step = vec![
            QuantumGateWrapper { qubits: vec![0], gate: QuantumGate::h_gate()}, 
            QuantumGateWrapper { qubits: vec![1], gate: QuantumGate::x_gate()}, 
            QuantumGateWrapper { qubits: vec![2], gate: QuantumGate::i_gate()}];
        
        let new_parsed_gates: Vec<QuantumGateWrapper> = account_for_entangled_qubits(entangled_groups, gates_in_time_step);
        let expected_result: Vec<QuantumGateWrapper> = vec![
            QuantumGateWrapper { qubits: vec![0], gate: QuantumGate::h_gate()}, 
            QuantumGateWrapper { qubits: vec![1,2], gate: QuantumGate::x_gate().kronecker(QuantumGate::i_gate())}];
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

    /*#[test]
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
    }*/
}
