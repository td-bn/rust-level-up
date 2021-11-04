## Arrays and Slices

_An array is a collection of objects of the same type `T`, stored in contiguous memory._

* Arrays are created using square brackets []
* Their length is **known** at compile time
* Type signature [T; length]

```rust
use std::mem;

let odds: [i32; 5] = [1,3,5,7,9];

// All elements can be initialized to the same value
let ys: [i32; 500] = [0; 500];

// `len` returns the count of elements in the array
println!("number of elements in array: {}", xs.len());


// Arrays are stack allocated
println!("array occupies {} bytes", mem::size_of_val(&xs));

```

Slices are similar to arrays, but their length is not known at compile time.

* Slices are two word objects
* Fist word is a **pointer** to the data, and the second word is the **length** of the slice.
* Slices can be used to borrow a section of an array
* Type signature &[T]


```rust

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

let xs: [i32; 5] = [1, 2, 3, 4, 5];

// Arrays can be automatically borrowed as slices
println!("borrow the whole array as a slice");
analyze_slice(&xs);

// Slices can point to a section of an array
// They are of the form [starting_index..ending_index]
// starting_index is the first position in the slice
// ending_index is one more than the last position in the slice
println!("borrow a section of the array as a slice");
analyze_slice(&ys[1 .. 4]);

// Out of bound indexing causes compile error
println!("{}", xs[5]);

```



