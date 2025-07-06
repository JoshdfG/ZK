use ark_bn254::Fq;
use gkr::evaluation::MultilinearPolynomialEV;
use gkr::product_poly::ProductPolynomial;

#[test]
#[should_panic(expected = "different number of variables")]
fn test_new_with_different_polynomial_lengths() {
    let poly1 = MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(2)]); // 1 variable
    let poly2 = MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]); // 2 variables

    // This is expected to panic
    ProductPolynomial::new(vec![poly1, poly2]);
}

#[test]
fn test_evaluate_product_poly() {
    let polynomail1 =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
    let polynomail2 =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]);

    let polynomials = vec![polynomail1, polynomail2];

    let product_polynomial = ProductPolynomial::new(polynomials);
    // a = 1, b = 2
    let values = vec![Fq::from(1), Fq::from(2)];

    assert_eq!(product_polynomial.evaluate(&values), Fq::from(24));
}

#[test]
fn test_partial_evaluate_product_poly() {
    let polynomail1 =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
    let polynomail2 =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]);

    let polynomials = vec![polynomail1, polynomail2];
    let product_polynomial = ProductPolynomial::new(polynomials);

    let expect_poly1 = MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(4)]);
    let expect_poly2 = MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(6)]);
    let expected_partial_eval_result = vec![expect_poly1, expect_poly2];

    assert_eq!(
        product_polynomial.partial_evaluate(0, Fq::from(2)),
        expected_partial_eval_result
    );
}

#[test]
fn test_multiply_polynomials_element_wise() {
    let polynomail1 =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
    let polynomail2 =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]);

    let polynomials = vec![polynomail1, polynomail2];

    let product_polynomial = ProductPolynomial::new(polynomials);
    let expected_product =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(6)]);

    assert_eq!(
        product_polynomial.multiply_polynomials_element_wise(),
        expected_product
    );
}

#[test]
fn test_product_poly_degree() {
    let polynomail1 =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
    let polynomail2 =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]);

    let polynomials = vec![polynomail1, polynomail2];

    let product_polynomial = ProductPolynomial::new(polynomials);

    assert_eq!(product_polynomial.degree(), 2);
}
