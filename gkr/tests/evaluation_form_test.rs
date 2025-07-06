use ark_bn254::Fq;
use gkr::evaluation::MultilinearPolynomialEV;

#[test]
fn test_partial_evaluate() {
    let polynomial = vec![Fq::from(0), Fq::from(0), Fq::from(3), Fq::from(8)];

    assert_eq!(
        MultilinearPolynomialEV::partial_evaluate(&polynomial, 0, Fq::from(6)),
        MultilinearPolynomialEV::new(&[Fq::from(18), Fq::from(48)])
    );
    assert_eq!(
        MultilinearPolynomialEV::partial_evaluate(&polynomial, 1, Fq::from(2)),
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(13)])
    );

    let small_polynomial = vec![Fq::from(18), Fq::from(48)];
    assert_eq!(
        MultilinearPolynomialEV::partial_evaluate(&small_polynomial, 0, Fq::from(2)),
        MultilinearPolynomialEV::new(&[Fq::from(78)])
    );

    let bigger_polynomial = vec![
        Fq::from(0),
        Fq::from(0),
        Fq::from(0),
        Fq::from(3),
        Fq::from(0),
        Fq::from(0),
        Fq::from(2),
        Fq::from(5),
    ];
    assert_eq!(
        MultilinearPolynomialEV::partial_evaluate(&bigger_polynomial, 2, Fq::from(3)),
        MultilinearPolynomialEV::new(&[Fq::from(0), Fq::from(9), Fq::from(0), Fq::from(11)])
    );
}

#[test]
fn test_evaluate() {
    let evaluated_values = vec![Fq::from(0), Fq::from(0), Fq::from(3), Fq::from(8)];
    let polynomial = MultilinearPolynomialEV::new(&evaluated_values);
    let values = vec![Fq::from(6), Fq::from(2)];

    assert_eq!(polynomial.evaluate(&values), Fq::from(78));
}

#[test]
fn test_polynomial_tensor_add() {
    // w(b) = [1,2] (one variable)
    let wb = MultilinearPolynomialEV::new(&[Fq::from(1), Fq::from(2)]);
    // w(c) = [3,4] (one variable)
    let wc = MultilinearPolynomialEV::new(&[Fq::from(3), Fq::from(4)]);

    let result = MultilinearPolynomialEV::polynomial_tensor_add(&wb, &wc);

    // Result should be [4,5,5,6] representing w(b,c) at points (0,0),(0,1),(1,0),(1,1)
    let expected = MultilinearPolynomialEV::new(&[
        Fq::from(4), // 1+3
        Fq::from(5), // 1+4
        Fq::from(5), // 2+3
        Fq::from(6), // 2+4
    ]);

    assert_eq!(result, expected);
}

#[test]
fn test_polynomial_tensor_mul() {
    // w(b) = [2,3]
    let w_b = MultilinearPolynomialEV::new(&[Fq::from(2), Fq::from(3)]);

    // w(c) = [4,5]
    let w_c = MultilinearPolynomialEV::new(&[Fq::from(4), Fq::from(5)]);

    // Get result of tensor multiplication
    let result = MultilinearPolynomialEV::polynomial_tensor_mul(&w_b, &w_c);

    // Expected: [8,10,12,15]
    // Because:
    // 2*4 = 8  (w_b[0] * w_c[0])
    // 2*5 = 10 (w_b[0] * w_c[1])
    // 3*4 = 12 (w_b[1] * w_c[0])
    // 3*5 = 15 (w_b[1] * w_c[1])
    let expected =
        MultilinearPolynomialEV::new(&[Fq::from(8), Fq::from(10), Fq::from(12), Fq::from(15)]);

    assert_eq!(result, expected);
}

#[test]
#[should_panic(expected = "different polynomial length")]
fn test_polynomial_tensor_mul_different_lengths() {
    let w_b = MultilinearPolynomialEV::new(&[Fq::from(2), Fq::from(3)]);
    let w_c = MultilinearPolynomialEV::new(&[Fq::from(4)]); // Different length

    // Should panic due to different lengths
    MultilinearPolynomialEV::polynomial_tensor_mul(&w_b, &w_c);
}
