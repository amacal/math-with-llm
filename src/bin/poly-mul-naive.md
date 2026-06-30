# Naive Polynomial Multiplication (Linear Convolution)

## What the problem is

A polynomial is represented as a vector of coefficients, where the coefficient at index `i` is the coefficient of `x^i`. So `[1, 2, 3]` represents `1 + 2x + 3x²`. Multiplying two polynomials means finding the coefficient vector of their product. The operation that produces this coefficient vector from the two input vectors is called **linear convolution**.

## The convolution formula

The derivation starts from the distributive law. When you multiply two polynomials term by term, every term `a[i] * x^i` from A meets every term `b[j] * x^j` from B, contributing the product `a[i] * b[j]` to the `x^(i+j)` coefficient of the result. Grouping by output degree k, the k-th output coefficient receives contributions from every pair (i, j) with i + j = k. Substituting j = k - i, this gives:

$$c[k] = \sum_{i=\max(0,\, k-(m-1))}^{\min(k,\, n-1)} a[i] \cdot b[k-i]$$

Here n is the length of A and m is the length of B. The bounds on i enforce two simultaneous constraints: i must be a valid index into A (so 0 ≤ i ≤ n-1), and k-i must be a valid index into B (so 0 ≤ k-i ≤ m-1, which gives k-(m-1) ≤ i ≤ k). The lower bound is the tighter of 0 and k-(m-1); the upper bound is the tighter of k and n-1.

## Output length

If A has degree n-1 and B has degree m-1, the leading term of the product is `a[n-1] * b[m-1] * x^(n+m-2)`, since no other pair of terms produces a higher power. The degree of the product is therefore (n-1) + (m-1) = n+m-2, giving n+m-1 coefficients. This is one less than n+m because the count of coefficients is always one more than the degree, and the degrees add rather than the coefficient counts.

## Complexity

The total work is O(n·m). The argument is that each pair (i, j) with 0 ≤ i < n and 0 ≤ j < m is visited exactly once — it contributes to c[i+j] during the iteration where k = i+j. There are n·m such pairs, so the total number of multiplications is exactly n·m. When both inputs have the same length n, this is O(n²). This is the baseline that the Number Theoretic Transform will reduce to O(n log n).

## Linear vs. cyclic convolution

The formula above computes **linear** convolution: the output has length n+m-1 and no wrapping occurs. There is also a **cyclic** (or circular) convolution, where the output has length n (or m) and index arithmetic is done modulo n. Cyclic convolution is what the NTT computes natively; recovering linear convolution from it requires zero-padding the inputs to length at least n+m-1. This distinction will matter in the NTT session.

## Worked example

Multiply A = [1, 2, 3] (representing 1 + 2x + 3x²) by B = [4, 5] (representing 4 + 5x).

Output length: 3 + 2 - 1 = 4 coefficients.

For each k, the bounds are max(0, k-1) to min(k, 2):

- k=0: i from 0 to 0. c[0] = a[0]·b[0] = 1·4 = **4**
- k=1: i from 0 to 1. c[1] = a[0]·b[1] + a[1]·b[0] = 1·5 + 2·4 = 5 + 8 = **13**
- k=2: i from 1 to 2. c[2] = a[1]·b[1] + a[2]·b[0] = 2·5 + 3·4 = 10 + 12 = **22**
- k=3: i from 2 to 2. c[3] = a[2]·b[1] = 3·5 = **15**

Result: [4, 13, 22, 15], representing 4 + 13x + 22x² + 15x³. Verify by expanding (1 + 2x + 3x²)(4 + 5x) by hand — collecting like terms gives the same answer.
