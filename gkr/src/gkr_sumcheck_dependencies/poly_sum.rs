use crate::evaluation::MultilinearPolynomialEV;
use crate::product_poly::ProductPolynomial;
use ark_ff::PrimeField;

// Sum Polynomial hold 2 or more multilinear polynomials and performs addition operations on them
#[derive(Clone, Debug, PartialEq)]
pub struct SumPolynomial<F: PrimeField> {
    pub product_polynomials: Vec<ProductPolynomial<F>>,
}

impl<F: PrimeField> SumPolynomial<F> {
    pub fn new(product_polynomials: Vec<ProductPolynomial<F>>) -> Self {
        // We expect that all the multilinear polynomial will have the number same variables
        let first_polynomial = &product_polynomials[0].polynomials[0];
        let num_of_variables = first_polynomial.number_of_variables();

        assert!(
            product_polynomials.iter().all(|product_poly| product_poly
                .polynomials
                .iter()
                .all(|polynomial| polynomial.number_of_variables() == num_of_variables)),
            "different number of variables"
        );

        Self {
            product_polynomials,
        }
    }

    pub fn evaluate(&self, values: &[F]) -> F {
        let mut result = F::zero();

        for product_polynomial in self.product_polynomials.iter() {
            result += product_polynomial.evaluate(values);
        }

        result
    }

    pub fn partial_evaluate(&self, evaluating_variable: usize, value: F) -> Self {
        let mut evaluated_polynomials = Vec::new();

        for product_polynomial in self.product_polynomials.iter() {
            let evaluated_product_poly =
                product_polynomial.partial_evaluate(evaluating_variable, value);

            evaluated_polynomials.push(ProductPolynomial::new(evaluated_product_poly));
        }

        Self {
            product_polynomials: evaluated_polynomials,
        }
    }

    // This function reduces the Vec of Product polynomials
    // to one Polynomial by basically performing element-wise addition
    pub fn add_polynomials_element_wise(&self) -> MultilinearPolynomialEV<F> {
        assert!(
            self.product_polynomials.len() > 1,
            "more than one product polynomial required for add operation"
        );

        let first_product = self.product_polynomials[0].multiply_polynomials_element_wise();

        let mut resultant_values = first_product.evaluated_values.to_vec();

        for product_polynomial in self.product_polynomials.iter().skip(1) {
            let multiplied_poly = product_polynomial.multiply_polynomials_element_wise();

            for (i, value) in multiplied_poly.evaluated_values.iter().enumerate() {
                resultant_values[i] += value
            }
        }

        MultilinearPolynomialEV::new(&resultant_values)
    }

    pub fn convert_to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for product_polynomial in &self.product_polynomials {
            bytes.extend_from_slice(&product_polynomial.convert_to_bytes());
        }

        bytes
    }

    pub fn degree(&self) -> usize {
        self.product_polynomials[0].degree()
    }

    pub fn number_of_variables(&self) -> u32 {
        self.product_polynomials[0].polynomials[0].number_of_variables()
    }
}
