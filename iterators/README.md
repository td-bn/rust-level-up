# Iterator Trait
The iterator trait is used to implement iterators over collections such as arrays.

The trait only require a method `next` to be declared. This can be manually defined or done automatically for arrays and ranges.

```rust
let a = [1, 2, 3];

let mut iter = a.iter();

// A call to next() returns the next value...
assert_eq!(Some(&1), iter.next());
assert_eq!(Some(&2), iter.next());
assert_eq!(Some(&3), iter.next());

// ... and then None once it's over.
assert_eq!(None, iter.next());
```

Also has some nice handy methods: step_by, enumerate, filter, map etc.

step_by creates an iterator starting at the same point, but stepping by the given amount at each iteration.


```rust
let a = [0, 1, 2, 3, 4, 5];
let mut iter = a.iter().step_by(2);

assert_eq!(iter.next(), Some(&0));
assert_eq!(iter.next(), Some(&2));
assert_eq!(iter.next(), Some(&4));
assert_eq!(iter.next(), None);
```

DOCS: https://doc.rust-lang.org/std/iter/trait.Iterator.html