use crate::gkr_sumcheck_dependencies::transcript::Transcript;
use crate::prover::Prove;
use crate::utility::utils::field_element_to_bytes;
use ark_ff::PrimeField;
use std::marker::PhantomData;

pub struct Verifier<F: PrimeField> {
    pub transcript: Transcript,
    _phantom: PhantomData<F>,
}
impl<F: PrimeField> Default for Verifier<F> {
    fn default() -> Self {
        Self::new()
    }
}

impl<F: PrimeField> Verifier<F> {
    pub fn new() -> Self {
        Self {
            transcript: Transcript::new(),
            _phantom: PhantomData,
        }
    }

    pub fn verify(&mut self, proof: Prove<F>) -> bool {
        if proof.evaluated_uni_polynomials.len()
            != proof.initial_polynomial.number_of_variables() as usize
        {
            return false;
        }
        let mut current_claimed_sum = proof.initial_claimed_sum;
        self.transcript
            .absorb(&proof.initial_polynomial.convert_to_bytes());
        self.transcript
            .absorb(&field_element_to_bytes(proof.initial_claimed_sum));

        let mut challenges = Vec::with_capacity(proof.evaluated_uni_polynomials.len());
        for i in 0..proof.evaluated_uni_polynomials.len() {
            let evaluation_at_zero = vec![F::zero()];
            let evaluation_at_one = vec![F::one()];

            if proof.evaluated_uni_polynomials[i].evaluate(&evaluation_at_zero)
                + proof.evaluated_uni_polynomials[i].evaluate(&evaluation_at_one)
                != current_claimed_sum
            {
                return false;
            }
            self.transcript
                .absorb(&proof.evaluated_uni_polynomials[i].convert_to_bytes());
            let challenge = self.transcript.random_challenge_as_field_element();
            challenges.push(challenge);
            current_claimed_sum = proof.evaluated_uni_polynomials[i].evaluate(&[challenge]);
        }
        let f_evaluations = proof.initial_polynomial.evaluate(&challenges);
        f_evaluations == current_claimed_sum
    }
}
