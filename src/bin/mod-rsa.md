# RSA Cryptosystem

## Overview

RSA is a public-key cryptosystem built on the algebraic structure of the multiplicative group modulo n = p·q. It synthesises every concept developed in the preceding sessions: primality testing (Miller-Rabin), the totient function, Euler's theorem, modular exponentiation, and modular inverse via the extended Euclidean algorithm.

## Key generation

Two large distinct primes p and q are chosen. Their product n = p·q is the public modulus. The group order is the totient

$$\varphi(n) = (p - 1)(q - 1)$$

This follows from the multiplicativity of φ and the prime formula φ(p) = p − 1. The public exponent e is any integer satisfying gcd(e, φ(n)) = 1 — coprimality ensures the modular inverse d exists. The private exponent d is then the unique solution to

$$d \cdot e \equiv 1 \pmod{\varphi(n)}$$

computed via extended GCD. The public key is the pair (n, e); the private key is (n, d). The security of the system rests on the fact that computing φ(n) from n alone requires factoring n into p and q — a problem for which no efficient algorithm is known for large n.

## Encryption and decryption

Given a message m with m < n, encryption produces ciphertext

$$c = m^e \bmod n$$

and decryption recovers

$$m = c^d \bmod n$$

Both operations are single calls to modular exponentiation.

## Correctness argument

Decryption recovers m because applying both exponents in sequence raises m to the power de:

$$c^d = (m^e)^d = m^{de} \pmod{n}$$

Since de ≡ 1 (mod φ(n)), there exists a positive integer k such that de = k·φ(n) + 1. Substituting:

$$m^{de} = m^{k \cdot \varphi(n) + 1} = m \cdot \left(m^{\varphi(n)}\right)^k \pmod{n}$$

Euler's theorem states that m^φ(n) ≡ 1 (mod n) whenever gcd(m, n) = 1, so the bracketed term vanishes and the result is m. The theorem applies because RSA requires m < min(p, q): a value smaller than both prime factors cannot be a multiple of either, which guarantees gcd(m, n) = 1. The same argument holds with e and d swapped, which is why signing (encrypt with d, verify with e) is mathematically equivalent.

The proof via Euler's theorem fails when gcd(m, n) > 1 — that is, when m is a multiple of p or q. For large primes the probability of this occurring at random is approximately 1/p, which is negligible in practice. A complete proof that covers even these edge cases uses CRT applied to the congruences mod p and mod q separately, but the Euler argument handles the relevant range.

## Textbook RSA and the padding gap

In this implementation encryption is deterministic: the same m always produces the same c. An attacker with the public key can precompute a dictionary mapping every possible ciphertext back to its plaintext, which breaks confidentiality for small message spaces. Real deployments prepend random padding to m before encryption (OAEP is the standard scheme), making each encryption probabilistic. The mathematical core developed here is correct; the padding layer is an engineering addition on top of it.

## Complexity

Key generation has two costs. Verifying primality of p and q via Miller-Rabin with a fixed set of bases is O(log n) per base, O(1) bases, so O(log n) total. Finding e requires checking candidates until gcd(e, φ(n)) = 1; the density of integers coprime to φ(n) in any interval is φ(φ(n))/φ(n), which is a small constant for typical φ(n), so the expected number of candidates is O(1). Each candidate costs O(log n) for the GCD check. Computing d via extended GCD is O(log n). Overall keygen is O(log n).

Encryption is O(log e). Since e is chosen by the implementer, it can be a number with very few set bits in binary (e = 65537 = 2^16 + 1 has only two set bits), making encryption very fast. Decryption is O(log d); since d is computed rather than chosen, it is generically a full-width number with O(log n) bits set, giving O(log n) multiplications.

The intentional asymmetry — fast encryption, slower decryption — matches the usage pattern: the public key is used by everyone who encrypts, the private key only by the owner who decrypts.

## Worked example

Take p = 7, q = 11 (below the implementation's minimum but useful for hand verification). Then n = 77 and φ(n) = 6 · 10 = 60. Choose e = 13: gcd(13, 60) = 1, confirmed by the Euclidean algorithm. Compute d as the inverse of 13 mod 60: 13 · 37 = 481 = 8 · 60 + 1, so d = 37.

Public key: (77, 13). Private key: (77, 37).

Encrypt m = 19: compute 19^13 mod 77 via square-and-multiply. The result is c = 61.

Decrypt c = 61: compute 61^37 mod 77. The result is 19, recovering the original message.

Check: de = 37 · 13 = 481 = 8 · 60 + 1, confirming de ≡ 1 (mod 60) with k = 8.
