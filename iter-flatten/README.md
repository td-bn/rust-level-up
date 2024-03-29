# Iterators

Iterator in Rust is a trait.
It has an associate type named `Item` and a method called `next`.

Item is the type of item that the iterator will yield. It will call next
and return items until they are exhausted at which point it returns `None`.

Iterators are also composable, and it’s common to chain them together
to do more complex forms of processing.

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

## Three forms of iterators

Three common ways to create iterators form a collection:
```rust
iter() // which iterates over &T.
iter_mut() // which iterates over &mut T.
into_iter() // which iterates over T.
```

## How for loop works under the hood

A for loop like
```rust
for x in vec!["a", "b", "c"] 
    ...
}
```
turns into something like:
```rust
let iter = vec![...].into_iter();
while let Some(x) = iter.next() {
    ...
}
```

In actuality it is something like: 
```rust
    // let values = vec![...];
    let result = match IntoIterator::into_iter(values) {
        mut iter => loop {
            let next;
            match iter.next() {
                Some(val) => next = val,
                None => break,
            };
            let x = next;
            // ....
        },
    };

```

## Into iter

```rust
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter;
}
```

Sort of a wrapper trait around iter. Anything that can be turned into an 
iterator.
Usually used for collections. By definiting `IntoIter` for a type, we define how it can be converted into an iterator. 

In addition to an associated type `Item`, it also has an associtaed type `IntoIter`.

If a collection `C` implements `iter()`, it usually also implements `IntoIterator`
for `&C`, with an implementation that just calles `iter()`.
Likewise, a collection `C` that implements `mut_iter()`, it usually also
implements `IntoIterator` for `&mut C`, with an implementation that just calles `iter_mut()`.

This enables convenient shorthands like:
```rust
let mut values = vec![41];
for x in &mut values { // same as `values.iter_mut()`
    *x += 1;
}
for x in &values { // same as `values.iter()`
    assert_eq!(*x, 42);
}
assert_eq!(values.len(), 1);
```

## Associated type vs generic type

Why do we have?
```rust
trait Iterator {
   type Item;
   ..
}
```
and not
```rust
trait Iterator<Item> {
   ...
```

Both of these would work. 
An associated type is used when we expect there to be only one implementation
of the trait for a given type.

e.g. if we have an iterator type for a HashMap there is only one 
implementation for it keys and values.

On the other hand we want generic types for a trait when we expect to have
multiple implementation for the same type.

e.g a service trait that supports multiple types of requests.

**Why don't we use generic always?**
It saves the type checker some headache. We make its job easier.
Reduces extra generic type params that we need to use.


## Different types of for loops

```rust
// General idea of Rust: being explicit is good

for v in vs {
    // consumes vs and owned acess to v
}

for v in vs.iter() {
    // borrows vs, & to v
}

for v in &vs {
    // eq to vs.iter()
}
```


## Other method in Iterator trait

Provides default implementation for other methods by using Item and next().
Note that `Iterator` provides a default implementation of methods such as 
`nth` and `fold` which call next internally. 
However, it is also possible to write a custom implementation of methods 
like `nth` and `fold` if an iterator can compute them more efficiently without 
calling `next`.

