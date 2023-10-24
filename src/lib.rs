use noir_rs::{
    native_types::{Witness, WitnessMap},
    prove, verify, FieldElement,
};

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type Proof;
        fn prove_swift(circuit_bytecode: String, initial_witness_vec_raw: Vec<i32>) -> Proof;
        fn verify_swift(circuit_bytecode: String, proof: Proof) -> bool;
    }
}

pub struct Proof {
    pub proof: Vec<u8>,
    pub verification_key: Vec<u8>,
}

pub fn prove_swift(circuit_bytecode: String, initial_witness_vec_raw: Vec<i32>) -> Proof {
    let initial_witness_vec: Vec<FieldElement> = initial_witness_vec_raw
        .into_iter()
        .map(|f| f as i128)
        .map(FieldElement::from)
        .collect();
    let mut initial_witness = WitnessMap::new();
    for (i, witness) in initial_witness_vec.into_iter().enumerate() {
        initial_witness.insert(Witness(i as u32 + 1), witness);
    }

    let (proof, verification_key) = prove(circuit_bytecode, initial_witness).unwrap();
    Proof {
        proof,
        verification_key,
    }
}

pub fn verify_swift(circuit_bytecode: String, proof: Proof) -> bool {
    verify(circuit_bytecode, proof.proof, proof.verification_key).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{prove_swift, verify_swift};

    const BYTECODE: &str = "H4sIAAAAAAAA/7WTMRLEIAhFMYkp9ywgGrHbq6yz5v5H2JkdCyaxC9LgWDw+H9gBwMM91p7fPeOzIKdYjEeMLYdGTB8MpUrCmOohJJQkfYMwN4mSSy0ZC0VudKbCZ4cthqzVrsc/yw28dMZeWmrWerfBexnsxD6hJ7jUufr4GvyZFp8xpG0C14Pd8s/q29vPCBXypvmpDx7sD8opnfqIfsM1RNtxBQAA";

    #[test]
    fn test_prove_verify() {
        let proof = prove_swift(String::from(BYTECODE), vec![1_i32, 10_i32]);
        let verdict = verify_swift(String::from(BYTECODE), proof);
        assert!(verdict);
    }
}
