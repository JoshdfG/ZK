use ark_bn254::Fq;
use gkr::evaluation::MultilinearPolynomialEV;
use gkr::gkr_sumcheck_dependencies::poly_sum::SumPolynomial;
use gkr::product_poly::ProductPolynomial;

#[test]
#[should_panic(expected = "different number of variables")]
fn test_new_with_different_polynomial_lengths() {
    let poly1 = MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(2)]); // 1 variable
    let poly2 = MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]); // 2 variables

    let product_poly1 = ProductPolynomial::new(vec![poly1]);
    let product_poly2 = ProductPolynomial::new(vec![poly2]);

    // This is expected to panic
    SumPolynomial::new(vec![product_poly1, product_poly2]);
}

#[test]
fn test_evaluate_sum_poly() {
    // First product polynomial
    let poly1a =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
    let poly1b =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]);
    let product_poly1 = ProductPolynomial::new(vec![poly1a, poly1b]);

    // Second product polynomial
    let poly2a =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(1)]);
    let poly2b =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
    let product_poly2 = ProductPolynomial::new(vec![poly2a, poly2b]);

    let sum_polynomial = SumPolynomial::new(vec![product_poly1, product_poly2]);

    // a = 1, b = 2
    let values = vec![Fq::from(1), Fq::from(2)];

    assert_eq!(sum_polynomial.evaluate(&values), Fq::from(32));
}

#[test]
fn test_partial_evaluate_sum_poly() {
    // First product polynomial
    let poly1a =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
    let poly1b =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]);
    let product_poly1 = ProductPolynomial::new(vec![poly1a, poly1b]);

    // Second product polynomial
    let poly2a =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(1)]);
    let poly2b =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
    let product_poly2 = ProductPolynomial::new(vec![poly2a, poly2b]);

    let sum_polynomial = SumPolynomial::new(vec![product_poly1, product_poly2]);
    let evaluated_sum_poly = sum_polynomial.partial_evaluate(0, Fq::from(2));

    // Expected partial evaluations:
    let expect_poly1a = MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(4)]);
    let expect_poly1b = MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(6)]);
    let expect_product1 = ProductPolynomial::new(vec![expect_poly1a, expect_poly1b]);

    // For second product: poly2a(2) * poly2b = [0,2] * [0,2]
    let expect_poly2a = MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(2)]);
    let expect_poly2b = MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(4)]);
    let expect_product2 = ProductPolynomial::new(vec![expect_poly2a, expect_poly2b]);

    let expected_sum_poly = SumPolynomial::new(vec![expect_product1, expect_product2]);

    assert_eq!(
        evaluated_sum_poly.product_polynomials[0].polynomials,
        expected_sum_poly.product_polynomials[0].polynomials
    );
    assert_eq!(
        evaluated_sum_poly.product_polynomials[1].polynomials,
        expected_sum_poly.product_polynomials[1].polynomials
    );
}

#[test]
fn test_add_polynomials_element_wise() {
    // First product polynomial: (2x)(3y)
    let poly1a =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
    let poly1b =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]);
    let product_poly1 = ProductPolynomial::new(vec![poly1a, poly1b]);

    // Second product polynomial: (1x)(2y)
    let poly2a =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(1)]);
    let poly2b =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
    let product_poly2 = ProductPolynomial::new(vec![poly2a, poly2b]);

    let sum_polynomial = SumPolynomial::new(vec![product_poly1, product_poly2]);

    let expected_sum = MultilinearPolynomialEV::new(&[
        Fq::from(0),
        Fq::from(0),
        Fq::from(0),
        Fq::from(8), // (2*3) + (1*2) = 6 + 2 = 8
    ]);

    assert_eq!(sum_polynomial.add_polynomials_element_wise(), expected_sum);
}

#[test]
fn test_degree_sum_poly() {
    let poly1a =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
    let poly1b =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]);
    let product_poly1 = ProductPolynomial::new(vec![poly1a, poly1b]);

    let poly2a =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(1)]);
    let poly2b =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
    let product_poly2 = ProductPolynomial::new(vec![poly2a, poly2b]);

    let sum_polynomial = SumPolynomial::new(vec![product_poly1, product_poly2]);

    assert_eq!(sum_polynomial.degree(), 2);
}

#[test]
fn test_number_of_variables() {
    let poly1a =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
    let poly1b =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]);
    let product_poly1 = ProductPolynomial::new(vec![poly1a, poly1b]);

    let poly2a =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(1)]);
    let poly2b =
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
    let product_poly2 = ProductPolynomial::new(vec![poly2a, poly2b]);

    let sum_polynomial = SumPolynomial::new(vec![product_poly1, product_poly2]);

    assert_eq!(sum_polynomial.number_of_variables(), 2);
}
