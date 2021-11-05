# Complete Search



Complete search is the process of recursively searching all possible solutions.
This will always yield an answer, however it may be inefficient.
In that case we would want to use a greedy approach or dynamic programming.

## Generating Subsets

A classic problem is generating subsets of a set.

The simplest way is to use recursion:
```rust
fn search(k: usize) {
    if k==n {
        do_something();
    } else {
        search(k+1);
        stack.push(k);
        search(k-1);
        stack.pop();
    }
}
```
although since we cannot have global mutable variables between function calls so we need to modify this a bit:
```rust
let mut k = 0;
loop {
    if k==n {
        do_something();
        break
    } else {
        k+1;
	stack.push(k);
.....
    }
}
```


## Generating Permutations

blah blah blah


# BackTracking

In a backtracking algorithm you start with an empty solution,
and build your way up constructing the next solution.

## Pruning

Pruning can be optimized by pruning the search tree.
Essentially we can remove any solutions that are no longer possible given the current solution.



