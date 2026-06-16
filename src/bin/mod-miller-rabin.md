# Miller-Rabin Primality Test

## The problem with Fermat's test

Fermat's little theorem says that for a prime p and any a with gcd(a, p) = 1, we have a^(p-1) ≡ 1 (mod p). The contrapositive gives a fast compositeness certificate: if a^(n-1) ≢ 1 (mod n), then n is definitely composite, and a is called a Fermat witness. The trouble is the other direction. Composites can satisfy a^(n-1) ≡ 1 (mod n) for every base a coprime to n — these are Carmichael numbers, and the smallest is 561 = 3 × 11 × 17.

The reason 561 fools the Fermat test is captured by Korselt's criterion: a squarefree composite n is a Carmichael number if and only if (p-1) | (n-1) for every prime factor p of n. For 561: φ(561) = 2 × 10 × 16 = 320, and n-1 = 560. The key is not that 320 | 560 (it does not), but that each individual (p-1) divides 560: 2 | 560, 10 | 560, 16 | 560. By CRT, a^560 ≡ 1 (mod 561) follows from a^560 ≡ 1 (mod 3), a^560 ≡ 1 (mod 11), and a^560 ≡ 1 (mod 17), each of which holds because (p-1) | 560 allows the Fermat exponent to be lifted by periodicity.

## The Miller-Rabin strengthening

Miller-Rabin exploits a property of primes that Fermat does not: in Z/pZ for prime p, the polynomial x^2 - 1 = (x-1)(x+1) has exactly two roots, namely x ≡ 1 and x ≡ -1. This follows because p is prime and therefore must divide one of the two factors (x-1) or (x+1). A composite modulus can have more square roots of 1: for n = 15, the equation x^2 ≡ 1 (mod 15) has four solutions (1, 4, 11, 14), because the factors 3 and 5 of 15 can distribute between (x-1) and (x+1) independently via CRT.

The test writes n-1 = 2^s × d where d is odd and considers the sequence

$$a^d,\ a^{2d},\ a^{4d},\ \ldots,\ a^{2^{s-1} d},\ a^{2^s d} = a^{n-1}$$

obtained by starting at a^d and squaring s times. For a prime p, the final term is 1 by Fermat. Working backwards, the term just before the first occurrence of 1 must be a square root of 1 mod p — and that square root must be ±1. So either the very first term a^d ≡ 1, or somewhere in the sequence we see -1 before reaching 1. A composite can fail both conditions for some base a (yielding a third square root of 1, which is impossible mod a prime), and in that case a is a strong witness to compositeness.

The formal test: given a and n, compute a^d mod n. If it equals 1, report MaybePrime. Otherwise square repeatedly: if any of the s-1 subsequent values equals -1 ≡ n-1, report MaybePrime. If neither condition is met, report IsComposite.

## Probabilistic correctness

A base a that fails to detect a composite is called a strong liar. Rabin proved that for any composite n, at most 1/4 of all bases in {1, ..., n-1} are strong liars. Running the test with k independent random bases gives a false positive probability of at most (1/4)^k. In practice k = 20 suffices for cryptographic use, giving a false positive rate below 10^{-12}.

When n-1 is odd (s = 0), the squaring loop has nothing to do and the test degenerates to pure Fermat. This can only happen when n is even; since the only even prime is 2, the loss of Miller-Rabin's strengthening is harmless in that edge case.

## Complexity

The decomposition of n-1 strips one bit per iteration: O(log n). The squaring loop runs at most s ≤ log₂(n) steps with one modular multiplication each: O(log n). The initial modular exponentiation a^d with d ≤ n-1 costs O(log n). The total cost of a single-base test is therefore O(log n), and running k bases costs O(k log n).

## Worked example

Take n = 561 = 3 × 11 × 17 (a Carmichael number) and base a = 2.

First, decompose n-1 = 560 = 2^4 × 35, giving s = 4 and d = 35.

Compute the sequence starting at 2^35 mod 561, squaring at each step:

$$2^{35} \equiv 263,\quad 2^{70} \equiv 166,\quad 2^{140} \equiv 67,\quad 2^{280} \equiv 1,\quad 2^{560} \equiv 1 \pmod{561}$$

The first term is not 1, and no term equals n-1 = 560. The test returns IsComposite. The key observation is that 2^140 ≡ 67 (mod 561) and 2^280 ≡ 1 (mod 561), yet 67 is neither 1 nor 560. This is a third square root of 1 modulo 561, which is impossible modulo a prime. The factor structure of 561 is what allows it: 67^2 ≡ 1 (mod 3), (mod 11), and (mod 17) hold independently, and CRT combines them into 67^2 ≡ 1 (mod 561) without 67 being ±1 globally.
