# Naive Polynomial Multiplication (Linear Convolution)

## Overview

A polynomial is represented as a vector of coefficients, where the coefficient at index `i` is the coefficient of `x^i` — so `[1, 2, 3]` represents `1 + 2x + 3x²`. Multiplying two polynomials means finding the coefficient vector of their product, and the operation that produces this coefficient vector directly from the two input vectors is called **linear convolution**. The intuition is the same as ordinary long multiplication of two numbers: every digit of one number gets multiplied against every digit of the other, and each result lands at a position determined by adding the two digits' place values together — here, the place values are exponents of x instead of powers of ten.

## The convolution formula

The derivation starts from the distributive law. When you multiply two polynomials term by term, every term `a[i] * x^i` from A meets every term `b[j] * x^j` from B, contributing the product `a[i] * b[j]` to the `x^(i+j)` coefficient of the result. Grouping by output degree k, the k-th output coefficient receives contributions from every pair (i, j) with i + j = k. Substituting j = k - i, this gives:

$$c[k] = \sum_{i=\max(0,\, k-(m-1))}^{\min(k,\, n-1)} a[i] \cdot b[k-i]$$

Here n is the length of A and m is the length of B. The bounds on i enforce two simultaneous constraints: i must be a valid index into A (so 0 ≤ i ≤ n-1), and k-i must be a valid index into B (so 0 ≤ k-i ≤ m-1, which gives k-(m-1) ≤ i ≤ k). The lower bound is the tighter of 0 and k-(m-1); the upper bound is the tighter of k and n-1.

## Output length

If A has degree n-1 and B has degree m-1, the leading term of the product is `a[n-1] * b[m-1] * x^(n+m-2)`, since no other pair of terms produces a higher power. The degree of the product is therefore (n-1) + (m-1) = n+m-2, giving n+m-1 coefficients. This is one less than n+m because the count of coefficients is always one more than the degree, and the degrees add rather than the coefficient counts.

## Linear vs. cyclic convolution

The formula above computes **linear** convolution: the output has length n+m-1 and no wrapping occurs. There is also a **cyclic** (or circular) convolution, where the output has length n (or m) and index arithmetic is done modulo n. Cyclic convolution is what the NTT computes natively; recovering linear convolution from it requires zero-padding the inputs to length at least n+m-1. This distinction will matter in the NTT session.

## Correctness

The correctness of the convolution formula rests on an exact accounting of where every product term ends up. Every pair of indices (i, j) with 0 ≤ i < n and 0 ≤ j < m contributes exactly one term, `a[i] * b[j]`, to the coefficient of `x^(i+j)` in the product — this follows directly from the distributive law, since expanding the product of two sums produces exactly one cross term per pair of factors. No pair is missed: every term `a[i] * x^i` from A really does get multiplied against every term `b[j] * x^j` from B once the product is fully expanded. No pair is double-counted either, since each ordered pair (i, j) appears exactly once in the double sum ranging over all i and all j. Grouping together the pairs that land on the same output power k — those with i+j=k — and summing their contributions is therefore both necessary (every pair must be counted somewhere) and sufficient (no pair needs to be counted twice) to produce the true coefficient c[k]. This is exactly why the bounds on i in the formula matter: restricting i to valid indices of A, and k-i to valid indices of B, ensures the sum touches precisely the pairs that genuinely exist, and no others.

## Complexity

The total work is O(n·m). The argument is that each pair (i, j) with 0 ≤ i < n and 0 ≤ j < m is visited exactly once — it contributes to c[i+j] during the iteration where k = i+j. There are n·m such pairs, so the total number of multiplications is exactly n·m. When both inputs have the same length n, this is O(n²). This is the baseline that the Number Theoretic Transform will reduce to O(n log n).

## Edge cases

A constant polynomial — one with a single coefficient, so n=1 or m=1 — is handled by the same formula without any special-casing: the bounds on i collapse to the single value i=0 (when n=1) or i=k (when m=1), and the sum correctly reduces to scalar multiplication of every coefficient of the other polynomial by that one constant. The zero polynomial, represented as a single coefficient equal to 0, needs no special handling either: every product `a[i] * b[j]` that involves the zero coefficient vanishes on its own, so the output coefficient at every position correctly sums to zero, without the algorithm ever needing to check for this case explicitly.

## Worked example

Multiply A = [1, 2, 3] (representing 1 + 2x + 3x²) by B = [4, 5] (representing 4 + 5x). Since n = 3 and m = 2, the output has 3 + 2 - 1 = 4 coefficients, indexed k = 0 through 3, and for each k the bounds on i are max(0, k-1) to min(k, 2).

For k=0, the bounds collapse to the single value i=0, so c[0] = a[0]·b[0] = 1·4 = 4. For k=1, i ranges over 0 and 1, so c[1] = a[0]·b[1] + a[1]·b[0] = 1·5 + 2·4 = 5 + 8 = 13. For k=2, i ranges over 1 and 2, so c[2] = a[1]·b[1] + a[2]·b[0] = 2·5 + 3·4 = 10 + 12 = 22. For k=3, the bounds again collapse to the single value i=2, so c[3] = a[2]·b[1] = 3·5 = 15.

The result is [4, 13, 22, 15], representing 4 + 13x + 22x² + 15x³. This can be verified by expanding (1 + 2x + 3x²)(4 + 5x) directly by hand and collecting like terms, which produces the same four coefficients.
