use ark_ff::PrimeField;

pub struct MultilinearPolynomial<F: PrimeField> {
    pub evaluations: Vec<F>,
}

impl<F: PrimeField> MultilinearPolynomial<F> {
    pub fn new(evaluations: Vec<F>) -> Self {
        MultilinearPolynomial { evaluations }
    }

    pub fn evaluate(&self, values: Vec<F>) -> F {
        let mut r_polynomial = self.evaluations.clone();

        let expected_number_of_partial_eval = values.len();

        let _ : Vec<_> = (0..expected_number_of_partial_eval).into_iter().map(|index| {
            r_polynomial = partial_evaluation(&r_polynomial, 0, values[index])
        }).collect();
        r_polynomial[0]
    }
}
pub fn partial_evaluation<F: PrimeField>(polynomial: &Vec<F>, evaluating_variable: usize, r_value: F) -> Vec<F> {
    let polynomial_size = polynomial.len();
    let divided_size = polynomial_size / 2;
    let poly_result = Vec::with_capacity(divided_size);

    let variables = polynomial_size.ilog2() as usize;
    let power = variables - 1 - evaluating_variable;

    (0..divided_size).fold((0, poly_result), |(j, mut result), _| {
        let y1 = polynomial[j];
        let y2 = polynomial[j | (1 << power)];

        result.push(y1 + (r_value * (y2 - y1)));

        let next_j = if (j + 1) % (1 << power) == 0 {
            (j + 1) + (1 << power)
        } else {
            j + 1
        };

        (next_j, result)
    }).1
}
