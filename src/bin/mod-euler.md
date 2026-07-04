# Euler's Theorem and Modular Inverse via Exponentiation

## Overview

Euler's theorem generalizes Fermat's little theorem from prime moduli to any modulus n. It says that raising any integer a coprime to n to the power φ(n) always lands back on 1 modulo n — no matter how large a or n are, as long as they share no common factor. A small concrete case makes this tangible: for n = 9, φ(9) = 6, and indeed 2^6 = 64 = 7·9 + 1, so 2^6 ≡ 1 (mod 9), exactly as the theorem predicts. The theorem is proved by placing the integers coprime to n inside a group structure and invoking a general fact about groups — the order of any element divides the size of the group — developed in the next section. Beyond its own interest, the theorem gives a second way to compute modular inverses, alongside the extended-GCD approach from the Modular Inverse session, by exploiting the fact that raising a to one power short of φ(n) undoes raising it to the full φ(n).

## The multiplicative group mod n

The integers in [1, n] that are coprime to n form a set with rich algebraic structure. This set is closed under multiplication mod n: if gcd(a, n) = 1 and gcd(b, n) = 1, then gcd(a·b, n) = 1 as well. The proof goes by contradiction — if some prime p divided both a·b and n, then by the property that a prime dividing a product must divide one of its factors, p would divide a or divide b, contradicting gcd(a, n) = 1 or gcd(b, n) = 1. The residue a·b mod n inherits the same coprimality because gcd(a·b, n) = gcd(n, (a·b) mod n) by the Euclidean identity.

The set also contains 1 (which is coprime to everything) as the multiplicative identity, and every element has a modular inverse — which exists precisely because of coprimality. A set closed under an associative binary operation, with an identity and inverses for every element, is called a group. This group is written (Z/nZ)* and has order φ(n).

## Euler's theorem

In any group, the order of an element — the smallest positive integer k such that a^k equals the identity — divides the order of the group. This is a consequence of Lagrange's theorem. In (Z/nZ)*, the group order is φ(n). So if a^k ≡ 1 (mod n), then k divides φ(n), and therefore φ(n) = k·m for some integer m. This gives:

$$a^{\varphi(n)} = a^{km} = (a^k)^m \equiv 1^m \equiv 1 \pmod{n}$$

That is Euler's theorem: for any a with gcd(a, n) = 1,

$$a^{\varphi(n)} \equiv 1 \pmod{n}$$

## Fermat's little theorem as a special case

When n = p is prime, every integer in [1, p−1] is coprime to p, so (Z/pZ)* has order p − 1, meaning φ(p) = p − 1. Substituting into Euler's theorem gives Fermat's little theorem: for any prime p and any a not divisible by p,

$$a^{p-1} \equiv 1 \pmod{p}$$

This says that every base a coprime to a prime p, raised to the (p−1)th power, cycles back to 1 — a sharper, more specific fact than Euler's general result, since here the exponent is pinned to a single value, p−1, for every valid a simultaneously. That sharpness is exactly what the Miller-Rabin primality test, covered in a later session, exploits: composites can be screened by checking whether they fail this exact congruence.

## Modular inverse via Euler's theorem

Starting from a^φ(n) ≡ 1 (mod n), multiply both sides by a^(−1):

$$a^{\varphi(n)} \cdot a^{-1} \equiv a^{-1} \pmod{n}$$

The left side combines as a^(φ(n) − 1), giving:

$$a^{-1} \equiv a^{\varphi(n) - 1} \pmod{n}$$

This provides an alternative to extended GCD for computing modular inverses: compute φ(n) by trial division, then raise a to the power φ(n) − 1 using modular exponentiation.

## Correctness

The correctness of this alternative inverse formula rests entirely on Euler's theorem proved above: since a^φ(n) ≡ 1 (mod n) whenever gcd(a, n) = 1, multiplying by a^(−1) and simplifying shows a^(φ(n)−1) is exactly that inverse, with no additional argument needed. The existence condition is the same one that governs the extended-GCD approach from the Modular Inverse session: an inverse exists precisely when gcd(a, n) = 1, so the implementation returns the computed power when that holds and None otherwise, matching that earlier version's contract exactly.

## Complexity

The overall cost is dominated by computing φ(n), which requires trial division up to sqrt(n): O(sqrt(n)). The modular exponentiation costs O(log φ(n)) = O(log n). The bottleneck is O(sqrt(n)), which is worse than the extended GCD approach at O(log(min(a, n))).

When n is a known prime, φ(n) = n − 1 requires no factorization, so the inverse reduces to a^(n−2) mod n — computed in O(log n), matching the extended GCD complexity. This is the standard trick used when the modulus is a fixed prime.

## Worked example

Compute the modular inverse of 5 mod 9.

First check: gcd(5, 9) = gcd(9, 5) = gcd(5, 4) = gcd(4, 1) = 1. Coprime, so the inverse exists.

Compute φ(9): factor 9 = 3². Using the prime power formula, φ(9) = 9 − 3 = 6.

Compute 5^(6−1) mod 9 = 5^5 mod 9 using the same bit-by-bit square-and-multiply method traced in full in the Modular Exponentiation session's worked example. The binary of 5 is 101, so bits 0 and 2 are set: starting from power = 5 and result = 1, bit 0 folds power into result to get result = 5, and squaring gives power = 25 mod 9 = 7; bit 1 is unset, so only power is squared again, to 49 mod 9 = 4; bit 2 then folds power into result to give result = 5·4 = 20 mod 9 = 2.

So 5^5 mod 9 = 2. Verify: 5·2 = 10 ≡ 1 (mod 9). Correct.
