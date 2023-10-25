# SWIFT-BRIDGE for `noir_rs`

![GitHub Workflow Status Fedora](https://github.com/visoftsolutions/noir_swift/actions/workflows/build&test@fedora.yml/badge.svg)
![GitHub Workflow Status Ubuntu](https://github.com/visoftsolutions/noir_swift/actions/workflows/build&test@ubuntu.yml/badge.svg)
![GitHub Workflow Status Macos](https://github.com/visoftsolutions/noir_swift/actions/workflows/build&test@macos.yml/badge.svg)
![Version](https://img.shields.io/badge/version-0.1.0-blue)

Bridge the gap between Swift and the `noir_rs` library, offering Swift developers a seamless way to utilize the `noir_rs` library's zkSNARK proving scheme functionalities across iOS, macOS, WatchOS, and other Swift-compatible platforms.

## 🚀 Features

- 📱 Directly use `noir_rs` zkSNARK functionalities in Swift.
- 🌉 Effortless interfacing between Swift and Rust.
- 🔒 Secure zkSNARK proof generation and verification.

## 📦 Rust Build

```bash
cargo build
```

## 🧪 Rust Testing

Ensure the reliability and robustness of your bridge with the provided tests:

```bash
cargo test
```

## 📦 Swift Build

```bash
# copy barretenberg lib to the swift directory
cp target/debug/build/barretenberg-<hash>/out/lib/libbarretenberg.a swift/
# copy noir_swift lib to the swift directory
cp target/debug/libnoir_swift.a swift/

cd swift

# compile swift project
./build

# run the program
./main.run
```

## 🎯 Quick Usage

We provide two main functions for the bridge:

1. **prove_swift**: Generates a zkSNARK proof using the given circuit bytecode and initial witness.
2. **verify_swift**: Verifies a given zkSNARK proof against a circuit and its verification key.

```rust
pub struct Proof {
    pub proof: Vec<u8>,
    pub verification_key: Vec<u8>,
}

fn prove_swift(circuit_bytecode: String, initial_witness_vec_raw: Vec<i32>) -> Option<Proof>;
fn verify_swift(circuit_bytecode: String, proof: Proof) -> Option<bool>;
```

## 🔍 Example

Refer to the `tests` module for a comprehensive usage example:
