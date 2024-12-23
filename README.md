## Introduction

HTLP-library is a Rust library for creating and solving Homomorphic Time Lock Puzzles (HTLP). These puzzles allow you to encrypt a message such that it can only be decrypted after a predetermined amount of computational effort. This is particularly useful for ensuring that a message remains unreadable until a specific time has passed.

## Features

- **Create Puzzles**: Easily generate homomorphic time lock puzzles.
- **Solve Puzzles**: Decrypt messages by solving the puzzles.
- **Configurable Difficulty**: Adjust the difficulty level to control the time required to solve the puzzle.

## Installation

To include HTLP-library in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
htlp-library = "0.1.0"
```

### Mathematical Background

Homomorphic Time Lock Puzzles rely on the difficulty of certain mathematical problems. The core idea is to use a function that is easy to compute in one direction but hard to reverse without significant computational effort.

#### Example: Modular Exponentiation

One common approach is to use modular exponentiation. Given a base \( g \), an exponent \( x \), and a modulus \( n \), the function \( f(g, x, n) = g^x \mod n \) is easy to compute. However, finding \( x \) given \( g \), \( g^x \mod n \), and \( n \) is computationally difficult if \( n \) is large enough.

### Creating a Puzzle

To create a puzzle, you can use the following steps:
1. Choose a large prime number \( p \) and a generator \( g \) of the multiplicative group of integers modulo \( p \).
2. Select a secret exponent \( x \) and compute \( y = g^x \mod p \).
3. The puzzle is then to find \( x \) given \( g \), \( y \), and \( p \).

### Solving a Puzzle

Solving the puzzle involves finding the exponent \( x \) such that \( g^x \equiv y \mod p \). This typically requires a brute-force search or other computationally intensive methods, ensuring that the puzzle cannot be solved quickly without the intended computational effort.

By adjusting the size of \( p \) and the range of \( x \), you can control the difficulty and the time required to solve the puzzle.

This mathematical foundation ensures that Homomorphic Time Lock Puzzles are secure and effective for time-based encryption.

## License

This project is licensed under the MIT License.
