use ark_bn254::{Fr};
use ark_ff::{BigInteger, PrimeField};
use fiat_shamir::transcript::Transcript;
use fiat_shamir::{prover};
use sha3::{Digest, Keccak256};

#[test]
fn test_hash() {
    let mut transcript = Transcript::new(Keccak256::new());

    transcript.absorb(Fr::from(7).into_bigint().to_bytes_be().as_slice());
    transcript.absorb("girl".as_bytes());

    let challenge: Fr = transcript.squeeze();
    let challenge1: Fr = transcript.squeeze();

    dbg!(challenge);
    dbg!(challenge1);
}

#[cfg(test)]
mod tests {
    use crate::prover::{ProofContainer};
    use ark_bn254::{ Fr};
    use fiat_shamir::verifier::Verifier;

    #[test]
    fn test_sumcheck_protocol_prove_and_verify() {
        let polynomial_evaluated_values = vec![
            Fr::from(0),
            Fr::from(0),
            Fr::from(2),
            Fr::from(7),
            Fr::from(3),
            Fr::from(3),
            Fr::from(6),
            Fr::from(11),
        ];


        let mut prover = ProofContainer::new(polynomial_evaluated_values);
        let proof = prover.generate_proofs();

        let mut verifiers: Verifier<Fr> = Verifier::new();

        assert_eq!(verifiers.verify(proof), true);
    }

    #[test]
    fn test_sumcheck_protocol_prove_and_verify2() {
        let polynomial_evaluated_values = vec![
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(1),
            Fr::from(1),
            Fr::from(1),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
        ];

        let mut prover = ProofContainer::new(polynomial_evaluated_values);
        let proof = prover.generate_proofs();

        let mut verifier: Verifier<Fr> = Verifier::new();

        assert_eq!(verifier.verify(proof), true);
    }

    #[test]
    fn test_sumcheck_protocol_prove_and_verify3() {
        let polynomial_evaluated_values = vec![
            Fr::from(1),
            Fr::from(3),
            Fr::from(5),
            Fr::from(7),
            Fr::from(2),
            Fr::from(4),
            Fr::from(6),
            Fr::from(8),
            Fr::from(3),
            Fr::from(5),
            Fr::from(7),
            Fr::from(9),
            Fr::from(4),
            Fr::from(6),
            Fr::from(8),
            Fr::from(10),
        ];

        let mut prover = ProofContainer::new(polynomial_evaluated_values);
        let proof = prover.generate_proofs();

        let mut verifier: Verifier<Fr> = Verifier::new();

        assert_eq!(verifier.verify(proof), true);
    }
}