# Wilson's Theorem as a Primality Test

## Overview

Wilson's Theorem gives a single, exact criterion for primality: a number n greater than one is prime if and only if the factorial of n-1, reduced modulo n, comes out to -1. Written as a congruence,

$$(n-1)! \equiv -1 \pmod{n}$$

holds precisely when n is prime, and fails for every composite n. This sits alongside the two other primality tests already in this repo, Trial Division Factorization and the Miller-Rabin Primality Test, but takes a different route: trial division searches for a witness divisor directly, Miller-Rabin searches probabilistically for a witness to compositeness via modular exponentiation, while Wilson's Theorem multiplies every nonzero residue mod n together at once and reads primality off the result. The interesting part of this session is not the final algorithm, a short loop, but the two-directional proof of why the congruence characterizes primality exactly, and why — despite being exact rather than probabilistic — nobody uses this test in practice.

## Multiplicative inverses and self-inverse elements

For a prime p, every residue a in {1, 2, ..., p-1} has a unique multiplicative inverse in that same set: some b with a times b congruent to 1 mod p. This is easiest to see with p = 7: 2 pairs with 4, since 8 mod 7 is 1; 3 pairs with 5, since 15 mod 7 is 1; and both 1 and 6 pair with themselves, since 1 times 1 is 1, and 6 times 6 is 36, which is 1 mod 7. The two self-paired elements are exactly 1 and p-1 (here 6, which is -1 mod 7), and this is a general fact about every prime, not a coincidence of p = 7 — it is the entire engine behind Wilson's Theorem.

To see why only 1 and p-1 can be self-inverse for any prime p, suppose some residue a satisfies a times a congruent to 1 mod p. This means p divides a^2 - 1, and a^2 - 1 factors as (a-1)(a+1), so

$$p \mid (a-1)(a+1).$$

Because p is prime, Euclid's Lemma applies: if a prime divides a product of two integers, it must divide at least one of them. So either p divides (a-1), giving a congruent to 1 mod p, or p divides (a+1), giving a congruent to -1 (that is, p-1) mod p. No other residue can be self-inverse.

One gap remains: could some x strictly between 2 and p-2 have its inverse land on 1 or p-1? Both cases contradict themselves. If x's inverse were 1, then x times 1 would have to equal 1 mod p, forcing x congruent to 1 — impossible, since x lies strictly between 2 and p-2. If x's inverse were p-1, then x times (p-1) would have to equal 1 mod p; since p-1 is its own inverse, multiplying both sides by p-1 again gives x congruent to p-1 — again impossible. So for every x in {2, ..., p-2}, the inverse of x also lies in {2, ..., p-2} and is never equal to x itself, since only 1 and p-1 are self-inverse. The inverse map therefore pairs up all of {2, ..., p-2} into disjoint pairs, each multiplying to 1 mod p.

## The forward and converse directions

Putting the pairing together with the two leftover elements gives one half of the proof. The product of all nonzero residues, 1 times 2 times ... times (p-1), regroups as 1, times the paired-up elements from {2, ..., p-2} (each pair contributing a factor of 1), times p-1. Every paired factor collapses to 1, leaving only 1 times (p-1), which is p-1 itself. So

$$(p-1)! \equiv p - 1 \equiv -1 \pmod{p}$$

for every prime p.

The other direction asks what happens when n is composite. Write n as d times e, with both d and e strictly between 1 and n. If d and e are distinct, they appear as two separate factors somewhere in the product P = 1 times 2 times ... times (n-1), so P can be written as d times e times (everything else), which is n times (everything else). This means n divides P outright, so P is congruent to 0 mod n — never -1 mod n once n is bigger than 1. So no composite n with two distinct factors can satisfy Wilson's congruence. The one case not covered, n equal to a prime squared, is treated under Edge cases below.

## Correctness

The full statement is an if-and-only-if: (n-1)! is congruent to -1 mod n exactly when n is prime. The forward direction is the inverse-pairing argument above; the converse is the factor argument above, that composite n with two distinct factors forces the product to be congruent to 0, never -1 mod n once n exceeds 1. Since the decision rule is exactly whether the computed product is congruent to -1 mod n, and both directions hold (n equal to a prime squared handled separately in Edge cases), the algorithm decides primality correctly for every n greater than 1. A companion fact justifies an optimization rather than correctness itself: the running partial product 1 times 2 times ... times k can never become congruent to 0 mod p for k less than a prime p, since none of those factors can be divisible by p, and Euclid's Lemma says p dividing their product would force p to divide one of them. Hitting a 0 partway through therefore only happens when n is composite, so the function can return composite the moment the accumulator hits 0, without finishing the remaining multiplications.

## Complexity

Computing (n-1)! mod n by iterated multiply-then-reduce takes n-2 multiplications in the worst case, each followed by a reduction mod n — so the algorithm runs in O(n) time and O(1) space, a single running accumulator regardless of n's size. As in Modular Exponentiation (`src/bin/mod-exponent.md`), each intermediate product can be reduced mod n at every step without changing the final answer, using the modular-reduction identity established there; this keeps the accumulator bounded below n, so the running product, stored as a u128 while n is a u64, never overflows, since the accumulator and the next factor are both strictly less than n, making their product strictly less than n squared, always smaller than 2^128 - 1 for any u64 value of n. This O(n) cost is what makes Wilson's Theorem impractical despite being exact: Trial Division Factorization costs O(sqrt(n)), and Miller-Rabin costs O(k log n) for k witnesses. For n around 10^18, Wilson's Theorem would need on the order of 10^18 multiplications, while Miller-Rabin with twenty witnesses needs only a few hundred — a clean mathematical characterization of primality with an enormous practical cost.

## Edge cases

The case n = 4 deserves its own treatment because it is the one place the general composite argument does not directly apply. Writing 4 as 2 times 2 offers only one copy of the factor 2, not two distinct factors, so the "n divides P" argument used for other composites does not go through as stated. Direct computation settles it instead: 3! is 6, which is 2 mod 4, and 2 is not congruent to -1 mod 4 (which is 3), so the test still correctly reports composite. This works because the decision rule only checks non-equality with -1, a strictly weaker requirement than equality with 0, so the general divisibility argument can fail while the numerical outcome for n = 4 still satisfies what the test relies on. The values n = 0 and n = 1 are handled by an explicit guard before the loop, since factorial and modulus are not meaningfully defined for primality there, and both are treated as composite by convention.

## Worked example

Take p = 7. The nonzero residues are 1, 2, 3, 4, 5, and 6. Their multiplicative inverses pair up as follows: 1 is its own inverse, since 1 times 1 is 1; 2 pairs with 4, since 2 times 4 is 8, which is 1 mod 7; 3 pairs with 5, since 3 times 5 is 15, which is 1 mod 7; and 6 is its own inverse, since 6 times 6 is 36, which is 1 mod 7. The two self-inverse elements, 1 and 6, are exactly 1 and p-1, matching the general proof above. Grouping the full product 1 times 2 times 3 times 4 times 5 times 6 by these pairs, (2, 4) contributes a factor of 1, and (3, 5) contributes a factor of 1, leaving only the product of the two self-inverse leftovers, 1 times 6, equal to 6. Directly, 1 times 2 times 3 times 4 times 5 times 6 equals 720, and 720 divided by 7 is 102 remainder 6, so 720 mod 7 is 6, which is -1 mod 7. This confirms Wilson's Theorem for p = 7 by both the pairing argument and direct computation, small enough to redo by hand as a self-check.
