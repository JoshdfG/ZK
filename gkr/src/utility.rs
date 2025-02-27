pub mod utils {
    use ark_ff::{BigInteger, PrimeField};

    pub fn split_polynomial_and_sum_each<F: PrimeField>(
        polynomial_evaluated_values: &Vec<F>,
    ) -> Vec<F> {
        let mut univariate_polynomial: Vec<F> = Vec::with_capacity(2);

        let mid = polynomial_evaluated_values.len() / 2;
        let (left, right) = polynomial_evaluated_values.split_at(mid);

        let left_sum: F = left.iter().sum();
        let right_sum: F = right.iter().sum();

        univariate_polynomial.push(left_sum);
        univariate_polynomial.push(right_sum);

        univariate_polynomial
    }

    pub fn field_element_to_bytes<F: PrimeField>(field_element: F) -> Vec<u8> {
        field_element.into_bigint().to_bytes_be()
    }
}
