use std::iter;

use crate::utility::DenseUnivariatePolynomial;

use ark_ff::PrimeField;

// secret: this is the values to be shared from a finite field
// Minimum number of shares required to reconstruct the secret.
// number_shares: Total number of shares to generate.
// this outputs a value of (x,y) pair x is the share identifier / index of the polynomials evaluated at x.
pub fn shares<F: PrimeField>(secret: F, threshold: u64, number_shares: u64) -> Vec<(F, F)> {
let mut rng = rand::thread_rng();
 let y_values = iter::once(secret).chain(iter::from_fn(|| Some(F::rand(&mut rng)))).take(threshold as usize -1).collect();
    let polynomial = DenseUnivariatePolynomial::new(y_values);
    // y is the result of evaluating the polynomial at x-coordinates
    (1..=number_shares)
        .map(|i| {
            let x_coordinates = F::from(i);
            (x_coordinates, polynomial.evaluate(x_coordinates))
        })
        .collect()
}

pub fn recover_secret<F: PrimeField>(shares: Vec<(F, F)>) -> F {
    let (x_values, y_values) = shares.into_iter().unzip();

    let polynomial = DenseUnivariatePolynomial::interpolate(x_values, y_values);
    polynomial.evaluate(F::zero())
}
