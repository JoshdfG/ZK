use crate::evaluation::MultilinearPolynomialEV;
use crate::{
    gkr_sumcheck_dependencies::transcript::Transcript, utility::utils::field_element_to_bytes,
    utility::utils::split_polynomial_and_sum_each,
};
use ark_ff::PrimeField;
use sha3::{Digest, Keccak256};

#[derive(Debug, Clone)]
pub struct ProofContainer<F: PrimeField> {
    pub claimed_sum: F,
    pub evaluated_uni_polynomials: Vec<MultilinearPolynomialEV<F>>,
    pub initial_polynomial: MultilinearPolynomialEV<F>,
    pub transcript: Transcript,
}

pub struct Prove<F: PrimeField> {
    pub initial_polynomial: MultilinearPolynomialEV<F>,
    pub initial_claimed_sum: F,
    pub evaluated_uni_polynomials: Vec<MultilinearPolynomialEV<F>>,
}

impl<F: PrimeField> ProofContainer<F> {
    pub fn new(_boolean_hypercube_evaluations: Vec<F>) -> Self {
        let polynomial = MultilinearPolynomialEV::new(&_boolean_hypercube_evaluations.clone());
        let transcript = Transcript {
            hasher: Keccak256::new(),
        };

        Self {
            claimed_sum: _boolean_hypercube_evaluations.iter().sum(),
            evaluated_uni_polynomials: Vec::new(),
            initial_polynomial: polynomial,
            transcript,
        }
    }

    pub fn generate_proofs(&mut self) -> Prove<F> {
        let mut random_challenges: Vec<F> = vec![];
        self.transcript
            .absorb(&self.initial_polynomial.convert_to_bytes());
        self.transcript
            .absorb(&field_element_to_bytes(self.claimed_sum));

        let mut current_polynomial = self.initial_polynomial.evaluated_values.clone();

        for _i in 0..self.initial_polynomial.number_of_variables() {
            let univariate_lr_values = split_polynomial_and_sum_each(&current_polynomial);
            let univariate_polynomials = MultilinearPolynomialEV::new(&univariate_lr_values);
            let univariate_to_bytes = univariate_polynomials.convert_to_bytes();
            self.evaluated_uni_polynomials.push(univariate_polynomials);
            self.transcript.absorb(&univariate_to_bytes);

            let random_challenge = self.transcript.random_challenge_as_field_element();
            random_challenges.push(random_challenge);

            current_polynomial =
                MultilinearPolynomialEV::partial_evaluate(&current_polynomial, 0, random_challenge)
                    .evaluated_values;
        }
        Prove {
            initial_polynomial: self.initial_polynomial.clone(),
            initial_claimed_sum: self.claimed_sum,
            evaluated_uni_polynomials: self.evaluated_uni_polynomials.clone(),
        }
    }
}
