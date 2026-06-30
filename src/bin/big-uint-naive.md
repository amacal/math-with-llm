# Big Integer Arithmetic (Schoolbook)

## Representation

A `BigNumber` is a `Vec<u64>` of base-2^64 digits stored least-significant chunk first. The chunk at index `i` contributes the value `d_i * (2^64)^i` to the number. This layout is natural because arithmetic proceeds from the least significant position upward, and pushing to the end of a `Vec` then corresponds to appending toward the most significant end.

The invariant is that there are no trailing zeros — the last element of `data` is never zero — except for the number zero itself, which is represented as exactly `vec![0]`. This invariant gives every number a unique representation, which makes equality a simple element-by-element comparison and avoids any normalisation logic in `PartialEq`.

## Comparison

Comparison exploits the unique representation invariant directly. The longer `Vec` holds the larger number, because any chunk present in the longer number and absent in the shorter is implicitly zero, and the longer number therefore has a nonzero contribution at a higher position. When lengths are equal, the chunks are compared from most significant to least significant — the first differing chunk decides the order. Rust's `Ord` trait carries the real logic; `PartialOrd` simply delegates to it via `Some(self.cmp(other))`. The `Eq` marker trait signals that equality is total — every pair of values is comparable, with no `NaN`-like incomparable cases.

## Addition

Addition mirrors the schoolbook algorithm in base 2^64. Chunks are processed from index 0 upward. At each position the two corresponding chunks (treating out-of-bounds as zero) and the incoming carry are added. Two `overflowing_add` calls suffice: the first adds the two chunks, the second adds the carry. The outgoing carry is the sum of the two overflow flags, which is always 0 or 1 — both overflows cannot fire simultaneously because if the first addition overflowed, the result wrapped to a small value that cannot overflow again when 0 or 1 is added. After the main loop a final carry, if nonzero, becomes a new most-significant chunk.

## Subtraction

Subtraction mirrors addition but uses `overflowing_sub` and borrows instead of carries. A borrow propagates when a chunk is too small to absorb the subtrahend plus the incoming borrow. The constraint `lhs >= rhs` must hold; a nonzero borrow after the loop signals violation and returns `None`. Because subtraction can reduce the most significant chunks to zero, trailing zeros must be stripped after the loop to restore the invariant. The strip stops when only one chunk remains, preserving the `vec![0]` representation of zero.

## mul128

Multiplying two `u64` values yields a result up to `(2^64 - 1)^2`, which requires 128 bits. Rather than using `u128`, the computation splits each input into two 32-bit halves:

$$a \cdot b = (a_h \cdot 2^{32} + a_l)(b_h \cdot 2^{32} + b_l) = a_h b_h \cdot 2^{64} + (a_h b_l + a_l b_h) \cdot 2^{32} + a_l b_l$$

Each of the four products fits in a `u64` since it is a product of two 32-bit values. The result is assembled into a `(lo, hi)` pair of `u64`. The two middle terms (`a_h b_l` and `a_l b_h`) each split across the boundary: their lower 32 bits (shifted left by 32) contribute to `lo`, and their upper 32 bits contribute to `hi`. The carries from adding the middle terms into `lo` are absorbed into `hi`. This is safe without overflow checking because `hi` starts at most `(2^32 - 1)^2 = 2^64 - 2^33 + 1`, and the maximum additional contribution from both middle terms' upper halves plus two carry bits is `2*(2^32 - 1) + 2 = 2^33 - 0`, which combined stays within `u64::MAX`.

## Schoolbook Multiplication

The correctness of schoolbook multiplication follows from the distributive law. Writing

$$a = \sum_i a_i \cdot B^i \qquad b = \sum_j b_j \cdot B^j$$

where B = 2^64, the product expands as

$$a \cdot b = \sum_{i,j} a_i \cdot b_j \cdot B^{i+j}$$

The double loop computes exactly this: each `(i, j)` pair contributes `a_i * b_j` (via `mul128`, which returns a `(lo, hi)` pair) deposited at positions `i+j` and `i+j+1`. The result array has size `n + m` because the maximum product of an `n`-chunk and `m`-chunk number satisfies `(2^(64n) - 1)(2^(64m) - 1) < 2^(64(n+m))`, so `n + m` chunks always suffice.

The carry management uses a deferred approach. Adding `lo` into `data[i+j]` can overflow, generating a carry that is absorbed into `high` (which always has room: `high` from `mul128` is at most `0xfffffffffffffffe`, so adding 1 gives at most `0xffffffffffffffff`). Adding the adjusted `high` into `data[i+j+1]` can then overflow, generating a carry that cannot be safely chased immediately because multiple `(i,j)` pairs write to the same position. Instead, these carries are accumulated in a separate `carries` array by incrementing `carries[i+j+2]` — a count, not a sum, so it never overflows (`u64` can hold up to `min(n,m)` increments per cell, far below `u64::MAX`). After the double loop, a single linear pass merges `carries` into `data`, propagating any resulting carries forward. Complexity is O(n*m).

## Worked example

Multiply `a = vec![3, 1]` (representing 3 + 1·2^64) by `b = vec![2]` (representing 2). Expected result: `vec![6, 2]` (representing 6 + 2·2^64).

Only one pair: `(i=0, j=0)` and `(i=1, j=0)`.

Pair `(0, 0)`: `mul128(3, 2) = (6, 0)`. Add 6 to `data[0]`: `data[0] = 6`, no carry. Add 0 to `data[1]`: `data[1] = 0`, no carry. `carries[2] += 0`.

Pair `(1, 0)`: `mul128(1, 2) = (2, 0)`. Add 2 to `data[1]`: `data[1] = 2`, no carry. Add 0 to `data[2]`: `data[2] = 0`, no carry. `carries[3] += 0`.

After double loop: `data = [6, 2, 0, 0]`, `carries = [0, 0, 0, 0, 0]`. Merge pass changes nothing. Strip trailing zeros: `data = [6, 2]`. Result: `vec![6, 2]`. Correct.
