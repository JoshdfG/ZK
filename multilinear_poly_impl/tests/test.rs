#[cfg(test)]
    use ark_bn254::Fq;
    use multilinear_poly_impl::{partial_evaluation,MultilinearPolynomial};

#[test]
    fn test_partial_evaluate() {

        let t1 = vec![Fq::from(18), Fq::from(48)];
        assert_eq!(partial_evaluation(&t1, 0, Fq::from(2)), vec![Fq::from(78)]);

        let t2 = vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3), Fq::from(0), Fq::from(0), Fq::from(2), Fq::from(5)];
        assert_eq!(partial_evaluation(&t2, 2, Fq::from(3)), vec![Fq::from(0), Fq::from(9), Fq::from(0), Fq::from(11)]);
    }

    #[test]
    fn test_evaluate() {
        let evaluated_values = vec![Fq::from(0), Fq::from(0), Fq::from(3), Fq::from(8)];
        let polynomial = MultilinearPolynomial::new(evaluated_values);
        let values = vec![Fq::from(6), Fq::from(2)];

        assert_eq!(polynomial.evaluate(values), Fq::from(78));
    }
