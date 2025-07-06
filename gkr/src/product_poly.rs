use crate::evaluation::MultilinearPolynomialEV;
use ark_ff::PrimeField;

// Product Polynomial hold 2 or more multilinear polynomials and performs multiplication operations on them
#[derive(Clone, Debug, PartialEq)]
pub struct ProductPolynomial<F: PrimeField> {
    pub polynomials: Vec<MultilinearPolynomialEV<F>>,
}

impl<F: PrimeField> ProductPolynomial<F> {
    pub fn new(polynomials: Vec<MultilinearPolynomialEV<F>>) -> Self {
        // get the number of variables of the first multilinear polynomial
        // then iterate through all the polynomials and check if they have the same number of variables
        // assert if their number of variables are not the same
        let num_of_variables = polynomials[0].number_of_variables();
        assert!(
            polynomials
                .iter()
                .all(|polynomial| polynomial.number_of_variables() == num_of_variables),
            "different number of variables"
        );

        Self { polynomials }
    }

    pub fn evaluate(&self, values: &[F]) -> F {
        let mut result = F::one();

        for polynomial in self.polynomials.iter() {
            result *= polynomial.evaluate(values);
        }

        result
    }

    pub fn partial_evaluate(
        &self,
        evaluating_variable: usize,
        value: F,
    ) -> Vec<MultilinearPolynomialEV<F>> {
        let mut evaluated_polynomials: Vec<MultilinearPolynomialEV<F>> = Vec::new();

        for polynomial in self.polynomials.iter() {
            let partially_evaluated_polynomial = MultilinearPolynomialEV::partial_evaluate(
                &polynomial.evaluated_values,
                evaluating_variable,
                value,
            );

            evaluated_polynomials.push(partially_evaluated_polynomial);
        }

        evaluated_polynomials
    }

    // This function reduces the Vec of Multilinear polynomials to one Polynomial by
    // basically performing element-wise multiplication on the multilinear polynomials that makes up the ProductPolynomial
    pub fn multiply_polynomials_element_wise(&self) -> MultilinearPolynomialEV<F> {
        assert!(
            self.polynomials.len() > 1,
            "more than one polynomial required for mul operation"
        );

        let mut resultant_values = self.polynomials[0].evaluated_values.to_vec();

        for polynomial in self.polynomials.iter().skip(1) {
            for (i, value) in polynomial.evaluated_values.iter().enumerate() {
                resultant_values[i] *= value
            }
        }

        MultilinearPolynomialEV::new(&resultant_values)
    }

    pub fn convert_to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for polynomial in &self.polynomials {
            bytes.extend_from_slice(&polynomial.convert_to_bytes());
        }

        bytes
    }

    pub fn degree(&self) -> usize {
        self.polynomials.len()
    }
}
