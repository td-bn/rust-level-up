fn main() {
    println!("Hello, world!");
    // x is a `function item`
    let _x = bar;
    // This would throw an error?
    // cannot infer type for type parameter `T` declared on the function `g_bar`
    // let gx = g_bar;

    // But this would work
    // This instances g_bar with i32 as the type
    let gx = g_bar::<i32>;
    println!("{}", std::mem::size_of_val(&gx)); // 0

    /*
     *  If we did something like this:
     *
     *  gx = g_bar::<u32>;
     *  print_type_of(&gx);
     *
     *  we'd get the following error:
     *
     *
     *     let mut gx = g_bar::<i32>;
     *                  ------------ expected due to this value
     *
     *     gx = g_bar::<u32>;
     *          ^^^^^^^^^^^^ expected `i32`, found `u32`
     *
     *     g_bar::<i32> is not the same type as g_bar::<u32>
     */

    // But function items can be coerced into function pointer types e.g. 
    
    // Function pointers
    baz(foo::<u32>);
    baz(foo::<i32>);

    // Trait 
    quox(&foo::<i32>);
}

fn bar() {}
fn g_bar<T>() {}

fn foo<T>(_: u32) -> u32 {0}

// Here f is a function pointer type
// Note `fn`
// 
// Function pointer is a pointer to an instantiation of a 
// function. It points to the code of the function. 
// It does not have a state associated with it.
//
// They don't access state outside themselves. There is nothing to move.

fn baz(f: fn(u32) -> u32) {
    println!("{}", std::mem::size_of_val(&f)); // 8
}

// Function traits? Heirarchy?
// Function traits are traits and not types 
// Fn, FnMut and FnOnce
//
// Function pointers implement all three of these traits.
// If we have a function pointer we can pass it to something that 
// is trait bound by any of these three traits.
//
// Fn -> takes a shared reference to self 
// FnMut -> takes a exclusive reference to self 
// FnOnce -> takes an owned value of self 
//
// Note there is a heirarchy here:
//  Fn implemnts FnMut, FnOnce 
//  FnMut implements FnOnce 
//  function pointers can be converted into Fn, FnMut or FnOnce


fn quox<F>(f: &F) -> u32
    where 
        F: Fn(u32) -> u32,
{
    f(0)
}


// To call a FnMut we need to get a exclusive reference to the f in the above function
//
// fn quox<F>(f: &mut F) -> u32 
//      where 
//          F: FnMut...
//
// Similarly ownership for FnOnce


// Where this become useful is when working with closures
// 
// Non capturing closures are corecible to function pointers 
//      
//      |x:i32, y: i32| x+y
//
// On the other hand, capturing closures consume 
//      
//      let z = String::new();
//      let f = || {
//          z.clear()
//      }
//      quox(f);
//
//  This is a capturing closure and passing f into quox would require f to be FnMut
//
// If this were an owned version:
//      let z = String::new();
//      let f = || {
//          drop(z);
//      }
//      quox(f);
//
// This is a capturing closure and passing f into quox would require f to be FnOnce
//
// If this were an owned version:
//      let z = String::new();
//      let f = || {
//          drop(z);
//      }
//      quox(f);
//
//  This is a capturing closure and passing f into quox would require f to be FnOnce


#[allow(dead_code)]
fn make_func() -> impl Fn() {
    let z = String::new();
    move || { // move z into the closure
        // z is dropped when the closure is dropped
        println!("{}", z);
    }
}



