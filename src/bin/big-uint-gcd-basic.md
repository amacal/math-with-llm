# BigInteger GCD

## Overview

This session extends the greatest common divisor to arbitrary-precision integers, combining two previously separate pieces of machinery: the `BigNumber` base-2^64 representation and long-division algorithm built in "BigInteger Division", and the reduction identity proved in "Euclidean GCD". The idea is exactly the same as the fixed-width version — repeatedly replace the pair (a, b) with (b, a mod b) until the second element reaches zero — but now a and b are multi-limb vectors rather than single machine words, and "a mod b" means invoking the multi-limb long-division machine rather than a single hardware instruction. Since this is a new, self-contained file under this repo's no-cross-file-reuse rule, the division routine itself is rebuilt here rather than imported, with one change from the original: where "BigInteger Division" only returned the quotient, here the `Div` implementation returns both quotient and remainder, since the remainder is what the Euclidean step actually consumes and it was already being computed internally as a byproduct of finding the quotient digits.

## Choosing Euclidean over binary GCD

Before writing any code, there was a genuine fork to resolve. "BigInteger Division" left this open explicitly: reach `BigInteger GCD` directly via the Euclidean algorithm, which needs a remainder operation not yet exposed, or take a detour through "Binary GCD (Stein's Algorithm)" first, which needs only subtraction, comparison, and bit-shifts — all cheaper primitives than long division. The deciding consideration was what comes after this session, not what is cheapest for this session alone. A natural next step for arbitrary-precision GCD is an extended version that also produces Bézout coefficients x and y satisfying

$$a \cdot x + b \cdot y = \gcd(a, b)$$

the same capability "Extended Euclidean GCD" already provides for machine words, since Bézout coefficients are what a later modular-inverse routine (and eventually BigInteger RSA key generation) will need. The ordinary Euclidean algorithm's recurrence is linear at every step — each remainder is exactly the previous dividend minus an integer multiple of the previous divisor — so the two coefficient sequences update by the same linear combination the remainders do, carrying forward for free once the base recurrence is in place. Stein's algorithm has no equivalent free ride: its halving step only behaves correctly when both operands are already even, so extending it to track Bézout coefficients requires extra machinery to handle the case where a coefficient is odd right when the underlying value needs to be halved. Since the whole reason to reach for `BigInteger GCD` at all is the arc toward modular inverse and RSA, building on the path that extends cleanly was worth more than saving a mod operation now, so the Euclidean route was chosen and the binary-GCD detour set aside.

## Correctness

The correctness argument is the same one proved in full in "Euclidean GCD" — that gcd(a, b) and gcd(b, a mod b) share exactly the same set of common divisors by a two-directions argument, hence the same greatest one — and nothing about that argument depends on how a and b are represented, so it transfers to `BigNumber` unchanged rather than needing to be re-proved. What is new here is confirming termination and the base case still behave correctly once zero is represented as a limb vector rather than a single integer. The loop invariant is that the current pair's gcd always equals the gcd of the original inputs a₀ and b₀:

$$\gcd(a, b) = \gcd(a_0, b_0)$$

and b forms a strictly decreasing sequence of non-negative integers, since a mod b is always strictly less than b whenever b is positive, so by well-ordering the sequence must reach zero after finitely many steps. Two edge cases sit at that boundary. When only b starts at zero, the loop's own condition — while b is not zero — never executes, so a is returned unchanged, which matches the base case

$$\gcd(a, 0) = a$$

with no special-casing required. When both a and b start at zero, the loop condition is equally false, but the value that would fall out of the loop is 0, which is the wrong answer: "Euclidean GCD" establishes that gcd(0, 0) is undefined, since every positive integer divides zero and there is therefore no finite greatest common divisor. This session follows that precedent with an explicit check ahead of the loop that returns `None` for exactly this pair, rather than letting the general mechanism produce a numerically plausible but conventionally wrong answer.

## Complexity

Let n be the limb-count of the larger initial input, matching the convention "BigInteger Division" uses for its own division-cost result, where at any point m is the current divisor's limb-count and n is the current dividend's:

$$O(m \cdot (n - m))$$

Naively, one might multiply that per-step cost by the number of loop iterations. "Euclidean GCD" already establishes, via Lamé's theorem, that the iteration count is the same order as n itself, since a limb-count is by definition the number of digits needed to represent a value in a fixed base, and that count already grows like the logarithm of the value:

$$\text{iterations} = O(\log(\text{value})) = O(n)$$

Multiplying a per-step cost of roughly n² by n iterations would suggest a total cost of

$$O(n^3)$$

but this overcounts badly, because the numbers involved shrink as the algorithm proceeds, and most iterations run on operands far smaller than the original input.

The right way to add up a sequence of shrinking costs is a telescoping sum: a technique for adding up many terms where consecutive terms share a piece that cancels, so that only the very first and very last pieces survive once everything in between has canceled out in pairs. A small illustration makes this concrete. Suppose a sequence of quantities satisfies, for every i,

$$g_i = n_i - n_{i+1}$$

Adding any consecutive run of them, say the first three,

$$g_0 + g_1 + g_2 = (n_0 - n_1) + (n_1 - n_2) + (n_2 - n_3)$$

the middle terms n₁ and n₂ cancel, leaving just

$$g_0 + g_1 + g_2 = n_0 - n_3$$

Extending this all the way through k terms leaves only the first and last:

$$g_0 + g_1 + \cdots + g_{k-1} = n_0 - n_k$$

the sum of many pieces collapses to a single subtraction, regardless of how the total was distributed among the individual steps.

Here nᵢ is the limb-count of a at the start of loop iteration i, so gᵢ measures how much a's size shrinks from one iteration to the next. Two structural facts let this telescoping sum bound the true total cost. First, since the loop always sets the next a to the current b, the current divisor's limb-count mᵢ is exactly the next iteration's dividend limb-count — not merely close to it, but literally the same quantity:

$$m_i = n_{i+1}$$

so the per-step cost can be rewritten using only the n sequence:

$$O(m_i \cdot (n_i - m_i)) = O(n_{i+1} \cdot g_i)$$

Second, since the sizes never grow across the run, nᵢ₊₁ is at most n₀ at every single step, however late in the algorithm that step occurs, so each term is bounded above by

$$O(n_{i+1} \cdot g_i) \le O(n_0 \cdot g_i)$$

Summing that bound over the whole run, using the telescoping identity established above, gives a total cost of

$$O(n_0 \cdot (g_0 + g_1 + \cdots + g_{k-1})) = O(n_0 \cdot (n_0 - n_k))$$

where nₖ is the limb-count once the loop terminates — small, effectively a constant. That total is

$$O(n_0^2)$$

the same order as a single full-size division of the two original inputs, not O(n³). The extra factor of n implied by the iteration count does not multiply the cost, because the telescoping sum shows that however the O(n) shrinkage budget is spent — a few large drops, or many small ones as in the Fibonacci worst case — the total division work stays O(n²).

## Edge cases

`gcd(a, 0)` and `gcd(0, n)` both fall out of the loop's own termination condition without any special-casing, returning the nonzero argument unchanged, exactly as in the fixed-width version. `gcd(0, 0)` is the one case needing an explicit branch, returning `None` to match the convention already established in "Euclidean GCD" that this pair has no finite greatest common divisor.

## Worked example

Consecutive Fibonacci numbers are the classical worst case for the Euclidean algorithm, since every step's quotient is exactly 1, so no step shrinks the pair by more than the minimum possible amount, and reproducing this by hand confirms both the termination argument and the well-known fact that consecutive Fibonacci numbers are always coprime. Take F(7) = 13 and F(6) = 8. The sequence of pairs runs

$$\gcd(13,\ 8) \;\to\; \gcd(8,\ 5) \;\to\; \gcd(5,\ 3) \;\to\; \gcd(3,\ 2) \;\to\; \gcd(2,\ 1) \;\to\; \gcd(1,\ 0)$$

since 13 mod 8 = 5, 8 mod 5 = 3, 5 mod 3 = 2, 3 mod 2 = 1, and 2 mod 1 = 0. The loop terminates once the second element hits 0, returning the first element of that final pair, which is 1 — confirming gcd(13, 8) = 1. Every one of these five steps used quotient 1, the maximally slow case Lamé's theorem bounds, and the session's stress test against F(20000) and F(20001) is the same computation at a scale no longer checkable by hand, terminating after roughly twenty thousand such steps and correctly returning 1 as well.
