# Swift-Bridge for `noir_rs`

- ![Version](https://img.shields.io/badge/version-0.16.7-darkviolet)

Swift-Bridge connects the Swift ecosystem to the `noir_rs` Rust library. This provides Swift developers with a streamlined experience when integrating `noir_rs`'s zkSNARK proving functionalities on `iOS`, `macOS`, `WatchOS`, and other platforms supported by Swift.

## 🚀 Features

- 📱 Native access to `noir_rs` zkSNARK tools within Swift.
- 🌉 Smooth integration between Swift and Rust, making the bridge almost invisible.
- 🔒 Ensured security with zkSNARK proof generation and verification.

## Building & Testing

### 📦 Building Rust

```bash
cargo build
```

### 🧪 Testing Rust

To maintain the utmost confidence in the bridge, leverage the provided tests:

```bash
cargo test
```

### 📦 Building Swift

```bash
# Copy noir_swift lib to the swift directory
cp target/debug/libnoir_swift.a swift/
# Copy barretenberg lib to the swift directory
cp target/debug/build/barretenberg-<hash>/out/lib/libbarretenberg.a swift/

cd swift

# Compile the Swift project
./build

# Run the program
./main.run
```

## 🎯 Usage

The bridge offers two main functions:

1. **prove_swift**: Create a zkSNARK proof using provided circuit bytecode and an initial witness.
2. **verify_swift**: Authenticate a zkSNARK proof against its respective circuit and verification key.

```rust
pub struct Proof {
    pub proof: Vec<u8>,
    pub verification_key: Vec<u8>,
}

fn prove_swift(circuit_bytecode: String, initial_witness_vec_raw: Vec<i32>) -> Option<Proof>;
fn verify_swift(circuit_bytecode: String, proof: Proof) -> Option<bool>;
```

## 🔍 Example

In this example, we're using a noir circuit that verifies if x is different from y:

```swift
let BYTECODE = "H4sIAAAAAAAA/7WTMRLEIAhFMYkp9ywgGrHbq6yz5v5H2JkdCyaxC9LgWDw+H9gBwMM91p7fPeOzIKdYjEeMLYdGTB8MpUrCmOohJJQkfYMwN4mSSy0ZC0VudKbCZ4cthqzVrsc/yw28dMZeWmrWerfBexnsxD6hJ7jUufr4GvyZFp8xpG0C14Pd8s/q29vPCBXypvmpDx7sD8opnfqIfsM1RNtxBQAA"

let witness = RustVec<Int32>()
witness.push(value: 1)
witness.push(value: 2)

guard let proof = prove_swift(BYTECODE, witness) else {
    return false
}

let verdict = verify_swift(BYTECODE, proof) ?? false

print(verdict)  // Outputs true upon successful verification
```