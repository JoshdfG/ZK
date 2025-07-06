use crate::circuits::gate::Gate;

pub struct Layer {
    pub gates: Vec<Gate>,
}

impl Layer {
    pub fn new(gates: Vec<Gate>) -> Self {
        Self { gates }
    }

    pub fn update_layer(&mut self, _layer_gates: Gate) {
        self.gates.push(_layer_gates);
    }
}
