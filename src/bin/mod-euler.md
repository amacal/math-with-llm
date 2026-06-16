# Euler's Theorem and Modular Inverse via Exponentiation

## The multiplicative group mod n

The integers in [1, n] that are coprime to n form a set with rich algebraic structure. This set is closed under multiplication mod n: if gcd(a, n) = 1 and gcd(b, n) = 1, then gcd(a·b, n) = 1 as well. The proof goes by contradiction — if some prime p divided both a·b and n, then by the prime divisibility property (p | a·b implies p | a or p | b), p would divide one of the factors, contradicting gcd(a, n) = 1 or gcd(b, n) = 1. The residue a·b mod n inherits the same coprimality because gcd(a·b, n) = gcd(n, (a·b) mod n) by the Euclidean identity.

The set also contains 1 (which is coprime to everything) as the multiplicative identity, and every element has a modular inverse — which exists precisely because of coprimality. A set closed under an associative binary operation, with an identity and inverses for every element, is called a group. This group is written (Z/nZ)* and has order φ(n).

## Euler's Theorem

In any group, the order of an element — the smallest positive integer k such that a^k equals the identity — divides the order of the group. This is a consequence of Lagrange's theorem. In (Z/nZ)*, the group order is φ(n). So if a^k ≡ 1 (mod n), then k | φ(n), and therefore φ(n) = k·m for some integer m. This gives:

$$a^{\varphi(n)} = a^{km} = (a^k)^m \equiv 1^m \equiv 1 \pmod{n}$$

That is Euler's Theorem: for any a with gcd(a, n) = 1,

$$a^{\varphi(n)} \equiv 1 \pmod{n}$$

## Fermat's Little Theorem as a special case

When n = p is prime, every integer in [1, p−1] is coprime to p, so (Z/pZ)* has order p − 1, meaning φ(p) = p − 1. Substituting into Euler's Theorem gives Fermat's Little Theorem: for any prime p and any a not divisible by p,

$$a^{p-1} \equiv 1 \pmod{p}$$

This is the theoretical foundation of the Miller-Rabin primality test.

## Modular inverse via Euler's Theorem

Starting from a^φ(n) ≡ 1 (mod n), multiply both sides by a^(−1):

$$a^{\varphi(n)} \cdot a^{-1} \equiv a^{-1} \pmod{n}$$

The left side combines as a^(φ(n) − 1), giving:

$$a^{-1} \equiv a^{\varphi(n) - 1} \pmod{n}$$

This provides an alternative to extended GCD for computing modular inverses: compute φ(n) by trial division, then raise a to the power φ(n) − 1 using modular exponentiation. The inverse exists and is returned when gcd(a, n) = 1; otherwise None is returned.

## Complexity

The overall cost is dominated by computing φ(n), which requires trial division up to sqrt(n): O(sqrt(n)). The modular exponentiation costs O(log φ(n)) = O(log n). The bottleneck is O(sqrt(n)), which is worse than the extended GCD approach at O(log(min(a, n))).

When n is a known prime, φ(n) = n − 1 requires no factorization, so the inverse reduces to a^(n−2) mod n — computed in O(log n), matching the extended GCD complexity. This is the standard trick used when the modulus is a fixed prime.

## Worked example

Compute the modular inverse of 5 mod 9.

First check: gcd(5, 9) = gcd(9, 5) = gcd(5, 4) = gcd(4, 1) = 1. Coprime, so the inverse exists.

Compute φ(9): factor 9 = 3². Using the prime power formula, φ(9) = 9 − 3 = 6.

Compute 5^(6−1) mod 9 = 5^5 mod 9 using square-and-multiply. Binary of 5 is 101.

- Start: power = 5, result = 1.
- Bit 0 (set): result = 1·5 = 5 mod 9 = 5. power = 5² = 25 mod 9 = 7.
- Bit 1 (unset): power = 7² = 49 mod 9 = 4.
- Bit 2 (set): result = 5·4 = 20 mod 9 = 2.

So 5^5 mod 9 = 2. Verify: 5·2 = 10 ≡ 1 (mod 9). Correct.
