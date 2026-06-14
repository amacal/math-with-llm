# Euclidean GCD

## Key insight

The algorithm rests on a single observation about divisibility. If some number d divides both a and b, then it also divides anything you can build by adding and subtracting multiples of a and b. In particular, it divides a minus any whole multiple of b. The remainder when you divide a by b is exactly that — a with as many copies of b removed as possible while staying non-negative. So a common divisor of a and b must also divide the remainder, and by the same argument in reverse, a common divisor of b and the remainder must divide a. The two pairs share exactly the same common divisors, so they share the same greatest one.

$$\gcd(a,\ b) = \gcd(b,\ a \bmod b)$$

## Proof of the core identity

We want to show that gcd(a, b) and gcd(b, a mod b) are equal. The strategy is to show that the two pairs have identical sets of common divisors. Since the sets are equal, their maxima — the greatest common divisors — must be equal too.

The starting point is understanding what the remainder actually is. When you divide a by b, integer division gives you a quotient and a remainder. The quotient is as large as possible while keeping the remainder non-negative:

$$r = a \bmod b \qquad a = n \cdot b + r$$

where n is the integer part of a divided by b, and r is what is left over. Rearranging gives:

$$r = a \bmod b \qquad r = a - n \cdot b$$

This is the only fact about r we will use: it is a linear combination of a and b, specifically a minus n copies of b.

To show that two sets are equal, it is not enough to show that one is contained in the other — you need both directions. If every element of set A is also in set B, and every element of set B is also in set A, then the sets must be identical. This is how equality of sets is always proved in mathematics, and it is the structure we follow here: we show that every common divisor of (a, b) is a common divisor of (b, r), and then show the reverse.

**Direction 1: any common divisor of (a, b) also divides r.**

Suppose d divides both a and b. Then a and b can each be written as d times some integer:

$$a = d \cdot p \qquad b = d \cdot q \qquad r = a - n \cdot b$$

Substituting into the expression for r, every term picks up a factor of d:

$$r = a - n \cdot b = d \cdot p - n \cdot d \cdot q = d \cdot (p - n \cdot q)$$

Since p, n, and q are all integers, so is their combination, and d divides r. Because d already divided b, it is now a common divisor of both b and r.

**Direction 2: any common divisor of (b, r) also divides a.**

Now go the other way. Suppose d divides both b and r. The key observation is that a is not independent of b and r — it is assembled from them by the equation we started with. Write:

$$b = d \cdot q \qquad r = d \cdot s \qquad a = n \cdot b + r$$

Then substituting b and r into a:

$$a = n \cdot d \cdot q + d \cdot s = d \cdot (n \cdot q + s)$$

Again, the combination in parentheses is an integer, so d divides a. Any common divisor of (b, r) is therefore also a common divisor of (a, b).

**Conclusion.** Both directions together tell us that the set of common divisors of (a, b) and the set of common divisors of (b, r) contain exactly the same elements. Two sets with identical elements have the same maximum, so:

$$\gcd(a,\ b) = \gcd(b,\ a \bmod b)$$

## Termination and base case

The algorithm applies the identity repeatedly, each time replacing the pair (a, b) with (b, r). For this process to be useful it must eventually stop. The reason it does is that the second argument strictly shrinks at every step. The remainder r is always less than b — that is built into the definition of remainder, since if r were as large as b you could subtract one more copy and reduce further. So we have a strictly decreasing sequence of non-negative integers, and any such sequence must reach zero in a finite number of steps.

When the second argument reaches zero, we have the pair (a, 0) for some value of a. The greatest common divisor of a and 0 is a itself, because every integer divides zero — for any d, the equation 0 = d · 0 holds — so the common divisors of (a, 0) are simply all the divisors of a, and the largest of those is a:

$$\gcd(a,\ 0) = a$$

## Complexity proof

To bound the number of steps, we look at two consecutive steps at a time rather than one. Suppose the current pair is (b, r1) and after one step it becomes (r1, r2). We want to prove that:

$$r_2 < b/2$$

meaning every two steps the value at least halves. We know two facts going in: the remainder is always less than the divisor, so:

$$r_1 < b \qquad \text{and} \qquad r_2 < r_1$$

The argument splits into two cases based on where r1 falls relative to b/2.

**Case 1**

$$r_1 \leq b/2$$

A remainder is always strictly less than the divisor it was computed from, so r2 < r1. Since r1 is already below b/2, transitivity gives us what we want:

$$r_2 < r_1 \leq b/2$$

**Case 2**

$$r_1 > b/2$$

When r1 exceeds half of b, there is only room for exactly one copy of r1 inside b, so the quotient q is 1 and the remainder is what is left after removing that one copy:

$$r_2 = b \bmod r_1 = b - q \cdot r_1$$
$$r_2 = b - r_1$$

Since r1 is greater than b/2, removing it from b leaves strictly less than b/2:

$$r_2 = b - r_1 < b - b/2 = b/2$$

The boundary case r1 = b/2 (only possible when b is even) gives quotient 2 and r2 = 0, which is also less than b/2, so it causes no trouble.

In both cases r2 is below b/2, so every two steps the working value at least halves. Starting from b, after k pairs of steps the remainder has fallen below:

$$b / 2^k$$

This reaches zero once 2 to the power k exceeds b, which happens after at most:

$$O(\log b) \text{ steps}$$

This result is known as Lamé's theorem, published in 1844 — it was the first rigorous analysis of the complexity of any algorithm.

## Edge cases

When one of the inputs is zero the answer is immediate. gcd(0, n) and gcd(n, 0) both return n, because every integer divides zero, making n the largest common divisor of the pair. The case gcd(0, 0) is undefined: every positive integer divides zero, so there is no finite greatest common divisor, and the implementation returns None.

When a is less than b, the first step computes a mod b, which equals a itself since a fits inside b without any subtraction. The pair becomes (b, a), so the inputs are swapped for free without any special handling.

## Worked example

Tracing gcd(48, 18) through each step:

$$\gcd(48,\ 18) \;\to\; \gcd(18,\ 12) \;\to\; \gcd(12,\ 6) \;\to\; \gcd(6,\ 0) \;\to\; 6$$

At each step the remainder is 48 mod 18 = 12, then 18 mod 12 = 6, then 12 mod 6 = 0. The answer is 6. To verify: 48 = 8 × 6 and 18 = 3 × 6, and since 8 and 3 share no common factor, no divisor larger than 6 can divide both.

## Negative numbers

The implementation uses unsigned 64-bit integers, which sidesteps the issue entirely. With signed integers, Rust's remainder operator truncates toward zero, so -12 % 8 produces -4 rather than 4. A negative remainder would cause the second argument to increase on that step, and the algorithm would fail to converge. The fix is straightforward: take the absolute value of both inputs before the loop begins.
