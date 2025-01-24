use ark_ff::PrimeField;

#[derive(Debug)]
pub struct DensedUnivariatePolynomial<F: PrimeField> {
    pub coefficients: Vec<F>,
}

impl<F: PrimeField> DensedUnivariatePolynomial<F> {
    pub fn new(coeffs: Vec<F>) -> Self {
        Self {
            coefficients: coeffs,
        }
    }

    pub fn degree(&self) -> u32 {
        self.coefficients.len() as u32 - 1
    }

    pub fn evaluate(&self, value: F) -> F {
        self.coefficients
            .iter()
            .enumerate()
            .map(|(index, coeff)| *coeff * value.pow([index as u64]))
            .sum()
    }

    pub fn lagrange_interpolate(x_values: Vec<F>, y_values: Vec<F>) -> Self {
        let coeffs = x_values
            .iter()
            .enumerate()
            .map(|(index, &x_value)| lagrange_basis(y_values[index], x_value, x_values.clone()))
            .fold(vec![F::zero()], add_polynomials);

        DensedUnivariatePolynomial::new(coeffs)
    }
}

pub fn lagrange_basis<F: PrimeField>(
    y_point: F,
    focus_x_point: F,
    interpolating_set: Vec<F>,
) -> Vec<F> {
    let numerator = interpolating_set
        .iter()
        .filter(|&x| *x != focus_x_point)
        .fold(vec![F::one()], |acc, &x| {
            multiply_polynomials(acc, vec![-x, F::one()])
        });

    let univariate_poly = DensedUnivariatePolynomial::new(numerator.clone());
    let denominator = univariate_poly.evaluate(focus_x_point);

    scalar_mul(y_point / denominator, numerator)
}

pub fn scalar_mul<F: PrimeField>(scalar: F, polynomial: Vec<F>) -> Vec<F> {
    polynomial.into_iter().map(|coeff| scalar * coeff).collect()
}

pub fn multiply_polynomials<F: PrimeField>(left: Vec<F>, right: Vec<F>) -> Vec<F> {
    let mut polynomial_product = vec![F::zero(); left.len() + right.len() - 1];

    left.iter()
        .enumerate()
        .flat_map(|(left_index, &left_coeff)| {
            right
                .iter()
                .enumerate()
                .map(move |(right_index, &right_coeff)| {
                    (left_index + right_index, left_coeff * right_coeff)
                })
        })
        .for_each(|(index, val)| polynomial_product[index] += val);

    polynomial_product
}

pub fn add_polynomials<F: PrimeField>(left: Vec<F>, right: Vec<F>) -> Vec<F> {
    let (larger, smaller) = if left.len() > right.len() {
        (left, right)
    } else {
        (right, left)
    };

    larger
        .into_iter()
        .enumerate()
        .map(|(exp, coeff)| {
            if exp < smaller.len() {
                coeff + smaller[exp]
            } else {
                coeff
            }
        })
        .collect()
}
