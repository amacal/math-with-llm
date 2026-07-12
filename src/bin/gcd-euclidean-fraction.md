# Continued-Fraction Term Generation via the Euclidean Algorithm

## Overview

This session turns a fact already proved into a genuinely working piece of code. The earlier Continued Fractions session (`gcd-euclidean-sqrt.md`) established that the sequence of terms in the continued-fraction expansion of a/b is exactly the sequence of quotients produced by running the Euclidean algorithm on the pair (a, b), yet that session stopped short of writing the algorithm itself, testing rational inputs with hand-computed term lists instead. The function `decompose` implemented here closes that gap directly, running the same division-with-remainder loop as `gcd-euclidean-basic.rs`'s gcd computation but collecting the quotient at every step instead of discarding all but the last one. Because the load-bearing theory — why this sequence of quotients equals the true continued fraction, and why the process terminates — was already proved in that earlier session and is only cited here rather than re-derived, this file runs shorter than a typical single-concept session; the genuinely new material is the algorithm itself and a small set of design questions about how it should behave at the edges of its input domain.

## From a proven identity to a loop

The earlier session showed that extracting one term from a/b and moving to the next fraction in the tower obeys the identity

$$\frac{a}{b} = q + \frac{1}{b/r} \qquad \text{where } a = b \cdot q + r$$

and that b/r is exactly the ratio the process is asked to expand next, since it is built from the same pair of remainders the Euclidean algorithm produces at the following step. `decompose` turns this into a loop by keeping a running pair (a, b), computing q and r from that pair, appending q to a result vector, and then replacing (a, b) with (b, r) to continue. The one subtlety in this update is that both new values must be computed from the same old pair rather than in sequence: if b were overwritten with r before a was updated, the value needed to become the new a would already be gone, so the implementation copies the old b into a temporary variable before either assignment happens, exactly mirroring the simultaneous-update discipline already used for the Euclidean algorithm itself.

## Why the input domain is not symmetric

A natural instinct, coming from `gcd-euclidean-basic.rs`, is to expect this function to treat its two arguments somewhat symmetrically the way gcd(a, b) equals gcd(b, a). That instinct is wrong here, and worth spelling out because gcd's symmetry makes it easy to conflate the two: gcd measures something that genuinely does not depend on argument order, but `decompose(a, b)` computes the continued fraction of the ratio a/b, and a/b is not equal to b/a in general, so there is no reason to expect this function to behave the same way under a swap. This settles which zero input is actually invalid. The value b = 0 makes a/b a literal division by zero and must be rejected, so the guard checks only b before the loop ever runs. By contrast a = 0 is a completely ordinary input: 0/b is a well-defined rational number equal to 0, and running the loop on (0, b) with b greater than 0 gives the single equation

$$0 = 0 \cdot b + 0$$

which pushes a single term 0 and then stops immediately, since the new remainder is already 0. A second, related worry — that a less than b might need some correction analogous to a hypothetical gcd-style swap — also turns out to be unfounded: the same loop, run on a pair where a is smaller than b, simply produces a first quotient of 0 and continues from there with no special-casing, because the division equation a = b times q plus r does not care which of a and b is larger.

## Correctness

Correctness here rests entirely on the fact proved in the earlier session: unwinding the substitution a/b = q + 1/(b/r) repeatedly, term by term, all the way down to the point where some remainder hits zero, produces exactly the nested expression

$$\frac{a}{b} = q_0 + \cfrac{1}{q_1 + \cfrac{1}{q_2 + \cfrac{1}{\ddots}}}$$

which is the definition of the continued fraction of a/b, so the vector of q's that `decompose` collects is, term for term, that expansion rather than merely some sequence of quotients that happens to look similar. Termination is the same argument already given for the Euclidean algorithm in `gcd-euclidean-basic.md`: the remainder strictly decreases at every step and is bounded below by zero, so by the well-ordering principle the loop cannot run forever, and it stops the moment a remainder of exactly zero is produced.

## Complexity

Both the number of iterations and the arithmetic performed per iteration are unchanged from plain Euclidean GCD, so the time complexity is the same bound already established via Lame's theorem in `gcd-euclidean-basic.md`,

$$O(\log(\min(a, b)))$$

which means the number of division steps grows only logarithmically as the smaller of a and b grows, so even inputs near u64::MAX finish in on the order of dozens of iterations rather than anything close to their magnitude. Space is the one place this function genuinely costs more than gcd: where the Euclidean algorithm only ever needs a constant number of registers, `decompose` must retain every quotient it produces rather than discard all but the last, so its space usage is also O(log(min(a, b))), proportional to the number of terms in the expansion rather than constant.

## Edge cases

Calling `decompose` with b = 0 returns `None`, since a/0 is not a number and the loop never gets a chance to run. Calling it with a = 0 and b greater than 0 is valid and returns the single-term vector [0], as shown above. Calling it with a less than b is also valid without any special handling, producing a leading term of 0 followed by the expansion of the reciprocal b/a; no swap or reordering of the inputs is ever needed, unlike the intuition borrowed from gcd.

## Worked example

Trace `decompose(17, 7)` directly against its three loop iterations. The first iteration divides 17 by 7, giving 17 = 2 times 7 plus 3, so 2 is pushed onto the result and the state becomes the pair (7, 3). The second iteration divides 7 by 3, giving 7 = 2 times 3 plus 1, pushing another 2 and moving the state to (3, 1). The third iteration divides 3 by 1, giving 3 = 3 times 1 plus 0, pushing 3 and moving the state to (1, 0); since the new b is 0, the loop exits here. The result is the vector [2, 2, 3], matching the hand-derived expansion of 17/7 from the earlier session, and feeding this same vector through `convergents` — unchanged from that session, since it depends only on the term list and not on how the terms were produced — reconstructs 2/1, 5/2, and finally 17/7 exactly, confirming that the algorithmically generated terms are interchangeable with the hand-computed ones used before.
