Rust's central feature is ownership!

## Problem of memory management

Every program has to deal with memory management in some form or the other. Some languages adopt a garbage collection mechanism, whereas others allow programmers to control the allocation and freeing of memory.

Rust uses a system of ownership to solve the problem of memory management.

### Stack vs Heap

Stack and heap are the parts of memory that are available to our programs at runtime. Both are structured differently. 

Stack is LIFO. All data going on the stack must have a fixed size. 

Data of uknown sizes at compile time or whose size might change have to be stored on the heap. The heap is less organized. When you request a certain amount of space on the heap, the allocator finds an empty spot, marks it as being used and returns a pointer to it. We can store this pointer on the stack. 

Pushing to a stack is faster than a heap because of the overhead of allocation involved in heaps and bookkeeping, and because the location in stacks is always the top of the stack. 

Reading from the heap is slower than reading from the stack as well because we have to follow a pointer. 

Modern processors also work better when the data they are accessing is closer to each other. 

**Managing heap data is WHY ownership exists.**

### Rules of Ownership

1. Each value in Rust has a variables that's called its' owner.
2. Only one owner at a time
3. When owner goes out of scope the value is dropped

### Curious case of String and String literal

String literals are stored in the stack because we know the size at compile time making them fast and efficient.

```rust
let s = "hello";
```

Scope being the range in the program where an item is valid.

```rust
fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}
```

String literals can't be used everywhere because they are immutable. When we need mutability, we need extendible strings which cannot be stored on the stack. This type is allocated on the heap.

```rust
fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`
}
```

With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time,

### Memory deallocation

Rust doesn't use GCs or explicit freeing memory like in some of the other languages. 

```rust
fn main() {
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
}
```

When a variable goes out of scope, Rust calls a special function for us. This function is called `drop`, and it’s where the author of String can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.

### Move in Rust

Consider the following two snippets of code given below:

```rust
let x = 5;
let y = x;

// vs

let s1 = String::from("hello");
let s2 = s1;
```

At first glance they seem to be doing the same thing. Binding a value to a variable and then making a copy of that value. 

BUT this is exactly how things are working below the hood.

For simple types like integers, we have simple values with a known fixed size which are stored on the stack. 

String are more complex. They are not stored on the stack and do not have a fixed known size. This is how strings are stored. 

![Untitled](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/11eeaf18-c413-482b-bdfe-7164f704c2a9/Untitled.png)

The metadata part of the left, is of known size and is stored on the stack. Whereas the contents of the string itself are stored on the heap. 

![Untitled](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/e666dba4-7453-4329-a0ce-9b8bc0e90b2f/Untitled.png)

When we copy strings, we do not copy the data on the heap itself, instead we copy only the pointer to the data. If Rust copied the data on the heap, the operation s2 = s1 could be very expensive in terms of runtime performance if the data on the heap were large.

But, copying the pointer raises concerns. When s2 and s1 go out of scope, they will both try to free the same memory. This is known as a double free error and is one of the memory safety bugs. To ensure memory safety, there’s one more detail to what happens in this situation in Rust. Instead of trying to copy the allocated memory, Rust considers s1 to no longer be valid and, therefore, Rust doesn’t need to free anything when s1 goes out of scope. 

```rust
		let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // error
```

s2=s1 looks like a shollow copy based on copying pointer to the string, but Rust also invalidates the first variable, so instead of being called a shallow copy, it’s known as a **move**.

![Untitled](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/e6c5b2e6-2f5a-42c2-b499-52f3ef157203/Untitled.png)

That solves our problem! With only s2 valid, when it goes out of scope, it alone will free the memory, and we’re done.

Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

## Cloning in Rust

When we want to create deep copies of Strings, we can use a method called `clone` 

```rust
		let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2); // Works!
```

## Stack only data copy in Rust

```rust
		let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y); // Works!
```

This seems to contradict what we learned about Strings, but the difference is that integers have fixed size which is known at compile time. In essence there is no difference between deep and shallow copies here. 

## How does Rust handle this?

Rust has a special annotation called the Copy trait that we can place on types like integers that are stored on the stack. If a type implements the Copy trait, an older variable is still usable after assignment. Rust won’t let us annotate a type with the Copy trait if the type, or any of its parts, has implemented the Drop trait.

As a general rule, any group of simple scalar values can implement `Copy`, and nothing that requires allocation or is some form of resource can implement `Copy`. Here are some of the types that implement `Copy`:

- All the integer types, such as `u32`.
- The Boolean type, `bool`, with values `true` and `false`.
- All the floating point types, such as `f64`.
- The character type, `char`.
- Tuples, if they only contain types that also implement `Copy`. For example, `(i32, i32)` implements `Copy`, but `(i32, String)` does not.

## Ownership and Functions

The semantics for passing a value to a function are similar to those for assigning a value to a variable. Passing a variable to a function will move or copy, just as assignment does.

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

If we tried to use s after the call to takes_ownership, Rust would throw a compile-time error.

Returning values can also transfer ownership

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it.