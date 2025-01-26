use ark_bn254::Fq;
use std::vec;
use sharmir_secret_sharing::utility::DenseUnivariatePolynomial;

// use super::*;
fn poly_1() -> DenseUnivariatePolynomial<Fq> {
    // f(x) = 1 + 2x + 3x^2
    DenseUnivariatePolynomial {
        coefficients: vec![Fq::from(1), Fq::from(2), Fq::from(3)],
    }
}

fn poly_2() -> DenseUnivariatePolynomial<Fq> {
    // f(x) = 4x + 3 + 5x^11

    DenseUnivariatePolynomial {
        coefficients: [
            vec![Fq::from(3), Fq::from(4)],
            vec![Fq::from(0); 9],
            vec![Fq::from(5)],
        ]
            .concat(),
    }
}

#[test]
fn test_degree() {
    let p = DenseUnivariatePolynomial {
        coefficients: vec![Fq::from(1), Fq::from(2), Fq::from(3)],
    };
    p.degree();

    assert_eq!(p.degree(), 2);
}

#[test]
fn test_eval() {
    // p(x) = 1 + 2x + 3x^2
    // p(3) = 1 + 6 + 27 = 34
    let p = DenseUnivariatePolynomial {
        coefficients: vec![Fq::from(1), Fq::from(2), Fq::from(3)],
    };
    let result = p.evaluate(Fq::from(3));
    assert_eq!(result, Fq::from(34));
}

#[test]
fn test_add_polynomials() {
    // f(x) = 1 + 2x + 3x^2
    // +
    // f(x) = 4x + 3 + 5x^11

    // r(x) = 4 + 6x + 3x^2 + 5x^11

    assert_eq!(
        (&poly_1() + &poly_2()).coefficients,
        [
            vec![Fq::from(4), Fq::from(6), Fq::from(3)],
            vec![Fq::from(0); 8],
            vec![Fq::from(5)],
        ]
            .concat()
    );
}

#[test]
fn test_mul() {
    // f(x) = 5 + 2x^2
    let poly_1 = DenseUnivariatePolynomial {
        coefficients: vec![Fq::from(5), Fq::from(0), Fq::from(2)],
    };
    //  f(x) = 2x + 6
    let poly_2 = DenseUnivariatePolynomial {
        coefficients: vec![Fq::from(6), Fq::from(2)],
    };

    // r(x) = 30 + 10x + 12x^2 + 4x^3
    assert_eq!(
        (poly_1 * poly_2).coefficients,
        vec![Fq::from(30), Fq::from(10), Fq::from(12), Fq::from(4)]
    );
}

#[test]
fn test_interpolate() {
    // f(x) = 2x
    // [(2,4),(4,3)]

    let m = DenseUnivariatePolynomial::interpolate(
        vec![Fq::from(2), Fq::from(4)],
        vec![Fq::from(4), Fq::from(8)],
    );
    assert_eq!(m.coefficients, vec![Fq::from(0), Fq::from(2)]);
}

#[test]
fn test_basis_function() {
    // Test basis polynomial for x = 2 with interpolating set [2, 4]
    let x = Fq::from(2);
    let interpolating_set = vec![Fq::from(2), Fq::from(4)];
    let basis = DenseUnivariatePolynomial::basis(&x, &interpolating_set);

    // L₁(x) should be (-1/2)x + 2
    // In prime field: [-*x_n, P::one()] creates (x - 4)
    // Then we multiply by inverse of (2 - 4) = -2
    println!("Basis for x=2: {:?}", basis.coefficients);
}

#[test]
fn test_scalar_mul() {
    // Test scalar multiplication with a simple polynomial
    let poly = DenseUnivariatePolynomial::new(vec![Fq::from(1), Fq::from(1)]);
    let scalar = Fq::from(2);
    let result = poly.scalar_mul(&scalar);
    println!("Scalar mul result: {:?}", result.coefficients);
}

