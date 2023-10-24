# SWIFT-BRIDGE for `noir_rs`

![GitHub Workflow Status](https://github.com/visoftsolutions/noir_swift/actions/workflows/rust.yml/badge.svg)
![Version](https://img.shields.io/badge/version-0.1.0-blue)

Bridge the gap between Swift and the `noir_rs` library, offering Swift developers a seamless way to utilize the `noir_rs` library's zkSNARK proving scheme functionalities across iOS, macOS, WatchOS, and other Swift-compatible platforms.

## 🚀 Features

- 📱 Directly use `noir_rs` zkSNARK functionalities in Swift.
- 🌉 Effortless interfacing between Swift and Rust.
- 🔒 Secure zkSNARK proof generation and verification.

## 📦 Build

```bash
cargo build
```

## 🧪 Testing

Ensure the reliability and robustness of your bridge with the provided tests:

```bash
cargo test
```

## 🎯 Quick Usage

We provide two main functions for the bridge:

1. **prove_swift**: Generates a zkSNARK proof using the given circuit bytecode and initial witness.
2. **verify_swift**: Verifies a given zkSNARK proof against a circuit and its verification key.

```rust
fn prove_swift(circuit_bytecode: String, initial_witness_vec_raw: Vec<i128>) -> Result<Proof, String>
fn verify_swift(circuit_bytecode: String, proof: Proof) -> Result<bool, String>
```

## 🔍 Example

Refer to the `tests` module for a comprehensive usage example:
