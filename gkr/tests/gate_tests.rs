use gkr::circuits::gate::{Gate, Operation};

#[test]
fn test_gate_new() {
    let left = 5;
    let right = 5;
    let output = 25;
    let operation_ = Operation::MUL;
    let new_gate = Gate::new(left, right, output, operation_);
    assert_eq!(new_gate.operation, Operation::MUL);
    assert_eq!(new_gate.output, output);
}

#[test]
fn test_gate_addition() {
    let left = 5;
    let right = 5;
    let output = 10;
    let operation_ = Operation::ADD;
    let new_gate = Gate::new(left, right, output, operation_);
    assert_eq!(new_gate.operation, Operation::ADD);
}

