# Cryptode

Introducing Cryptode: A Comprehensive Cryptography Framework

Cryptode is a state-of-the-art cryptography framework originally developed in pure Rust exclusively for the Rust programming language. However, it has now expanded its horizons and is available as an API for other programming languages, including Python and more.

With Cryptode, developers gain access to a wide range of powerful cryptographic functionalities. This framework leverages the robustness and efficiency of Rust to deliver secure and reliable encryption, decryption, and other cryptographic operations. By adhering to industry-standard cryptographic algorithms and protocols, Cryptode ensures the utmost security and integrity of data.

What sets Cryptode apart is its ability to seamlessly integrate into various programming environments. Whether you're a Rust enthusiast or prefer working with Python or other supported languages, Cryptode's versatile API allows you to harness its cryptographic capabilities in your preferred development ecosystem.

By incorporating Cryptode into your projects, you can confidently handle sensitive data, secure communications, and implement cutting-edge cryptographic techniques. Its availability across multiple programming languages broadens its reach and enables developers from different communities to leverage its robust cryptographic features.

Stay at the forefront of data security with Cryptode. Explore its vast array of cryptographic functions and unlock the power of secure communication and information protection in your applications.

# Installation

Add following line in your `Cargo.toml` file

```rust
[dependencies]
Cryptode = { git = "https://github.com/enginestein/Cryptode.git" }
```

# Usage

Import Cryptode in following syntax:

```rust
extern crate Cryptode;
use Cryptode::FileName::FunctionName;
```

For example, here is example encoding in `blake2b` encryption system:

```rust
extern crate Cryptode;
use Cryptode::blake2b::blake2b;

fn main() {
let message = b"Hello, World!";
let key = b"SecretKey";
let output_size = 32; // Output size in bytes

let hash = blake2b(message, key, output_size);

println!("Hash: {:?}", hash);
}
```

Documentation is available [here](https://enginestein.github.io/Cryptode/)

# Features

- Written in pure rust for rust

- Secure algorithms

- Multiple approaches to select from

- Advanced approaches

- 50+ old and new algorithms
