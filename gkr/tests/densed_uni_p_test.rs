use ark_bn254::Fq;
use gkr::gkr_sumcheck_dependencies::densed_uni_poly::{
    add_polynomials, multiply_polynomials, DensedUnivariatePolynomial,
};
fn test_setup() -> DensedUnivariatePolynomial<Fq> {
    let set_of_points = vec![
        Fq::from(0),
        Fq::from(0),
        Fq::from(2),
        Fq::from(0),
        Fq::from(0),
        Fq::from(0),
        Fq::from(0),
        Fq::from(3),
    ];
    DensedUnivariatePolynomial::new(&set_of_points)
}

#[test]
fn test_degree() {
    let polynomial = test_setup();
    assert_eq!(polynomial.degree(), 7);
}

#[test]
fn test_evaluation() {
    let polynomial = test_setup();
    let evaluation_value = Fq::from(2);

    assert_eq!(polynomial.evaluate(evaluation_value), Fq::from(392));
}

#[test]
fn test_evaluation_advanced() {
    let polynomial = test_setup();
    let evaluation_value = Fq::from(2);

    assert_eq!(
        polynomial.evaluate_advanced(evaluation_value),
        Fq::from(392)
    );
}

#[test]
fn test_add_polynomials() {
    let p1 = vec![Fq::from(5), Fq::from(2), Fq::from(5)];
    let p2 = vec![Fq::from(2), Fq::from(1), Fq::from(8), Fq::from(10)];

    assert_eq!(
        add_polynomials(p1, p2),
        vec![Fq::from(7), Fq::from(3), Fq::from(13), Fq::from(10)]
    );
}

#[test]
fn test_multiply_polynomials() {
    let p1 = vec![Fq::from(5), Fq::from(0), Fq::from(2)];
    let p2 = vec![Fq::from(6), Fq::from(2)];

    assert_eq!(
        multiply_polynomials(p1, p2),
        vec![Fq::from(30), Fq::from(10), Fq::from(12), Fq::from(4)]
    );
}

#[test]
fn test_lagrange_interpolate() {
    let x_values = vec![Fq::from(0), Fq::from(1), Fq::from(2)];
    let y_values = vec![Fq::from(2), Fq::from(4), Fq::from(10)];

    assert_eq!(
        DensedUnivariatePolynomial::lagrange_interpolate(&x_values, &y_values).coefficients,
        vec![Fq::from(2), Fq::from(0), Fq::from(2)]
    );
}
