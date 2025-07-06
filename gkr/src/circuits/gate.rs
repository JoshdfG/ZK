use ark_ff::PrimeField;

#[derive(Clone, Debug, PartialEq)]
pub struct Gate {
    pub left: usize,
    pub right: usize,
    pub output: usize,
    pub operation: Operation,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    ADD,
    MUL,
}
impl Operation {
    pub fn operations<F: PrimeField>(&self, _left: &F, _right: &F) -> F {
        match self {
            Operation::ADD => *_left + *_right,
            Operation::MUL => *_left * *_right,
        }
    }
}
impl Gate {
    pub fn new(left: usize, right: usize, output: usize, operation: Operation) -> Self {
        Self {
            left,
            right,
            output,
            operation,
        }
    }
}
