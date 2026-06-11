# Extended Euclidean GCD

## Key insight
The regular Euclidean algorithm computes `gcd(a, b)`. The extended version additionally finds integers `x` and `y` such that `ax + by = gcd(a, b)` — Bézout's identity. Such coefficients always exist and can be derived as a byproduct of the same reduction steps.

## Why the coefficients exist (Bézout's identity)
At every step of the Euclidean algorithm, the current remainder can be expressed as a linear combination of the original `a` and `b`. The seeds are trivial: `a = a·1 + b·0` and `b = a·0 + b·1`. Each new remainder `r' = r_prev - q·r` inherits coefficients by substitution:

```
r' = (a·x_prev + b·y_prev) - q·(a·x + b·y)
   = a·(x_prev - q·x) + b·(y_prev - q·y)
```

So `x' = x_prev - q·x` and `y' = y_prev - q·y`. When the algorithm terminates, the last non-zero remainder is the GCD, and it carries the Bézout coefficients.

## State carried through the loop
Six values: the current `(a, b)` pair as in the basic algorithm, plus two coefficient pairs `(xa, ya)` and `(xb, yb)` for each. At termination, `b = 0` so `a` is the GCD; `(xa, ya)` are its Bézout coefficients.

## Seeds
- `a = a·1 + b·0` → `(xa, ya) = (1, 0)`
- `b = a·0 + b·1` → `(xb, yb) = (0, 1)`

## Edge cases
- `gcd(0, 0)` is undefined — returns `None`, same as the basic algorithm.
- `gcd(0, n)` returns `(n, (0, 1))` — the loop never executes, seed values are returned directly.
- `gcd(n, 0)` returns `(n, (1, 0))` — same reason.
- `gcd(n, n)` returns `(n, (0, 1))` — after one iteration, `b` becomes 0.
- Multiple valid coefficient pairs exist (e.g. `gcd(25, 25)` could yield `(1, 0)` or `(0, 1)`); the implementation returns whichever the algorithm naturally produces.

## Overflow handling
Inputs are `u64` but Bézout coefficients are `i64` (they can be negative). The quotient `q = a / b` is computed as `u64` and then converted with `i64::try_from`. The only case where `q` exceeds `i64::MAX` is when `b = 1`, which means `a % b = 0` so `b` becomes 0 after this step. In that case the coefficient update is skipped entirely — the values being computed would be discarded anyway since the loop is about to exit and `(xa, ya)` (not `(xb, yb)`) is returned.

## Loop invariant
At every point in the loop: `a = initial_a · xa + initial_b · ya` and `b = initial_a · xb + initial_b · yb`. The invariant for `b` technically breaks in the final step when `q > i64::MAX` and the coefficient update is skipped — but `b` becomes 0 at that point and `(xb, yb)` are never used, so correctness is unaffected.

## Termination
`b` is a non-negative integer. Each step sets `b_new = a % b`, which is strictly less than `b`. So `b` strictly decreases at every iteration and must reach 0 in finite steps.

## Correctness
The key invariant of the Euclidean algorithm: the set of common divisors of `(a, b)` equals the set of common divisors of `(b, a mod b)`. Both directions hold — if `d` divides `a` and `b`, it divides `a mod b`; if `d` divides `b` and `a mod b`, it divides `a`. So gcd is preserved at every step. When `b = 0`, the common divisors of `(a, 0)` are exactly the divisors of `a`, so `a` is the GCD.

## Complexity
O(log(min(a, b))) — identical to the basic Euclidean algorithm. The coefficient tracking adds only constant work per iteration.

## Relation to modular inverses
Bézout's identity `ax + by = 1` (when `gcd(a, b) = 1`) means `ax ≡ 1 (mod b)` — so `x` is the modular inverse of `a` modulo `b`. The extended Euclidean algorithm is the standard way to compute modular inverses, which are foundational to RSA and other number-theoretic cryptographic schemes.
