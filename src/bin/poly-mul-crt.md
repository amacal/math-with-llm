# Multi-Modulus NTT with CRT Reconstruction

## Overview

This session extends the single-prime NTT from `poly-mul-ntt.rs` to handle polynomial products whose coefficients exceed any single NTT prime. The idea is to run the NTT independently modulo several primes, then use the Chinese Remainder Theorem to reconstruct each true coefficient from its residues. The session is a thin composition of two previously developed tools — NTT and CRT — with the new insight being exactly when and why the composition is correct.

## The single-prime limitation

The NTT computes the convolution of two polynomials exactly modulo the chosen prime p. Each output coefficient is a residue mod p, which equals the true integer coefficient only if that coefficient is strictly less than p. The maximum output coefficient across all positions is bounded by

$$n \cdot B^2$$

where n is the number of terms and B is the largest input coefficient, because the convolution formula

$$c[k] = \sum_i a[i] \cdot b[k - i]$$

sums at most n products, each at most B². When this bound meets or exceeds p, at least one output coefficient might wrap around modulo p, giving a silently wrong answer.

The three NTT-friendly primes used in this session are 998244353, 985661441, and 469762049, all of the form

$$k \cdot 2^m + 1 \quad \text{with } m \geq 22$$

which guarantees they support transform sizes up to at least 2^22. Each prime is around 10^9, so any single one fails for inputs where n·B² meets or exceeds 10^9. For example, eight-term polynomials with coefficients near 10^8 produce output coefficients around

$$8 \cdot (10^8)^2 = 8 \cdot 10^{16}$$

which is far above any one prime.

## CRT reconstruction

If the NTT is run separately modulo each of p1, p2, p3, then for each output position k the three computations produce c[k] mod p1, c[k] mod p2, and c[k] mod p3 respectively. Because p1, p2, p3 are distinct primes they are pairwise coprime, so the CRT guarantees a unique value in the range

$$[0,\ p_1 \cdot p_2 \cdot p_3)$$

satisfying all three congruences simultaneously. That unique value equals the true integer c[k] exactly when c[k] is strictly less than this product, because then c[k] is already the unique representative of its residue class in the combined range. The product of the three primes is approximately 4.6·10^26, far exceeding u64::MAX which is approximately 1.8·10^19, so any output coefficient that fits in a u64 is also below the prime product — the reconstruction is exact for all inputs where the true coefficients are representable as u64.

The CRT chaining follows the associative structure of the theorem. A single CRT call combines two residues (r1 mod p1, r2 mod p2) into a combined residue mod p1·p2 via the construction from `gcd-crt.md`: write x = r1 + k·p1 and solve for k from the second congruence, giving

$$k \equiv (r_2 - r_1) \cdot p_1^{-1} \pmod{p_2}$$

where the right-hand side is computed via the modular inverse of p1. A second CRT call then combines the result mod p1·p2 with r3 mod p3 to produce the final value mod p1·p2·p3. The intermediate combined modulus p1·p2 is approximately 9.8·10^17 and fits in u64, but the final combined modulus p1·p2·p3 does not, so the CRT function was updated to operate in u128 throughout.

## Implementation structure

The function runs a single loop over the three primes. For each prime it computes a primitive root, builds the forward and inverse root-of-unity tables (powers of h and h^(−1) respectively, where

$$h = g^{(p-1)/n}$$

is an element of order exactly n), zero-pads both input polynomials to length 16, runs the forward NTT on each, multiplies pointwise, and runs the inverse NTT to recover the coefficients modulo that prime. A running `solution` array accumulates the CRT-combined result: after the first prime it holds the raw residues mod p1; after each subsequent prime it holds the combined residue mod the product of all primes seen so far. The combined modulus is recomputed at each step as the product of the primes already processed.

Two bugs were found during the session. First, the inverse NTT requires dividing by the transform size n = 16, so the pre-computed inverse should be mod_inverse(16, p), not mod_inverse(p − 1, p). With p = 17 these coincide because 17 − 1 = 16, masking the error; with p = 998244353 they diverge and produce wrong output. Second, an early draft of the chaining loop passed results[0] (residues mod p1 only) as the first argument of every CRT call instead of the running combined solution, so the second CRT call effectively combined p1 and p3 residues while ignoring the p2 information entirely.

## Correctness

The correctness of each NTT pass is inherited from `poly-mul-ntt.md` — the recursive evaluate–multiply–interpolate argument establishes that the inverse NTT recovers exact coefficients modulo the chosen prime, provided the prime exceeds every true coefficient. The correctness of CRT reconstruction rests on two facts from `gcd-crt.md`: existence (there is always a value in the combined range satisfying all three congruences) and uniqueness (there is only one such value). Uniqueness is the load-bearing half: it guarantees that the value CRT returns is the only candidate, and since the true coefficient c[k] itself satisfies all three congruences by construction — it was reduced modulo each prime to produce the residue inputs — c[k] must be that unique value. That unique value equals c[k] exactly because

$$c[k] < p_1 \cdot p_2 \cdot p_3$$

ensures c[k] already lies in the range where no reduction occurred.

## Complexity

Each of the three NTT passes costs O(n log n). The coefficient-wise CRT reconstruction performs two CRT calls per output position, each costing O(log p) for the modular inverse inside, giving O(n log p) total for reconstruction. Since p is a fixed prime around 10^9, log p is a small constant, and the reconstruction is effectively O(n). The dominant cost is the three NTT passes, giving overall

$$O(n \log n)$$

— the same asymptotic class as the single-prime NTT, with a constant factor of three for the extra passes.

## Edge cases

If the true coefficient equals or exceeds p1·p2·p3 (equivalently, if n·B² is at least as large as p1·p2·p3), the CRT value still satisfies all three congruences but wraps around and does not equal the true coefficient. This is the same failure mode as the single-prime NTT, just pushed to a much higher threshold. The implementation assumes output coefficients fit in u64 and casts the u128 CRT result to u64 at the return boundary; inputs violating this assumption produce silently truncated output. For applications where this matters, the function signature should return u128 directly.

The CRT function uses checked arithmetic (checked_mul, checked_add) to propagate None on overflow rather than producing wrong answers silently. The product p1·p2·p3 is approximately 4.6·10^26, which fits comfortably in u128 whose maximum is approximately 3.4·10^38, so these checks serve as a safety net for wildly wrong inputs rather than a concern for normal usage with the three chosen primes.

## Worked example

Multiply A(x) = 1 + 2x + 3x² by B(x) = 2 + 3x + x² using multi-modulus NTT with p1 = 5, p2 = 17, p3 = 97. The true product, by direct convolution, is C(x) = 2 + 7x + 13x² + 11x³ + 3x⁴ — the largest coefficient is 13, so any prime exceeding 13 would suffice, but we use three small primes here to trace the reconstruction mechanism explicitly. Running the NTT mod p1 = 5 gives coefficient residues (2, 2, 3, 1, 3), since 13 mod 5 = 3 and 7 mod 5 = 2 and so on. Running mod p2 = 17 gives (2, 7, 13, 11, 3) — all coefficients are below 17 so no reduction occurs. Running mod p3 = 97 gives the same (2, 7, 13, 11, 3) for the same reason. Applying CRT to the first coefficient: all three residues agree on 2, so CRT trivially returns 2. For the second coefficient, residues are (2, 7, 7) mod (5, 17, 97): the first CRT call combines (2 mod 5, 7 mod 17) into a value mod 85 by writing x = 2 + 5k and solving 2 + 5k ≡ 7 mod 17, giving 5k ≡ 5 mod 17 so k = 1 and x = 7; the second combines (7 mod 85, 7 mod 97) trivially to give 7 mod 8245, which equals 7 exactly since 7 is far below 8245. The coefficient 13 has residues (3, 13, 13) mod (5, 17, 97): combining (3 mod 5, 13 mod 17) gives x = 3 + 5k with 3 + 5k ≡ 13 mod 17, so 5k ≡ 10 mod 17, and since 5^(−1) ≡ 7 mod 17 we get k ≡ 70 ≡ 2 mod 17, giving x = 3 + 10 = 13; combining (13 mod 85, 13 mod 97) gives 13 mod 8245, which equals 13 exactly. All five coefficients reconstruct to their true integer values, confirming the scheme.
