# Iterators

Iterator in Rust is a trait.
It has an associate type named `Item` and a method called `next`.

Item is the type of item that the iterator will yield. It will call next
and return items until it returns `None`.

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
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


