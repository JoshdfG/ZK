use ark_ff::{BigInteger, PrimeField};

// This implementation of Multi linear interpolation uses an evaluation over the boolean hypercube
// then the values from the boolean hypercube evaluation is used as the polynomial
// which will be evaluated at a given variable values using partial evaluation
#[derive(Debug, Clone, PartialEq)]
pub struct MultilinearPolynomialEV<F: PrimeField> {
    pub evaluated_values: Vec<F>,
}

impl<F: PrimeField> MultilinearPolynomialEV<F> {
    pub fn new(evaluated_values: &[F]) -> Self {
        Self {
            evaluated_values: evaluated_values.to_vec(),
        }
    }

    // The evaluate function calls the partial evaluate multiple times
    pub fn evaluate(&self, values: &[F]) -> F {
        let mut r_polynomial = self.clone();
        let expected_number_of_partial_eval = values.len();

        let mut i = 0;

        while i < expected_number_of_partial_eval {
            r_polynomial = Self::partial_evaluate(&r_polynomial.evaluated_values, 0, values[i]);
            i += 1;
        }

        r_polynomial.evaluated_values[0]
    }

    pub fn convert_to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for value in &self.evaluated_values {
            bytes.extend(value.into_bigint().to_bytes_be());
        }

        bytes
    }

    pub fn number_of_variables(&self) -> u32 {
        self.evaluated_values.len().ilog2()
    }

    pub fn scalar_mul(&self, scalar: F) -> Self {
        let scaled_values: Vec<F> = self
            .evaluated_values
            .iter()
            .map(|value| *value * scalar)
            .collect();

        MultilinearPolynomialEV::new(&scaled_values)
    }

    // This function will receive a polynomial in it's evaluated form
    // That means the polynomial it will receive has already been evaluated over a boolean hypercube
    pub fn partial_evaluate(polynomial: &[F], evaluating_variable: usize, value: F) -> Self {
        let polynomial_size = polynomial.len();
        let expected_polynomial_size = polynomial_size / 2;
        let mut result_polynomial: Vec<F> = Vec::with_capacity(expected_polynomial_size);

        let mut i = 0;
        let mut j = 0;

        while i < expected_polynomial_size {
            let first_pair_value = polynomial[j]; // y1

            // since the number of boolean hypercube evaluations for a polynomial with n number of variable is 2^n
            // The number of variables, when given the evaluations: n = log2(polynomial length)
            let number_of_variables = polynomial.len().ilog2() as usize;

            // 0 1 2 => evaluating variable for a = 0, b = 1, c = 2
            // | | |
            // a b c
            // using evaluating_variable as variable index in boolean hypercube
            let power = number_of_variables - 1 - evaluating_variable;

            let second_pair_value = polynomial[j | (1 << power)]; // y2

            // using the formula: y1 + r(y2 - y1)
            // y1 => first_pair_value
            // y2 => second_pair_value
            // r => value
            result_polynomial
                .push(first_pair_value + (value * (second_pair_value - first_pair_value)));

            i += 1;

            // After pairing, we need to determine what our next y1 value, which will be used for pairing to get a y2
            // To get the next y1, we first add 1 to the previous y1 and check if the modulo with the 2^power of the variable we are evaluating at is zero
            // ie: (previous_y1 + 1) % 2^power
            // If it is zero we jump by (previous_y1 + 1 + 2^power)
            // If it is not zero, we jump by adding 1: (previous_y1 + 1)
            j = if (j + 1) % (1 << power) == 0 {
                j + 1 + (1 << power)
            } else {
                j + 1
            }
        }

        MultilinearPolynomialEV::new(&result_polynomial)
    }

    pub fn polynomial_tensor_add(
        w_b: &MultilinearPolynomialEV<F>,
        w_c: &MultilinearPolynomialEV<F>,
    ) -> MultilinearPolynomialEV<F> {
        assert!(w_b.evaluated_values.len() == w_c.evaluated_values.len());

        let mut add_result = Vec::new();

        for b in &w_b.evaluated_values {
            for c in &w_c.evaluated_values {
                add_result.push(*b + *c);
            }
        }

        MultilinearPolynomialEV::new(&add_result)
    }

    pub fn polynomial_tensor_mul(
        w_b: &MultilinearPolynomialEV<F>,
        w_c: &MultilinearPolynomialEV<F>,
    ) -> MultilinearPolynomialEV<F> {
        assert!(
            w_b.evaluated_values.len() == w_c.evaluated_values.len(),
            "different polynomial length"
        );

        let mut mul_result = Vec::new();

        for b in &w_b.evaluated_values {
            for c in &w_c.evaluated_values {
                mul_result.push(*b * *c);
            }
        }

        MultilinearPolynomialEV::new(&mul_result)
    }

    pub fn add_polynomials(
        poly1: &MultilinearPolynomialEV<F>,
        poly2: &MultilinearPolynomialEV<F>,
    ) -> Self {
        assert_eq!(
            poly1.evaluated_values.len(),
            poly2.evaluated_values.len(),
            "Polynomials must have same number of evaluations for addition"
        );

        let sum_values: Vec<F> = poly1
            .evaluated_values
            .iter()
            .zip(poly2.evaluated_values.iter())
            .map(|(a, b)| *a + *b)
            .collect();

        MultilinearPolynomialEV::new(&sum_values)
    }
}
