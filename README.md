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

## Pedersen Commitments

Pedersen commitments allow us to encode arbitrarily large numbers into a single elliptic curve point while optionally hiding any information about the vector. This way we can make claims about a vector without revealing the vector itself.

These vectors can't be just random values, they have to be mathematical entities in the real world. Generally, the prover doesn't want to just pass the two vectors to the verifier,but they still need to pass something to the verifier to represent that they've selected a pair of vectors and cannot change it.

In an inner product argument, the prover provides two commitments to two vectors, then provides a proof that the inner product of the two vectors is equal to some value.

Normally, traditional commitment schemes has the form where the commitment is the hash of the value and a random blinding factor which is called salt or randomizer to prevent an attack from brute-forcing the value.

$$
C = H(v, r)
$$

For example, if we were committing a vote and there are only a limited number of choices, the vote selection can be guessed by trying all the votes seeing which hash matches the commitment.

During the reveal phase, the commiter reveals the value and the salt so the other party can validate that it is the same value they committed to. It is not possible to get another value and salt that hash to the same commitment so this means this scheme is binding - the commiter cannot change their commitment after it's sent. Also since the commitment cannot be forged, we say it is hiding - the commiter cannot reveal the value without revealing the salt.

A value and salt pair that results in the hash/commitment is called the opening of the commitment.

When discussing Pedersen commitments, there's a distinction between knowing the opening and opening the commitment. We usally want to prove we know the opening without necesarily opening it.

Pedersen commitments behave very similarly to hash functions. The main difference is they use elliptic curve groups instead of hash functions.

Elliptic curve cryptography is dependent on the discerete logarithm problem. Considering this computation is very hard, it makes it very difficult to reverse the commitment since given elliptic curve points `V`and `U`, we cannot compute `x` where `V = x * U` because we don't know how many times `U` needs to be added to itself to get `V`.

We'll still refer to `x` as the discrete logarithm of `V` because we know it exists but it's very hard to compute.

All (cryptogaphic) elliptic curves have a discrete logarithm, even if they can't be computed.

In this sense, elliptic curve multiplication behaves like a hash function. They're binding as long as we allow oppenings within the curve order. (Curve order is the number of points in the elliptic curve)

However, if the range of discrete logarithms is limited (for example voting choices) to the curve order, we can use the discrete logarithm to reverse the commitment, so it might become guessable.

We can make a Pedersen commitment hiding in the following way:

$$
C = v * G + s * B
$$

Where `G` and `B` are points on the elliptic curve, `s` is the salt (or blinding factor) and `v` is the value we want to commit to.

The points `B`and `G` are public and known to everyone but the committer does not know the discrete logarithm of `B` and `G` so it's very hard to reverse the commitment.


Here's why the commiter must not know the discrete logarithm of `B` and `G`:

Let's say that the commiter knows `b` such that `B = bG`. In that case, they can open the commitmment:

$$
C = vG + sB
$$

to a different (v', s') pair they originally committed to.

They can cheat by opening the commitment to a different value because they know `b`:

C = vG + sB
  = vG + s(bG) (substitute B with bG)
  = (v + sb)G (factor out G)

The commiter can pick a new value `v'` and compute `s'` from the original commitment:

$$
v' + s'b = v + sb
$$
and then compute:

$$
s' = (v + sb - v') / b
$$

Then the prover presents `v'` and `s'` as the valid commitment to the new value.

This works because the commiter knows the discrete logarithm of the elliptic curve points being used. `B` and `G` in this case.

To make sure that this process is done correctly there are 2 main ways:

1. The verifier supply the elliptic curve points for the committer.
2. A simpler way, picking up elliptic curve points in a random and transparent way so that the commiter can't know the discrete logarithm of the points.

For example we can start with the generator point, hash the `x`and `y` values, then use that as a seed for a pseudorandom but deterministic search for the next point.


What makes Pedersen commitments special is that they're homomorphic. This means that if we have two commitments:

$$
C_1 = v_1G + s_1B
$$

$$
C_2 = v_2G + s_2B
$$

We can add them together to get a new commitment:

$$
C_1 + C_2 = (v_1 + v_2)G + (s_1 + s_2)B
$$

If we include random blinding factors, we can still do a valid opening by adding the blinding factors together and proving that to the verifier.

Regular hash functions are not homomorphic, for example SHA256 of two values is not equal to the SHA256 of the sum of the values.

This way Pedersen commitments allow a prover to make claims about the sums of commited values without revealing the values themselves.  


We can take Pedersen commitments one step further and commit a set of values rather than a value and a blinding factor. This is called Pedersen Vector Commitments.

Suppose we have a set of random ellipic curve points (that we don't know the discrete logarithm of) and we want to commit to a set of values `(v_1, v_2, ..., v_n)`.

We can do this by taking the inner product of the values and the points:

$$
C = v_1G_1 + v_2G_2 + ... + v_nG_n + sB
$$

This is a valid commitment to the values `(v_1, v_2, ..., v_n)`. This lets us commit `n` values to `C`and hide it with `s`.

Since the committer does not know the discrete logarithm of the points, they cannot reveal the values without revealing `s`. That's why this scheme is binding and they can only reveal the values to product `C` not any other vector.

Pedersen vector commitments are homomorphic. This means that if we have two commitments:

$$
C_1 = v_1G_1 + v_2G_2 + ... + v_nG_n + s_1B
$$  

$$
C_2 = w_1G_1 + w_2G_2 + ... + w_nG_n + s_2B
$$

We can add them together to get a new commitment:

$$
C_1 + C_2 = (v_1 + w_1)G_1 + (v_2 + w_2)G_2 + ... + (v_n + w_n)G_n + (s_1 + s_2)B
$$

This lets us add commitments together and prove that the sum of the commitments is equal to a given value.

Now how can we generate these random points?

We can start with the generator point, one solution is trusted setup but this is not necessary.. The committer is able to set up the points in a way they cannot know their discrete logarithm by randomizing the points in a transparent way.

They can pick the generator point, mix in a publicly chosen random number, and hash that result to obtain another value. If that results in an x value that lies on the elliptic curve, they can use that as the next generator and hash the pair again to get another point. If the value is not on the curve, they can increment the random number and try again until i tdoes. Because the committer is not generating the points, they don't know their discrete logarithm.
























## Resources
- [Bulletproofs](https://eprint.iacr.org/2017/1066.pdf)
- [RareSkills - ZK Book](https://www.rareskills.io/zk-book)
- [Mina Book by o1Labs](https://o1-labs.github.io/proof-systems/plonk/inner_product.html)
- [Proofs for Inner Pairing Products and Application](https://eprint.iacr.org/2019/1177.pdf)