// use super::*;
// use ark_bn254::Fq;
// use polynomials::composed::product_polynomial::ProductPolynomial;
// use polynomials::multilinear::evaluation_form::MultilinearPolynomial;
//
// #[test]
// fn test_generate_round_univariate() {
//     let poly1a =
//         MultilinearPolynomial::new(&vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
//     let poly2a =
//         MultilinearPolynomial::new(&vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]);
//     let product_poly1 = ProductPolynomial::new(vec![poly1a, poly2a]);
//
//     let poly1b =
//         MultilinearPolynomial::new(&vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
//     let poly2b =
//         MultilinearPolynomial::new(&vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]);
//     let product_poly2 = ProductPolynomial::new(vec![poly1b, poly2b]);
//
//     let sum_polynomial = SumPolynomial::new(vec![product_poly1, product_poly2]);
//
//     let univariate_poly = generate_round_univariate(&sum_polynomial);
//
//     println!("Round Poly: {:?}", univariate_poly);
//     assert_eq!(
//         univariate_poly,
//         vec![Fq::from(0), Fq::from(12), Fq::from(48)]
//     );
// }
//
// #[test]
// fn test_prover_and_verifier() {
//     let poly1a =
//         MultilinearPolynomial::new(&vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
//     let poly2a =
//         MultilinearPolynomial::new(&vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]);
//     let product_poly1 = ProductPolynomial::new(vec![poly1a, poly2a]);
//
//     let poly1b =
//         MultilinearPolynomial::new(&vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(2)]);
//     let poly2b =
//         MultilinearPolynomial::new(&vec![Fq::from(0), Fq::from(0), Fq::from(0), Fq::from(3)]);
//     let product_poly2 = ProductPolynomial::new(vec![poly1b, poly2b]);
//
//     let sum_polynomial = SumPolynomial::new(vec![product_poly1, product_poly2]);
//
//     let mut prover_transcript = Transcript::new();
//     let mut verifier_transcript = Transcript::new();
//
//     let result = prove(sum_polynomial, Fq::from(12), &mut prover_transcript);
//
//     let verified = verify(&result, &mut verifier_transcript);
//
//     assert_eq!(verified.is_proof_valid, true);
// }
