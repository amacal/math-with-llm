# Euclidean GCD

## Key insight
If `d` divides both `a` and `b`, it also divides `a - n*b` for any `n` — because subtraction is closed under common divisibility. The remainder `a mod b` is just `a - n*b` for the largest valid `n`, so `gcd(a, b) == gcd(b, a mod b)`.

## Edge cases
- `gcd(0, n)` and `gcd(n, 0)` return `n` — every integer divides 0, so the non-zero input is the greatest.
- `gcd(0, 0)` is undefined — every positive integer divides 0, so there is no greatest. Returns `None`.
- `gcd(a, b) == gcd(b, a)` — when `a < b`, the first iteration computes `a % b == a`, effectively swapping them for free.
- `gcd(n, n) == n`.

## Complexity
O(log(min(a, b))). Worst case is consecutive Fibonacci numbers — they reduce as slowly as possible because each step produces the previous Fibonacci number as the remainder. This is Lamé's theorem (1844), the first ever algorithm complexity analysis.

## Negative numbers
`u64` sidesteps the issue. With `i64`, Rust's `%` truncates toward zero (remainder has the sign of the dividend), so `-12 % 8 == -4`, not `4`. The loop would oscillate. Fix: `abs()` both inputs before entering the loop.
