## KZG Commitment in Rust

This is a Rust implementation of the KZG commitment scheme. There are two main modules:
1. `kzg.rs` implements the basic polynomial commitment that allows both opening at a single point and also batch opening (sometimes known as multi proof).
2. `asvc.rs` implements a vector commitment scheme based on [this paper](https://eprint.iacr.org/2020/527.pdf). It supports proving vector position and also aggregating multiple KZG proofs into a single proof.


## Resources:
[Alin Tomescu’s notes on KZG](https://alinush.github.io/2020/05/06/kzg-polynomial-commitments.html)

[Dankrad Feist’s notes on KZG](https://dankradfeist.de/ethereum/2020/06/16/kate-polynomial-commitments.html)

### Original Implementation:
[eerkaijun/kzg-rust](https://github.com/eerkaijun/kzg-rust)
