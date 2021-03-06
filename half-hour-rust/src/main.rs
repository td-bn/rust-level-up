fn main() {
    println!("Hello, world!");

    // Declare x
    let x;
    x = 42;
    println!("{}", x);

    // Type can be explicitly set using `:`
    let y: i32;
    y = 420;
    println!("{}", y);

    // One liner 
    let z: i32 = 42024;
    println!("{}", z);

    // _ can only be used in LHS of assignments, used
    // when we need to throw things away
    let _: i32 = 42024;

    // !!! throws error ^ `_` not allowed here
    // println!("{}", _);

    // Same name, separate binding can be introduced. 
    // Shadows previous
    let x = x * x;
    println!("{}", x);

    // Tuples are fixed length collection of values of different types
    let pair: (i32, char) = (65, 'a');
    println!("{}, {}", pair.0, pair.1);
    
    // Tuples can be destructured
    let (integer, character) = pair;
    println!("{}, {}", integer, character);

    // Throw away first value
    let (_, character) = pair;
    println!("{}", character);


    // ; marks end of statements, can be multiple line
    let x = vec![1, 2, 3, 4]
            .iter()
            .map(|x| x*x)
            .fold(0, |x,y| x + y);
    println!("Sum of squares: {}", x);
    
    // fn declares a function
    fn greet() {
        println!("Well, hello there!")
    }
    greet();


    // Fn that returns a i32
    fn unfair_roll_dice() -> i32 {
        6
    }
    println!("Rolling fairly: {}", unfair_roll_dice());


    // A pair of brackets declares a scope
    // Prints: "Well, hello there!"
    fn scopy_boy() {
        let x = "there!";
        {
            let x = "Well, hello ";
            print!("{}", x);
        }
        println!("{}", x);
    }
    scopy_boy();

    // Blocks are also expressions, which means they evaluate to a value
    let x = {42};
    println!("{}", x);

    // Can have mutiple lines
    let x = {
        let y = 10;
        let z = 32;
        y+z
    };
    println!("{}", x);

    // Omitting the semicolon is the same as returning
    fn fair_roll_dice() -> i32 {
        4
    }
    println!("Rolling fairly: {}", fair_roll_dice());

    {
        // If conditionals are also expressions
        fn fair_roll_dice(feeling_lucky: bool) -> i32 {
            if feeling_lucky {
                6
            } else {
                4
            }
        }
        println!("Rolling fairly: {}", fair_roll_dice(false));
    }

    {
        // A match is also an expression
        fn fair_roll_dice(feeling_lucky: bool) -> i32 {
            match feeling_lucky {
                true => 6,
                false => 3
            }
        }
        println!("Rolling fairly: {}", fair_roll_dice(false));
    }

    // :: are used to access modules from namespaces, and functions from modules
    let m = std::cmp::min(2,33);
    println!("Min of 2, 23: {}", m);
    
    // Curly braces can be used to import multiple items
    {
        // this works:
        // use std::cmp::min;
        // use std::cmp::max;

        // this also works:
        // use std::cmp::{min, max};

        // this also works!
        use std::{cmp::min, cmp::max};
        let m = min(2,33);
        let m_m = max(2,33);
        println!("Min of 2, 23: {} Max: {}", m, m_m);

    }
 

    {
        // Types are namespaces as well
        let _x = "diggory".len();
        let x = str::len("diggory");
        println!("Length of diggory: {}", x);

    }

    {
        // NOTE: Some types are already in the scope
        // This is because Rust imports the follwing in every module
        // use std::prelude::v1::*;
    }

    {
        // Structs are declared using the 'struct' keyword
        struct Point {
            x: i32,
            y: i32
        }
        let center = Point{ x: 10, y: 12};
        println!("x:{}, y:{}", center.x, center.y);

        // Shortcut for initializing rest of fields from another struct
        let left = Point { x: 5, ..center};
        println!("x:{}, y:{}", left.x, left.y);

        // Structs can be destructured
        let Point{x, y} = left;
        println!("x:{}, y:{}", x, y);

        // Also valid
        let Point{x, ..} = left;
        println!("x:{}", x);

    }


    // Let patterns can be used as conditions in if
    {
        struct Number {
            odd: bool,
            value: i32
        }

        fn driver() {
            let one = Number {odd: true, value: 1};
            let hundred = Number {odd: false, value: 100};
            print_number(one);
            print_number(hundred);
        }

        fn print_number(n: Number) {
            if let Number {odd: true, value} = n {
                println!("Odd number: {}", value);
            } else if let Number {odd: false, value} = n {
                println!("Even number: {}", value);
            }
        }
        driver();
    }

    // match arms are also patterns, just like if let:
    {
        struct Number {
            odd: bool,
            value: i32
        }

        fn driver() {
            let one = Number {odd: true, value: 1};
            let hundred = Number {odd: false, value: 100};
            print_number(one);
            print_number(hundred);
        }

        fn print_number(n: Number) {
            match n {
                Number {odd: true, value} => println!("Odd number: {}", value),
                Number {odd: false, value} => println!("Even number: {}", value)
            }
        }
        driver();
    }

    // A match has to be exhaustive: at least one arm needs to match.
    {
        struct Number {
            value: i32
        }

        let one = Number {value: 1};
        let hundred = Number {value: 100};

        fn print_number(n: Number) {
            match n.value {
                1 => println!("One"),
                2 => println!("Two"),
                _ => println!("{}", n.value)
            }
        }

        print_number(one);
        print_number(hundred);

    }

    // You can declare methods on your own types:
    {
        struct Number {
            value: i32
        }

        impl Number {
            fn is_positive(self) -> bool {
                self.value > 0
            }
        }

        let minusone = Number {value: -1};
        let hundred = Number {value: 100};

        fn print_number(n: Number) {
            println!("Positive? {}", n.is_positive());
        }

        print_number(minusone);
        print_number(hundred);

    }

    // Variable bindings are immutable by default
    {
        struct Number {
            odd: bool,
            value: i32
        }

        let n = Number {
            odd: true,
            value: 99
        };
        // n.value = 97; Error
        // n = Number { odd: false, value: 90} //Error
        println!("{} {}", n.odd, n.value);
    }

    // mut makes a variable binding mutable
    {
        struct Number {
            odd: bool,
            value: i32
        }

        let mut n = Number {
            odd: true,
            value: 99
        };

        n.value = 96;
        n.odd = false;
    }

    // Traits
    /*
        Traits are something that multiple types have in common.

        (ORPHAN RULES)
        We can implement:
        * our traits on ANY types
        * foreign traits on our types
        * BUT NOT foreign types on foreign types

    */


    // Our trait, our type
    {
        trait Signed {
            fn is_negative(self) -> bool;
        }

        struct Number {
            value: i32
        }

        impl Signed for Number {
            fn is_negative(self) -> bool {
                self.value < 0
            }
        }

        let n = Number{value: 1};
        println!("Negative? {}", n.is_negative());

    }

    // Our trait, foreign type
    {
        trait Signed {
            fn is_negative(self) -> bool;
        }

        impl Signed for i32 {
            fn is_negative(self) -> bool {
                self < 0
            }
        }

        let n = -10;
        println!("Negative? {}", n.is_negative());
    }

    // Foreign trait, our type
    {
        struct Number {
            value: i32
        }

        impl std::ops::Neg for Number {
            type Output = Self;
            fn neg(self) -> Self {
                Self {
                    value: -self.value
                }
            }
        }

        // let n = Number { value: 99 };
        // println!("n: {}\tNegative n: {}", n.value, n.neg().value);
    }

    // Marker traits
    /*
      Some traits are markers
      They don't say that the type implements some methods
      They say certain things can be done with the type
     */
    {
        // i32 implements the copy trait i.e. i32 is Copy
        let a: i32 = 42;
        let b = a; // a is copied
        let c = a; // a is copied again
        println!("Copied a into b and c. Value: {}, {}", b, c);
    }

    {
        fn print_num(x: i32) {
            println!("{}", x);
        }

        let a = 42;
        print_num(a); // a is copied
        print_num(a); // a is copied again
    }

    {
        // Following code results in error

        // let n = Number { value: 51 };
        // let m = n; // `n` is moved into `m`
        // let o = n; // error: use of moved value: `n`
        
        // So does THIS 

        // fn print_number(n: Number) {
        //     println!("number {}", n.value);
        // }

        // fn main() {
        //     let n = Number {value: 51 };
        //     print_number(n); // `n` is moved
        //     print_number(n); // error: use of moved value: `n`
        // }
    }
}
