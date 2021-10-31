## Tuples

* Tuples are collections of values of different types
* Construted using parenthesis ()
* Each tuple is itself a value with the type signature (T1, T2, ...)
* Tuples can be used as function arguments and return types

```rust
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables
    let (integer, boolean) = pair;

    (boolean, integer)
}
```

* A tuple can have a bunch of different types
* Indexing can be used to extract values

```rust
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, -4u64,
                      0.1f32, 0.2f64,
                      'a', true);

    println!("long tuple second value: {}", long_tuple.1);

```

* Tuples can be nested i.e. we can have tuples as members of tuples
* Too long tuples cannot be printed
* To create on element tuples a `,` is needed

```rust
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (integer, boolean) = pair;
        (boolean, integer)
    }

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));
```

* Tuples can be desctructured in bindings

```rust
    //tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
```


