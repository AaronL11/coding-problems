# Dynamic Programming



Dynamic programming is about breaking a problem into subproblems. Then using the solutions of those subproblems
to generate a find an optimal solution.

It is a lot like greedy algorithms, except you remember the solutions of the previous subproblems.
It is a lot like tree pruning on a complete search.

## Kadane's algorithm

A classic problem in competitive programming is called the **Minimum Sum Subarray** problem.
This problem has a well known solution, that being Kadane's algorithm.

This section is a brief intro into kadane's as well as a simple Rust implementation.

```rust
use std::cmp::min;

let (mut sum,mut ans) = (0,0);
for i in 0..n {
    sum += A[i];
    ans = max(ans,sum);
    if sum < 0 {
        sum = 0;
    }
}
ans
```

```rust

let n = A.len();
let mut lmax = 0;
let mut gmax = 0;
for a in A {
    lmax = max(a,a+lmax);
    if lmax > gmax {
        gmax = lmax;
    }
}
gmax
```

