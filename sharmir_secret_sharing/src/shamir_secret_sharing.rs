use std::iter;

use crate::utility::DensedUnivariatePolynomial;

use ark_ff::PrimeField;


// secret: this is the values to be shared from a finite field
// Minimum number of shares required to reconstruct the secret.
// number_shares: Total number of shares to generate.
// this outputs a value of (x,y) pair x is the share identifier / index of the polynomials evaluated at x.
pub fn shares<F: PrimeField>(secret: F, threshold: u64, number_shares: u64) -> Vec<(F, F)> {
    let mut rng = rand::thread_rng();
    let y_values: Vec<F> = std::iter::once(secret)
        .chain(iter::from_fn(|| Some(F::rand(&mut rng))).take(threshold as usize - 1))
        .collect();

    let polynomial = DensedUnivariatePolynomial::new(y_values);

    (1..=number_shares)
        .map(|i| {
            let x = F::from(i);
            (x, polynomial.evaluate(x))
        })
        .collect()
}

pub fn recover_secret<F: PrimeField>(shares: Vec<(F, F)>) -> F {
    let (x_values, y_values) = shares.into_iter().unzip();

    let polynomial = DensedUnivariatePolynomial::lagrange_interpolate(x_values, y_values);
    polynomial.evaluate(F::from(0))
}
