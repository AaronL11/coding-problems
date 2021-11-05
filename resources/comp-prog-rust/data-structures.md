# Data Structures in Rust



A lot of helpful data structures can be accessed via the [collections](https://doc.rust-lang.org/std/collections/) module in the standard library.

Here is an overview of how to use various data structures in rust.

## Linear Data Structures

Rust provides a nice variety of linear data structures within it.

### Dynamic Arrays

If you're already familiar with programming in Rust you are likely
aware of the [`Vec<_>`](https://doc.rust-lang.org/std/vec/index.html) struct.
This is essentially our dynamic array in Rust and it is very efficient.

### Stack

Stacks are implemented through `Vec<_>` via the `push()` and `pop()` methods.
In order to use it as a stack we must declare it as mutable with `mut`:
```rust
let mut stack = Vec::new();

stack.push(item);

stack.pop();
```

### Queues

Rust comes bundled with a queue data structure via the [`Vecdeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html).
By degault this is a double ended queue (dequeue).
To use it we simply grab it with a use statement and call the relevant methods:
```rust
use std::collections::VecDeque;

let queue = VecDeque::new();

queue.push_back(item);
queue.pop_front();
```

### Priority Queues

A priority queue can be implemented with a heap.
Luckily Rust comes with a [`BinaryHeap`](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html) data structure.
The default implementation is a max heap but for a min heap simply put the negative values.
```rust
use std::collections::BinaryHeap;

let heapq = BinaryHeap::new();

heapq.push(1);
heapq.push(2)
heapq.pop()
// returns max value `2`
```

### Linked Lists

Linked Lists are [historically annoying](https://rust-unofficial.github.io/too-many-lists/) to implement in rust.
Luckily the standard library has our back by providing a simple doubly ended linked list implementation.
For those who don't know, Rust's `Iterators` are already linked lists.

### Honourable Mention `binary_search()`

We can perform a binary search on a slice using the `binary_search` method.
Since we can form a slice from a `Vec<T>` this method is available on it as well:
```rust
let v = vec![1,25,50,75,100];

v.binary_search(34);
```
please refer to the [documentation](https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search) for more information on how to use it.
There are also similar methods that allow you to perform a binary search by a certain value.

## Nonlinear Data Structures

### Sets

### Maps

For maps we have two implementations [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) (similar to C++'s `map<K,V>`)
and [`HashMap`](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html) (C++'s `unordered_map<K,V>` or python's `dict`).

A `BTreeMap` is short for a balanced binary tree, while a `HashMap` uses hashing to map values.

## Rust Specific

[`Iterators`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
aren't specific to rust, however idiomatic rust is incomplete without them. If you are going to program efficiently with rust it is best to avoid for loops in favor of iterators.

To do this you need to be familiar with the `Iterator` trait and it's various implementations and methods.

### Lambda expressions

Lambda expressions are anonymous functions which take an input and return an output.
In Rust, lambda's are called closures and have the following syntax:
```rust
|arg1,arg2| do_something()

|arg1,arg2| {
    do_something();
    do_another_thing()
}
```
notice the actual amount of arguments in our closure can vary depending on the context it is called.
See the [relevant section](https://doc.rust-lang.org/stable/book/ch13-01-closures.html)
in the Rust book for more detail about closures.

### Map

Quite simply the most useful method on iterators is `map()`.
This method takes the current item in the iterator and transforms it (without changing the original data):
```rust
for i in sometype {
  do_something(i)
}

// equivalent to

sometype.iter()
        .map(|i| do_something(i))
```
notice we could also call a function like as `map(do_something)`, and a method as `map(Thing::do_something)`.

### Filter

The `filter` method takes in a closure that returns a boolean.
It pretty much does what it says by filtering the types of data.
```rust
sometype.iter()
        .filter(|i| cond(i))

// These two are the same

for i in iter {
    if cond(i) {
        ...
    }
}```
we can then follow up those dotted lines with a call to `map()`.
```rust
sometype.iter()
        .filter(|i| cond(i))
        .map(|i| do_something(i))
```

Rust also provides us with a handy `filter_map` that can help us condense the following pattern:
```rust
sometype.iter()
        .map(|i| do_something(i))
        .filter(|i| cond(i))
// can become:
sometype.iter()
        .filter_map(|i| cond(do_something(i)))
```

### Fold

Performing a `fold()` allows us to collapse our iterator into a single value.

Mathematically a fold is anything that can be recursively defined as:
$$
\begin{aligned}
\text{fold}(f,x) = f(fold(f,x_{i-1})))
\end{aligned}
$$
or
$$
\begin{aligned}
f(f(f(x_0),x_1),x_2),...
\end{aligned}
$$
for example, the sum of all the elements in a list can be defined recursively like that:
$$
\text{sum}(A) = (((((A_0)+A_1)+..)+A_k)+A_n)
$$
This might be a bit confusing, but it makes more sense in code.
In the language of for loops it is equivalent to:
```rust
let mut acc = start;
for i in sometype {
    acc = do_something(i);
}

sometype.iter()
        .fold(start,|acc,i| do_something(i));
```
For example, we can take a sum as:
```rust
let mut sum = 0;
for i in sometype {
    sum = += i;
}

sometype.iter()
        .fold(0,|sum,i| sum+i);

```

There is a similar pattern called `reduce()` that is like `fold()` except `acc` starts at the first element of our iterator.
However, **be careful** using reduce as it might not be allowed by judges using
an older version of Rust's compiler.
Also note that `reduce()` returns an `Option<T>` while `fold()` returns an actual type.

We can also take sums and products using the following methods:
```rust
sometype.iter()
        .sum::<T>();

sometype.iter()
        .product::<T>();
```

Combining `map()`, `filter()` and `fold()` allow us to write the following code more compactly:
```rust
let mut acc = start;
for i in sometype {
    let j = do_something(i);
    if cond(j) {
        let k = do_something_else(j);
        acc = finally(acc,k);
    }
}
return acc;
```
as:
```rust
sometype.iter()
        .filter_map(|i| cond(do_something(i)))
        .map(do_something_else)
        .fold(start,finally);
```
Think about how much easier this is to read,
and how much time you save coding!

### Scan

A slightly more advanced method is `scan()`. I say advanced because it's a little trickier to understand at first. However it is pretty much `fold()` except it returns an iterator instead of a single value:
$$
\begin{aligned}
\text{fold}(f,x) &= f(f(f(f(x))))\\
\text{scan}(f,x) &= [x,f(x),f(f(x)),f(f(f(x))),f(f(f(f(x))))]
\end{aligned}
$$
notice `fold()` is simply the last item in `scan()`.

### Collect

Finally we have `collect::<T>()` which collects an iterator into a collection.
Various data structures implement the `Collect` trait so using it is as simple as calling:
```rust
let v = type.iter()
            .map(do_something)
            .collect::<Vec<_>>();
```
It is roughly equivalent to the following for loop:
```rust
let mut v = Vec::with_capacity(type.len());
for i in type {
    v.push(do_something(i));
}
return v;
```

