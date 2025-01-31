use crate::traits::HashTrait;
use ark_ff::PrimeField;
use sha3::{Digest, Keccak256};
// use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Transcript {
    pub hasher: Keccak256,
}

impl Transcript {
    pub fn new(hasher: Keccak256) -> Self {
        Self { hasher }
    }
    pub fn absorb(&mut self, data: &[u8]) {
        self.hasher.append(data);
    }
    pub fn squeeze<F: PrimeField>(&self) -> F {
        let hash_output = self.hasher.generate_hash();
        F::from_be_bytes_mod_order(&hash_output)
    }

    fn sample_random_challenge(&mut self) -> [u8; 32] {
        let mut output_hash = [0; 32]; // fixed sized array of 32-bytes initially filled with zeros
        output_hash.copy_from_slice(&self.hasher.finalize_reset());
        self.hasher.update(output_hash);

        output_hash
    }

    pub(crate) fn random_challenge_as_field_element<F: PrimeField>(&mut self) -> F {
        let random_challenge = self.sample_random_challenge();

        // convert bytes into field element using: from_bytes_mod_order()
        F::from_le_bytes_mod_order(&random_challenge)
    }
}

impl HashTrait for Keccak256 {
    fn append(&mut self, data: &[u8]) {
        self.update(data);
    }
    fn generate_hash(&self) -> Vec<u8> {
        self.clone().finalize().to_vec()
    }
}
