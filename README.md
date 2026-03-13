# 🦀 Lattice-Chat: Post-Quantum Secure Messaging

Lattice-Chat is a research-oriented messaging system built in Rust. It implements a full networking stack combined with a Post-Quantum Cryptography (PQC) layer based on Learning With Errors (LWE) and the LLL (Lenstra–Lenstra–Lovász) lattice basis reduction algorithm.

The goal of this project is to demonstrate an end-to-end encrypted chat where security relies on the hardness of lattice problems (Shortest Vector Problem), while also providing tools to simulate lattice-based attacks.

## 🗺️ Development Roadmap

### Phase 1: Lattice Algebra & Core Math (March 12 - 14)

[x] March 12: Implement foundational linear algebra structures (Matrix and Vector).

[ ] March 13:

- Refactor core structures using Rust Generics (<T>) to seamlessly support both floats and integers.
- Implement the modular arithmetic structure (Z_q).
- Implement extended_gcd.

[ ] March 14: Implement the Gram-Schmidt orthogonalization process and the LLL (Lenstra–Lenstra–Lovász) lattice reduction algorithm.

### Phase 2: Asynchronous Networking Stack (March 15)

[ ] March 15: Build the concurrent networking layer for the chat application using the tokio runtime (TCP/WebSockets).

### Phase 3: Cryptography & LWE Implementation (March 17 - 18)

[ ] March 17: Implement the ChaCha20 stream cipher for fast, secure symmetric encryption of chat payloads.

[ ] March 18: Implement the LWE (Learning With Errors) protocol for secure post-quantum key exchange.

### Phase 4: Cryptanalysis & Benchmarks (March 19–20)

[ ] Simulate lattice-based attacks using the implemented LLL algorithm.

[ ] Benchmark attack success rate with respect to: - lattice dimension - noise magnitude - modulus size

[ ] Measure runtime performance and algorithmic scaling.
