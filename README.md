# `noir_swift`: Swift-based zkSNARK Proving&Verifying tool for noir-lang

Welcome to Swift-Bridge, a seamless connector between the Swift programming ecosystem and the `noir_rs` Rust library. This bridge is designed to provide Swift developers with easy and efficient access to `noir_rs`'s zkSNARK proving functionalities, making it highly applicable in iOS, macOS, WatchOS, and other platforms that support Swift.

- ![Version](https://img.shields.io/badge/version-0.19.4-darkviolet)

## Key Highlights:

- **Swift Integration**: Custom-built for Swift developers, enabling native access to zkSNARK tools within the Swift environment.
- **Rust-Swift Bridge**: Offers a fluid connection between Swift and Rust, providing a smooth and efficient integration.
- **Security Focused**: Ensures top-notch security with advanced zkSNARK proof generation and verification capabilities, integral for privacy and scalability in computing.

## Building & Testing Instructions:

### Building Rust Components:
```bash
cargo build
```

### Testing Rust Components:
For high confidence in the bridge's reliability, utilize the included test suites:
```bash
cargo test
```

### Building Swift Components:
```bash
# Transfer noir_swift library to Swift directory
cp target/debug/libnoir_swift.a swift/

cd swift

# Compile Swift project
./build

# Execute the program
./main.run
```

## Usage:

Swift-Bridge mainly provides two crucial functions:

1. **Prove in Swift**: Create a zkSNARK proof using provided circuit bytecode and initial witness.
2. **Verify in Swift**: Authenticate a zkSNARK proof against its respective circuit and verification key.

```rust
struct Proof {
    proof: Vec<u8>,
    verification_key: Vec<u8>,
}

fn prove_swift(circuit_bytecode: String, initial_witness_vec_raw: Vec<i32>) -> Option<Proof>;
fn verify_swift(circuit_bytecode: String, proof: Proof) -> Option<bool>;
```

## Example:

Here's an example using a noir circuit to verify if x is different from y:

```swift
let BYTECODE = "<Your Noir Bytecode>"

let witness = RustVec<Int32>()
witness.push(value: 1) // x value
witness.push(value: 2) // y value

guard let proof = prove_swift(BYTECODE, witness) else {
    return false
}

let verdict = verify_swift(BYTECODE, proof) ?? false
print(verdict)  // Outputs true if successful verification
```

## Future Developments:

We are constantly working to improve the Swift-Bridge, focusing on enhancing its compatibility with a broader range of Swift-supported platforms and streamlining its integration into Swift-based projects. 

Stay tuned for more updates and advancements in bridging Swift with the world of zkSNARKs!