# Smart pointers and Interior Mutability


## Unsafe Cell

Core primitive for interior mutability in Rust.

An usafe primitive. UnsafeCell opts out of the immutability guarantee for &T.

A shared reference to `&UnsafeCell<T>` may point to data that is being mutated.




## Cell

Rust allows shared references: everyone can read ONLY.
Rust allows mutable references: ONLY one owner can change things.

Cell is a sharable mutable reference. Sometimes it is required to have multiple references to an object and yet mutate it.

Cell allows us to do this is a single threaded context. Usually used to a flag or an int in a thread local environment.

Cell is copy. Generally used for small types that are Copy.


## RefCell

Normally all borrow checks are done at compile time. 
RefCell allows us to do safe dynamically checked borrowing.


## Rc

Rc refers to reference counting pionters. 

It provides a shared ownership of a value of type `T`, allocating in the heap. 
Calling a `clone` method returns a pointer to the same allocation in the heap.

When the last `Rc` pointer to the allocation is destroyed, the value is dropped.

Since sharing mutable references is not permitted by default in Rust, we have to put a 
`Cell` and `Refcell` insdie an `Rc`.

`Rc` cannot be send between thread.


## Thread safe version of these

### Cell
No thread safe version. Having two threads modify the same reference at the same time is not okay.

### Refcell -> RwLock
`RwLock` is kinda similar in functionality to a RefCell. 

`RwLock` doesn't return `Option`s for giving out read and write references. Instead it using 
blocks the thread if the borrow can't suceed. When the conditions are met, the operations are
allowed.

### Rc -> Arc
Thread safe reference counting pointer.
Use CPU atomics to managing the reference count.


NOTE: There is a cost to using these atomics. That's why we might want to use Cell, RefCells and Rcs.


