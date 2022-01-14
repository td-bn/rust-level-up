# RAII: Resource Acquisition Is Initialization

In addition to holding data on the stack, variables in Rust also __own__ resources.
e.g. `Box<T>` owns memory in the heap

Rust enforces `RAII` so whenever an object goes out of scope , its desctructor is called and its __owned resources are freed__.

This behavior shields against resource bugs: one never has to manually free memory or protect against memory leaks.

```rust
fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);

    // `_box1` is destroyed here, and memory gets freed
}
```

# Drop Trait / Desctruction

The notion of desctruction in Rust is provided by the `Drop` trait. We don't need to manually implement the drop trait for our types. This is only requried when we need to provide custom logic.

```rust
struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}
```

Output: 
```
Made a ToDrop!
ToDrop is being dropped
```