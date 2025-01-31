use ark_ff::{BigInteger, PrimeField};

#[derive(Debug, Clone)]
pub struct MultilinearPolynomial<F: PrimeField> {
    pub evaluations: Vec<F>,
}

impl<F: PrimeField> MultilinearPolynomial<F> {
    pub fn new(evaluations: Vec<F>) -> MultilinearPolynomial<F> {
        MultilinearPolynomial { evaluations }
    }

    pub fn evaluate(&self, values: Vec<F>) -> F {
        let mut r_polynomial = self.evaluations.clone();

        let expected_number_of_partial_eval = values.len();

        let _: Vec<_> = (0..expected_number_of_partial_eval)
            .into_iter()
            .map(|index| r_polynomial = self.partial_evaluation(&r_polynomial, 0, values[index]))
            .collect();
        r_polynomial[0]
    }

    pub fn get_number_of_var(&self) -> usize {
        self.evaluations.len().ilog2() as usize
    }

    pub fn partial_evaluation(
        &self,
        polynomial: &Vec<F>,
        evaluating_variable: usize,
        r_value: F,
    ) -> Vec<F> {
        let polynomial_size = polynomial.len() ;
        let divided_size = polynomial_size / 2;
        let poly_result = Vec::with_capacity(divided_size);

        let variables = polynomial.len().ilog2() as usize;
        let power = variables - 1 - evaluating_variable;

        (0..divided_size)
            .fold((0, poly_result), |(j, mut result), _| {
                let y1 = polynomial[j];
                let y2 = polynomial[j | (1 << power)];

                result.push(y1 + (r_value * (y2 - y1)));

                let next_j = if (j + 1) % (1 << power) == 0 {
                    (j + 1) + (1 << power)
                } else {
                    j + 1
                };

                (next_j, result)
            })
            .1
    }

    pub fn convert_to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for value in &self.evaluations {
            bytes.extend(value.into_bigint().to_bytes_be());
        }

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Vec<F> {
        let mut computation = Vec::new();
        let mut i = 0;

        while i < bytes.len() {
            let mut bytes_array = [0u8; 32];
            bytes_array.copy_from_slice(&bytes[i..i + 32]);
            computation.push(F::from_be_bytes_mod_order(&bytes_array));
            i += 32;
        }

        computation
    }
}
