\newpage

# Greedy Algorithms


Let's start with a simple problem. What is the fewest number of coins
to give someone \$3.46 in change using Canadian dollars?
(2, 1, 0.25, 0.10, 0.05, 0.01)

This is a pretty straight forward problem, just subtract by the largest
coin and keep going until we reach zero:
(2, 1, 0.25, 0.10, 0.10, 0.01)


What about the highest number of coins? Well that is simply 346 pennies.

The process for solving a problem like this is called a **Greedy Algorithm**.

## What is a Greedy Algorithm?

There is no exact answer for what classifies a particular algorithm as greedy.
It is more like a family of algorithms and techniques.
Creating a greedy algorithm is essentially following certain guidelines
while solving your problem.

Sometimes algorithms may not be called greedy when they are. For example:
* Djikstra's
* Prim's
* Khruskal's

A greedy alorithm is defined by a few characteristics:
1. Builds a solution in small steps
2. Decision at each step only optimizes for immediate situation
3. Never go back to reconsider past choices
  * Unlike Dynamic Programming

A greedy algorithm can use one or many simple or complicated rules.

At a theoretical level, a greedy algorithm is simply an algorithm that
divides a problem into various subproblems, each giving an optimal solution.

When we put this in terms of our original coin problem we can break this down into a subproblem and an iteration:
1. Pick the highest value coin that's smaller than the remaining total
2. Repeat until you have 3 dollars and 46 cents, or our remaining total is zero.

The subproblem doesn't care about minimizing our coins, it only wants to find the largest coin that fits.
Only after we finish iterating do we find the minimum number of coins.

***This may not be obvious for other problems***

Here is what our solution would look like in Rust:
```rust
use std::cmp::max;

fn coins(amount: &f64) -> Vec<f64> {
    let dollars = [2.00,1.00,0.25,0.10,0.05,0.01];
    let mut coins: Vec<f64> = Vec::new();
    let mut total = *amount;
    while total {
    	coin = 
    	total -= coin
    }
    coins
}


```


## Interval Scheduling

A classic greedy algorithm problem is **interval scheduling**.

**Problem:** We have a single room and many people want to book it at different times.
We want to allow the maximum number of people to book it. How do we decide?

We say that two requests are compatible if they do not overlap.
A set of requests is compatible if all requests in that set are compatible with each other.

Essentially this question is asking about the largest number of request we can create without overlap.


To figure out how to solve this problem we need to come up with
possible solutions, and find counterexamples to them.
Let's analyze the following ideas:
1. Shortest start times
2. The one that starts first
3. Pick the ones with fewest overlaps
TODO:

The best solution is to pick the start time that finishes the earliest,
then remove any overlapping request. We then repeat until we run out of time.


```rust
type Times = Vec<(u64,u64)>
let times: Times;
let mut answer: Times;

let mut prev = 0;
for (s,e) in &times {
    if s >= prev {
        answer.push(s,e);
        prev = e;
    }
}
answer
```












