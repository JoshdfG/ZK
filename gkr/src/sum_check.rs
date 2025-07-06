use crate::gkr_sumcheck_dependencies::densed_uni_poly::DensedUnivariatePolynomial;
use crate::gkr_sumcheck_dependencies::poly_sum::SumPolynomial;
use crate::gkr_sumcheck_dependencies::transcript::Transcript;
use ark_ff::{BigInteger, PrimeField};

#[derive(Clone, Debug)]
pub struct SumcheckProverProof<F: PrimeField> {
    pub claimed_sum: F,
    pub round_univariate_polynomials: Vec<DensedUnivariatePolynomial<F>>,
    pub random_challenges: Vec<F>,
}

#[derive(Clone, Debug)]
pub struct SumcheckVerifierProof<F: PrimeField> {
    pub is_proof_valid: bool,
    pub random_challenges: Vec<F>,
    pub last_claimed_sum: F,
}

// The prove function is performing sumcheck on a SumPolynomial which is a combination of two ProductPolynomial that also holds two MultilinearPolynomialEVs each.
// This implies that this particular implementation of Sumcheck prove, is running of 4 MultilinearPolynomialEVs at the same time
pub fn prove<F: PrimeField>(
    sum_polynomial: SumPolynomial<F>,
    claimed_sum: F,
    transcript: &mut Transcript,
) -> SumcheckProverProof<F> {
    let number_of_variables = sum_polynomial.number_of_variables();

    let mut round_univariate_polynomials = Vec::new();
    let mut random_challenges = Vec::with_capacity(number_of_variables as usize);
    let mut current_polynomial = sum_polynomial.clone();

    transcript.absorb(&field_element_to_bytes(claimed_sum));

    for _round in 0..number_of_variables {
        // The generate_round_univariate() generates a round univariate polynomial for each sumcheck round.
        // It does the major computation in this function.
        // It performs partial evaluation simultaneously on 4 polynomials at the same time and at the same variable.
        let univariate = generate_round_univariate(&current_polynomial);

        // Handle interpolation of the univariate values, to get the univariate polynomial
        // The Univariate Polynomial is what we are sending to the verifier,
        // so that the verifier doesn't have to do the work of interpolating before evaluating to get claimed sum
        let x_values: Vec<F> = (0..=sum_polynomial.degree())
            .map(|i| F::from(i as u64))
            .collect();
        let univariate_poly =
            DensedUnivariatePolynomial::lagrange_interpolate(&x_values, &univariate);

        transcript.absorb(&univariate_to_bytes(&univariate_poly.coefficients));
        round_univariate_polynomials.push(univariate_poly);

        let random_challenge: F = transcript.random_challenge_as_field_element();

        current_polynomial = current_polynomial.partial_evaluate(0, random_challenge);

        random_challenges.push(random_challenge);
    }

    SumcheckProverProof {
        claimed_sum,
        round_univariate_polynomials,
        random_challenges,
    }
}

pub fn verify<F: PrimeField>(
    proof: &SumcheckProverProof<F>,
    transcript: &mut Transcript,
) -> SumcheckVerifierProof<F> {
    transcript.absorb(&field_element_to_bytes(proof.claimed_sum));

    let mut current_sum = proof.claimed_sum;
    let mut random_challenges = Vec::with_capacity(proof.round_univariate_polynomials.len());

    for round_polynomial in &proof.round_univariate_polynomials {
        // The verifier only evaluates the univariate polynomial at 0 and 1
        // then checks if it equals the claimed sum, received from the prover
        let eval_at_zero = round_polynomial.evaluate(F::zero());
        let eval_at_one = round_polynomial.evaluate(F::one());

        if eval_at_zero + eval_at_one != current_sum {
            return SumcheckVerifierProof {
                is_proof_valid: false,
                random_challenges: vec![],
                last_claimed_sum: current_sum,
            };
        }

        transcript.absorb(&univariate_to_bytes(&round_polynomial.coefficients));

        let random_challenge = transcript.random_challenge_as_field_element();

        current_sum = round_polynomial.evaluate(random_challenge);

        random_challenges.push(random_challenge);
    }

    SumcheckVerifierProof {
        is_proof_valid: true,
        random_challenges,
        last_claimed_sum: current_sum,
    }
}

/// This function generates the univariate polynomial which is regarded as the proof for each sumcheck round: number of rounds is based on number of polynomial variables
/// The univariate polynomial is generated using (degree + 1) points, since we need (degree + 1) points to represent a UnivariatePolynomial
/// We are using this approach because we want to run Sumcheck simultaneously on the individual polynomials (4 - polynomials) that makes up the SumPolynomial: f(b,c) polynomial
/// This is because we can't combine the polynomials together to be a single polynomial, if we do that, we would get a polynomial with powers greater than one.
/// Hence we won't be able to use the polynomial as a MultilinearPolynomialEV that is evaluated over the boolean hypercube. We might be forced to work with a 3HC structure, which we are avoiding.
pub fn generate_round_univariate<F: PrimeField>(current_polynomial: &SumPolynomial<F>) -> Vec<F> {
    let degree = current_polynomial.degree();
    let num_evaluations = degree + 1;

    // This vector holds each evaluation of the for loop below: The evaluations is the UnivariatePolynomial evaluations points
    let mut evaluations = Vec::with_capacity(num_evaluations);

    // For every iteration, we partially evaluate each of the 4 MultilinearPolynomialEV at the same variable across all 4 polynomials,
    // This iteration will run at (degree + 1) number of times
    // Then we perform an element-wise addition on the SumPolynomial, which calls an element-wise product on the ProductPolynomial
    // The result of the element-wise operations will be a single MultilinearPolynomialEV,
    // Then we sum all the evaluations of the MultilinearPolynomialEV from the element-wise operations, to get a value (FieldElement)
    // That value, will be one the evaluation of the UnivariatePolynomial, that means if (degree + 1) is 3,
    // the loop will run 3 times and there will be 3 evaluation points representing the UnivariatePolynomial
    for i in 0..num_evaluations {
        let value = F::from(i as u64);
        let partial_eval_sum_poly = current_polynomial.partial_evaluate(0, value); // holding Vec<ProductPoly> : length of 2

        // Performing a chain of element-wise operation to combine the 4 MultilinearPolynomialEV to a single MultilinearPolynomialEV
        // Then sum the values to get a single value (Field Element) - represent an evaluation of the UnivariatePolynomial
        let evaluation = partial_eval_sum_poly
            .add_polynomials_element_wise()
            .evaluated_values
            .iter()
            .sum();

        evaluations.push(evaluation)
    }

    evaluations
}

pub fn univariate_to_bytes<F: PrimeField>(univariate_poly: &[F]) -> Vec<u8> {
    univariate_poly
        .iter()
        .flat_map(|x| x.into_bigint().to_bytes_le())
        .collect()
}

pub fn field_element_to_bytes<F: PrimeField>(field_element: F) -> Vec<u8> {
    field_element.into_bigint().to_bytes_be()
}
