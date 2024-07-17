# EpicChain.Cryptography.BLS12_381.Native

## Overview

`EpicChain.Cryptography.BLS12_381.Native` is a native library implemented in Rust, providing efficient and secure implementations of BLS (Boneh-Lynn-Shacham) signatures using the BLS12-381 curve. This library is designed to be a core component for cryptographic operations within the EpicChain ecosystem, ensuring high performance and robust security.

## Features

- **Efficient BLS Signatures**: Provides native implementations of BLS signatures using the BLS12-381 curve, optimized for performance.
- **Secure Cryptographic Operations**: Utilizes state-of-the-art cryptographic techniques to ensure the security of your data.
- **Interoperability**: Designed to be easily integrated with other components of the EpicChain ecosystem.

## Prerequisites

- **Rust Installation**: Ensure Rust is installed, at least version 1.50.0. You can install Rust using [rustup](https://rustup.rs/).
- **Cargo**: Rust's package manager, which comes with Rust.

## Installation

### 1. Cloning the Repository

Clone the repository to your local machine:
```sh
git clone https://github.com/epicchainlabs/EpicChain.Cryptography.BLS12_381.Native.git
cd EpicChain.Cryptography.BLS12_381.Native
```

### 2. Building the Library

1. Navigate to the cloned directory.
2. Build the library using Cargo:
    ```sh
    cargo build --release
    ```

The compiled library will be located in the `target/release` directory.

### 3. Integration

#### Rust Projects

1. Add `EpicChain.Cryptography.BLS12_381.Native` as a dependency in your `Cargo.toml`:
    ```toml
    [dependencies]
    epicchain-cryptography-bls12_381-native = { path = "../path_to_your_cloned_repo" }
    ```

2. Use the library in your project:
    ```rust
    extern crate epicchain_cryptography_bls12_381_native;

    use epicchain_cryptography_bls12_381_native::{BLSSecretKey, BLSPublicKey, BLSSignature};

    fn main() {
        // Initialize the library
        epicchain_cryptography_bls12_381_native::init();

        // Generate key pair
        let sk = BLSSecretKey::generate();
        let pk = sk.public_key();

        // Sign message
        let message = b"Hello, EpicChain!";
        let signature = sk.sign(message);

        // Verify signature
        let valid = pk.verify(&signature, message);
        if valid {
            println!("Signature is valid.");
        } else {
            println!("Signature is invalid.");
        }
    }
    ```

#### Other Languages

For projects in other languages (e.g., Python, JavaScript), use appropriate Foreign Function Interfaces (FFI) or bindings to integrate the native library.

## Usage

### Example: Generating and Verifying BLS Signatures

#### Generating a BLS Signature

```rust
extern crate epicchain_cryptography_bls12_381_native;

use epicchain_cryptography_bls12_381_native::{BLSSecretKey, BLSSignature};

fn main() {
    // Initialize the library
    epicchain_cryptography_bls12_381_native::init();

    // Generate a secret key
    let sk = BLSSecretKey::generate();

    // Message to be signed
    let message = b"Hello, EpicChain!";

    // Generate a signature
    let signature = sk.sign(message);

    println!("Signature: {:?}", signature);
}
```

#### Verifying a BLS Signature

```rust
extern crate epicchain_cryptography_bls12_381_native;

use epicchain_cryptography_bls12_381_native::{BLSSecretKey, BLSPublicKey, BLSSignature};

fn main() {
    // Initialize the library
    epicchain_cryptography_bls12_381_native::init();

    // Generate a secret key and corresponding public key
    let sk = BLSSecretKey::generate();
    let pk = sk.public_key();

    // Message to be signed
    let message = b"Hello, EpicChain!";

    // Generate a signature
    let signature = sk.sign(message);

    // Verify the signature
    let valid = pk.verify(&signature, message);
    if valid {
        println!("Signature is valid.");
    } else {
        println!("Signature is invalid.");
    }
}
```

## Contributing

We welcome contributions from the community. To contribute, follow these steps:

1. Fork the repository on GitHub.
2. Create a new branch for your feature or bugfix.
3. Commit your changes and push the branch to your fork.
4. Submit a pull request with a detailed description of your changes.

## License

`EpicChain.Cryptography.BLS12_381.Native` is released under the MIT License. See the [LICENSE](LICENSE) file for more details.

## Contact

For any questions or issues, please contact the EpicChain development team at [support@epic-chain.org](mailto:support@epic-chain.org).

By following this guide, you can effectively set up, build, and integrate `EpicChain.Cryptography.BLS12_381.Native` into your projects, ensuring secure and efficient cryptographic operations.