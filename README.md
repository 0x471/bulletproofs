# Bulletproofs

Bulletproofs, is a non-interactive zero-knowledge proof protocol with very short proofs and with no trusted setup.

It's well suited especially for efficient range proofs on commited values. It also supports aggregating range proofs.

---

## Introduction

Bulletproofs enable a prover to convince a verifier that they correctly computed an inner product between two vectors without revealing any information about the vectors. This is called an inner product argument (IPA).

Let's say we have two vectors:

$$
\mathbf{a} = (a_1, a_2, a_3)
$$ 

$$
\mathbf{b} = (b_1, b_2, b_3)
$$

The inner product of these two vectors is:

$$
\mathbf{a} \cdot \mathbf{b} = a_1b_1 + a_2b_2 + a_3b_3
$$

The prover can optionally hide or reveal the values of the vectors or the inner product result but still convince the verifier that they correctly computed the inner product.

The verifier doesn't receive vectors or the inner product result (scalar value) but rather commitments to the vectors. The verifier can then use these commitments to compute the inner product of the vectors and verify that the prover knows the values of the vectors.

Bulletproofs use Pedersen Commitments to commit to the vectors. They're based on elliptic curves and can be reconstructed without revealing the original value of the committed value.

Bulletproofs allow to create a proof for the R1CS (Rank-1 Constraint System) directly without a QAP (Quadratic Arithmetic Program).

Bulletproofs do not use pairings, they mainly use elliptic curve addition and scalar multiplication.

The primary tradeoff of Bulletproofs is that the runtime of the verifier is linear in the size of the circuit. This is because the work that would have been done by the trusted setup in other SNARKs is now done by the verifier.

One major advantage of IPA is they don't require an arithmetic circuit to be constructed. For example proving that a number `v` is in the range `[0, 2^n)` can be done by showing that `v`has a binary representation of `b`and the inner product of `b`and the vector `[1, 2, 4, 8, ..., 2^(n-1)]` is equal to `v`. This implies that `v` is in the range `[0, 2^n)`. This is called a range proof.

Monero uses Bulletproofs for their range proofs to prove that the amount being sent is positive and does not exceed the balance of the sender. Also ZCash uses Bulletproofs as a replacement for the SNARK polynomial commitment using a PLONKish circuit.

The linear runtime of Bulletproofs make them unsuitable for proving statements with large circuits, for example use in smart contracts on Ethereum but for protocols that need a fast proof generation and verification of a small problem, they're one of the best options.

---







## Resources
- [Bulletproofs](https://eprint.iacr.org/2017/1066.pdf)
- [RareSkills - ZK Book](https://www.rareskills.io/zk-book)
- [Mina Book by o1Labs](https://o1-labs.github.io/proof-systems/plonk/inner_product.html)
- [Proofs for Inner Pairing Products and Application](https://eprint.iacr.org/2019/1177.pdf)