use crate::gate::Gate;

pub struct Layer {
    pub layers: Vec<Vec<Gate>>,
}

impl Layer {
    pub fn new(layer_one: Vec<Gate>) -> Self {
        Self { layers: vec![layer_one] }
    }

    pub fn update_layer(&mut self, _layer_gates:Vec<Gate>){
        self.layers.push(_layer_gates);
    }
}


