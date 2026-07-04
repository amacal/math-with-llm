# Miller-Rabin Primality Test

## Overview

Miller-Rabin is a fast, randomized test for deciding whether a number n is prime. Rather than proving primality with certainty, it either declares n definitely composite, when it can produce hard evidence of that fact, or reports that n is probably prime, with the probability of being wrong controlled by how many independent checks are run. This trade of absolute certainty for speed is what makes the test practical for numbers with hundreds of digits, the sizes used in real cryptographic systems, where trial division or even sieving would be far too slow. The test builds directly on Fermat's little theorem, but strengthens it to close a specific gap where Fermat's test alone can be fooled.

## The problem with Fermat's test

Fermat's little theorem says that for a prime p and any a with gcd(a, p) = 1, we have a^(p-1) ≡ 1 (mod p). The contrapositive gives a fast compositeness certificate: if a^(n-1) ≢ 1 (mod n), then n is definitely composite, and a is called a Fermat witness. The trouble is the other direction. Composites can satisfy a^(n-1) ≡ 1 (mod n) for every base a coprime to n — these are Carmichael numbers, and the smallest is 561 = 3 × 11 × 17.

The reason 561 fools the Fermat test is captured by Korselt's criterion, which characterizes exactly which squarefree composites behave this way:

$$n \text{ is a Carmichael number} \iff (p-1) \text{ divides } (n-1) \text{ for every prime factor } p \text{ of } n$$

This says the condition is checked one prime factor at a time, not on the totient of n as a whole. For 561, φ(561) = 2 × 10 × 16 = 320, and n-1 = 560; the key is not that 320 divides 560 (it does not), but that each individual factor does — 2 divides 560, 10 divides 560, and 16 divides 560, since 560 = 2×280 = 10×56 = 16×35. By the Chinese Remainder Theorem, a^560 ≡ 1 (mod 561) then follows from a^560 ≡ 1 (mod 3), a^560 ≡ 1 (mod 11), and a^560 ≡ 1 (mod 17) each holding separately, and each is guaranteed because (p-1) dividing 560 lets the Fermat exponent be lifted from p-1 up to the larger multiple 560 by periodicity.

The "if" direction of Korselt's criterion generalizes this example to any squarefree n. Write n = p1 × p2 × ... × pk with (pi - 1) dividing (n-1) for every i. The Chinese Remainder Theorem says a value mod n is fully determined by its values mod each pi separately, and Fermat's little theorem gives a^(pi-1) ≡ 1 (mod pi) for any a coprime to pi. Since (pi-1) divides (n-1), raising both sides of that congruence to the appropriate power extends it to a^(n-1) ≡ 1 (mod pi) for every i, and CRT recombines these per-prime facts into a^(n-1) ≡ 1 (mod n) — exactly the condition a Fermat test checks, and it holds for every base a coprime to n at once.

## The Miller-Rabin strengthening

Miller-Rabin exploits a property of primes that Fermat does not: in Z/pZ for prime p, the polynomial x^2 - 1 = (x-1)(x+1) has exactly two roots, namely x ≡ 1 and x ≡ -1. This follows because p is prime and therefore must divide one of the two factors (x-1) or (x+1). A composite modulus can have more square roots of 1: for n = 15, the equation x^2 ≡ 1 (mod 15) has four solutions (1, 4, 11, 14), because the factors 3 and 5 of 15 can distribute between (x-1) and (x+1) independently via CRT.

The test writes n-1 = 2^s × d where d is odd and considers the sequence

$$a^d,\ a^{2d},\ a^{4d},\ \ldots,\ a^{2^{s-1} d},\ a^{2^s d} = a^{n-1}$$

obtained by starting at a^d and squaring s times. For a prime p, the final term is 1 by Fermat. Working backwards, the term just before the first occurrence of 1 must be a square root of 1 mod p — and that square root must be ±1. So either the very first term a^d ≡ 1, or somewhere in the sequence we see -1 before reaching 1. A composite can fail both conditions for some base a, yielding a third square root of 1, which is impossible mod a prime — and in that case a is a strong witness to compositeness.

The formal test: given a and n, compute a^d mod n. If it equals 1, report MaybePrime. Otherwise square repeatedly; if any of the s-1 subsequent values equals -1 ≡ n-1, report MaybePrime. If neither condition is met anywhere in the sequence, report IsComposite.

## Correctness

The algorithm is correct in two complementary senses. For a genuine prime p, the square-root argument above guarantees the sequence always either starts at 1 or passes through -1, so the test always reports MaybePrime — there are no false negatives at all. For a composite n, correctness is only probabilistic: a base a that fails to detect a composite is called a strong liar, and Rabin proved that for any composite n, at most 1/4 of all bases in {1, ..., n-1} are strong liars. This 1/4 bound is not just a convenient round number — it is provably tight, attained exactly for certain composites, so the k-independent-bases argument below cannot be improved for every n, even though most composites in practice have far fewer strong liars than the worst case. Running the test with k independent random bases gives a false positive probability of at most (1/4)^k. In practice k = 20 suffices for cryptographic use, giving a false positive rate below 10^{-12}.

## Complexity

The decomposition of n-1 strips one bit per iteration: O(log n). The squaring loop runs at most s ≤ log₂(n) steps with one modular multiplication each: O(log n). The initial modular exponentiation a^d with d ≤ n-1 costs O(log n). The total cost of a single-base test is therefore O(log n), and running k bases costs O(k log n).

## Edge cases

When n-1 is odd, meaning s = 0 in the decomposition n-1 = 2^s × d, the squaring loop never executes and the algorithm degenerates to a single Fermat check: it reports MaybePrime exactly when a^d ≡ 1 (mod n), and IsComposite otherwise. This situation arises precisely when n itself is even, since n-1 odd is equivalent to n even, and since the only even prime is 2, losing Miller-Rabin's strengthening is harmless for correctly classifying n = 2 itself. It is not harmless for every base, however: with a = 1, a^d mod n is always 1 regardless of n, so a = 1 would report MaybePrime for every even n, including composite ones. The implementation does not explicitly exclude a = 1 as a base, so correctness on even inputs ultimately still depends on the caller supplying a nontrivial base.

## Worked example

Take n = 561 = 3 × 11 × 17 (a Carmichael number) and base a = 2.

First, decompose n-1 = 560 = 2^4 × 35, giving s = 4 and d = 35.

Compute the sequence starting at 2^35 mod 561, squaring at each step:

$$2^{35} \equiv 263,\quad 2^{70} \equiv 166,\quad 2^{140} \equiv 67,\quad 2^{280} \equiv 1,\quad 2^{560} \equiv 1 \pmod{561}$$

The first term is not 1, and no term equals n-1 = 560. The test returns IsComposite. The key observation is that 2^140 ≡ 67 (mod 561) and 2^280 ≡ 1 (mod 561), yet 67 is neither 1 nor 560. This is a third square root of 1 modulo 561, which is impossible modulo a prime. The factor structure of 561 is what allows it: 67^2 ≡ 1 (mod 3), (mod 11), and (mod 17) hold independently, and CRT combines them into 67^2 ≡ 1 (mod 561) without 67 being ±1 globally.
