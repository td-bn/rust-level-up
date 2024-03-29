# Declerative macros

## Declaring
```rust
	macro_rules! avec {
	    () => {};
	}
```
We can supply any arguments to the macro definition as if it were a function e.g. 
```rust
	macro_rules! avec {
	    ($arg1: ty, $arg2:expr, $arg3: path) => {};
	}
```

But, we don't have to.

Rust allows us to write whatever we want as syntax pattern as long as its syntactically valid Rust. 
It should be parsed. And the output has to be **valid Rust grammar**.
That means its doesn't need to compile, but it must be comprised of valid Rust syntax. e.g.
```rust
	macro_rules! avec {
	    ($arg1: ty => $arg2:expr; $arg3: path) => {};
	}
```

NOTE: Rust macros are not allowed to use values outside their own scope.


## using macros 
```rust
	macro_rules! avec {
	    () => {};
	}
```
We can use any brackets, because there is no way to specify
what brackets we expect in current syntax.

SO its up to the user to use what he wants.

```rust
 avec!();
 avec![];
 avec! {}
```


## Cargo expand

Handy tool to expand all the macros in out code.
```rust
macro_rules! avec {
    ($arg1: ty => $arg2:ident) => {
        type $arg2 = $arg1; 
    };
}


avec!{u32 => AlsoU32}
```
expands to:

```Rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
type AlsoU32 = u32;
```


## Patterns

We can specify patterns for repitition logic:
```rust
    // $($elem:expr),+: 1 or more of this pattern
    // $(,)?: Zero or one trailing comma
    ($($elem:expr),+ $(,)?) => {
        // We want this to be a block so that when we
        // do `let x = avec![..] cargo doesn't get mad
        {
            let mut vs = Vec::new();
            // Repeat inside these parens the same num of time
            // as the pattern that had 'elem' in it
            $(vs.push($elem);)*
            vs
        }
    };
```

