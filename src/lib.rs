use noir_rs::{
    native_types::{Witness, WitnessMap},
    prove, verify, FieldElement,
};

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type Proof;
        fn prove_swift(
            circuit_bytecode: String,
            initial_witness_vec_raw: Vec<i32>,
        ) -> Option<Proof>;
        fn verify_swift(circuit_bytecode: String, proof: Proof) -> Option<bool>;
    }
}

pub struct Proof {
    pub proof: Vec<u8>,
    pub verification_key: Vec<u8>,
}

/// Generates a zkSNARK proof from given bytecode and witness values.
///
/// This function converts the initial witness vector from i32 values to FieldElement values, then creates a WitnessMap
/// from these values. Using the provided circuit bytecode and this WitnessMap, it generates the zkSNARK proof.
///
/// # Parameters
/// - `circuit_bytecode`: The bytecode of the ACIR circuit for which a proof needs to be generated.
/// - `initial_witness_vec_raw`: The initial witness values represented as a vector of i32.
///
/// # Returns
/// - `Option<Proof>`: The generated proof and its associated verification key wrapped in a `Proof` struct. Returns `None` if the proof generation fails.
pub fn prove_swift(circuit_bytecode: String, initial_witness_vec_raw: Vec<i32>) -> Option<Proof> {
    let initial_witness_vec: Vec<FieldElement> = initial_witness_vec_raw
        .into_iter()
        .map(|f| f as i128)
        .map(FieldElement::from)
        .collect();
    let mut initial_witness = WitnessMap::new();
    for (i, witness) in initial_witness_vec.into_iter().enumerate() {
        initial_witness.insert(Witness(i as u32 + 1), witness);
    }

    let (proof, verification_key) = prove(circuit_bytecode, initial_witness).ok()?;
    Some(Proof {
        proof,
        verification_key,
    })
}

/// Verifies a given zkSNARK proof using the associated circuit bytecode and verification key.
///
/// This function takes in the circuit bytecode, the generated proof, and its verification key, then verifies
/// the correctness of the proof.
///
/// # Parameters
/// - `circuit_bytecode`: The bytecode of the ACIR circuit against which the proof needs to be verified.
/// - `proof`: The `Proof` struct containing the zkSNARK proof and its verification key.
///
/// # Returns
/// - `Option<bool>`: Returns `true` if the proof is valid, `false` otherwise. Returns `None` if the verification process fails.
pub fn verify_swift(circuit_bytecode: String, proof: Proof) -> Option<bool> {
    verify(circuit_bytecode, proof.proof, proof.verification_key).ok()
}

#[cfg(test)]
mod tests {
    use crate::{prove_swift, verify_swift};

    const BYTECODE: &str = "H4sIAAAAAAAA/7WTMRLEIAhFMYkp9ywgGrHbq6yz5v5H2JkdCyaxC9LgWDw+H9gBwMM91p7fPeOzIKdYjEeMLYdGTB8MpUrCmOohJJQkfYMwN4mSSy0ZC0VudKbCZ4cthqzVrsc/yw28dMZeWmrWerfBexnsxD6hJ7jUufr4GvyZFp8xpG0C14Pd8s/q29vPCBXypvmpDx7sD8opnfqIfsM1RNtxBQAA";

    #[test]
    fn test_prove_verify() {
        let proof = prove_swift(String::from(BYTECODE), vec![1_i32, 10_i32]).unwrap();
        let verdict = verify_swift(String::from(BYTECODE), proof).unwrap();
        assert!(verdict);
    }
}
