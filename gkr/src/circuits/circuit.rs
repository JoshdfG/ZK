use crate::circuits::layer::Layer;
use crate::{circuits::gate::Operation, evaluation::MultilinearPolynomialEV};
use ark_ff::PrimeField;
use std::marker::PhantomData;

pub struct Circuit<F: PrimeField> {
    pub layers: Vec<Layer>,
    _phantom: PhantomData<F>,
}

pub struct CircuitEvaluationResult<F: PrimeField> {
    pub output: Vec<F>,
    pub layer_evaluations: Vec<Vec<F>>,
}

impl<F: PrimeField> Circuit<F> {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self {
            layers,
            _phantom: PhantomData,
        }
    }

    pub fn evaluate(&mut self, values: Vec<F>) -> CircuitEvaluationResult<F> {
        let mut current_input = values;

        let mut reversed_evaluations = Vec::new();
        reversed_evaluations.push(current_input.clone());

        // Iterate through the layers vector: in each iteration, iterate through the gates of each layer
        for layer in self.layers.iter().rev() {
            let max_output_index = layer
                .gates
                .iter()
                .map(|gate| gate.output)
                .max()
                .unwrap_or(0);

            let mut resultant_evaluations = vec![F::zero(); max_output_index + 1];

            // Iterate through the gates vector of each layer:
            // use the left_index, right_index and operator of each Gate struct to perform an operation
            // based on the values in the left and right index positions.
            // The operation is based on the Operator of the Gate: Add or Mul
            for gate in layer.gates.iter() {
                let left_index_value = current_input[gate.left];
                let right_index_value = current_input[gate.right];

                let current_gate_evaluation = match gate.operation {
                    Operation::ADD => left_index_value + right_index_value,
                    Operation::MUL => left_index_value * right_index_value,
                };

                // place the result of the evaluation of each gate at the specified output index
                resultant_evaluations[gate.output] += current_gate_evaluation;
            }

            current_input = resultant_evaluations;
            reversed_evaluations.push(current_input.clone());
        }

        reversed_evaluations.reverse();

        CircuitEvaluationResult {
            output: reversed_evaluations[0].clone(),
            layer_evaluations: reversed_evaluations,
        }
    }

    // This function gets the evaluations of a layer: Vec<F> whose index is passed as layer_index,
    // then it converts it to a Multilinear polynomial
    // This will be used for the MLE: Multilinear Extension
    pub fn w_i_polynomial(
        circuit_evaluation: &CircuitEvaluationResult<F>,
        layer_index: usize,
    ) -> MultilinearPolynomialEV<F> {
        assert!(
            layer_index < circuit_evaluation.layer_evaluations.len(),
            "layer index out of bounds"
        );

        MultilinearPolynomialEV::new(&circuit_evaluation.layer_evaluations[layer_index])
    }

    pub fn add_i_and_mul_i_mle(
        &mut self,
        layer_index: usize,
    ) -> (MultilinearPolynomialEV<F>, MultilinearPolynomialEV<F>) {
        let number_of_layer_variables = num_of_layer_variables(layer_index);
        let boolean_hypercube_combinations = 1 << number_of_layer_variables; // 2 ^ number_of_layer_variables

        let mut add_i_values = vec![F::zero(); boolean_hypercube_combinations];
        let mut mul_i_values = vec![F::zero(); boolean_hypercube_combinations];

        for gate in self.layers[layer_index].gates.iter() {
            match gate.operation {
                Operation::ADD => {
                    let position_index = convert_to_binary_and_to_decimal(
                        layer_index,
                        gate.output,
                        gate.left,
                        gate.right,
                    );
                    add_i_values[position_index] = F::one();
                }
                Operation::MUL => {
                    let position_index = convert_to_binary_and_to_decimal(
                        layer_index,
                        gate.output,
                        gate.left,
                        gate.right,
                    );
                    mul_i_values[position_index] = F::one();
                }
            }
        }

        let add_i_polynomial = MultilinearPolynomialEV::new(&add_i_values);
        let mul_i_polynomial = MultilinearPolynomialEV::new(&mul_i_values);

        (add_i_polynomial, mul_i_polynomial)
    }
}

pub fn num_of_layer_variables(layer_index: usize) -> usize {
    if layer_index == 0 {
        return 3;
    }

    let var_a_length = layer_index;
    let var_b_length = var_a_length + 1;
    let var_c_length = var_a_length + 1;

    var_a_length + var_b_length + var_c_length
}

pub fn convert_to_binary_and_to_decimal(
    layer_index: usize,
    variable_a: usize,
    variable_b: usize,
    variable_c: usize,
) -> usize {
    // convert decimal to binary
    let a_in_binary = convert_decimal_to_padded_binary(variable_a, layer_index);
    let b_in_binary = convert_decimal_to_padded_binary(variable_b, layer_index + 1);
    let c_in_binary = convert_decimal_to_padded_binary(variable_c, layer_index + 1);

    // combine a, b and c binaries
    let combined_binary = a_in_binary + &b_in_binary + &c_in_binary;

    // convert the combined binaries back to decimal
    usize::from_str_radix(&combined_binary, 2).unwrap_or(0)
}

pub fn convert_decimal_to_padded_binary(decimal_number: usize, bit_length: usize) -> String {
    format!("{:0>width$b}", decimal_number, width = bit_length)
}

// unused function: just another method of converting a decimal number to padded binary number
pub fn transform_decimal_to_padded_binary(decimal_number: usize, mut bit_length: usize) -> String {
    if bit_length == 0 {
        bit_length = 1;
    }

    let binary = format!("{:b}", decimal_number);

    "0".repeat(bit_length.saturating_sub(binary.len())) + &binary
}
