fn main() {
    // Variable
    {
        let name = "Linz";
        println!("Hello, {}!", name);
    }
    // Struct 
    {
        struct Greeting {
            name: String,
        }

        let greeting = Greeting{name: "Linz".to_string()};
        println!("Struct: {}!", greeting.name);
    }

    // Constructor
    {
        struct Greeting {
            name: String,
        }

        impl Greeting {
            fn new(name: &str) -> Self {
                Greeting {
                    name: name.to_string()
                }
            }
        }
        let greeting = Greeting::new("Linz");
        println!("Constructor: {}!", greeting.name);
    }

    // Accepting both &str and String
    /*
     * from docs: By creating a generic function that takes an AsRef<str> we express 
     * that we want to accept all references that can be converted to &str as an 
     * argument. Since both String and &str implement AsRef<str> we can accept 
     * both as input argument.
     */
    {
        struct Greeting {
            name: String,
        }

        impl Greeting {
            fn new<T: AsRef<str>>(name: T) -> Self {
                Greeting {
                    name: name.as_ref().to_string()
                }
            }
        }
        let greeting = Greeting::new("Linz");
        println!("AsRef: {}!", greeting.name);
    }


    // Implementing display for Greeting
    {
        use std::fmt;

        struct Greeting {
            name: String,
        }

        impl Greeting {
            fn new<T: AsRef<str>>(name: T) -> Self {
                Greeting {
                    name: name.as_ref().to_string()
                }
            }
        }

        impl fmt::Display for Greeting {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    write!(f, "Display: {}", self.name)
                }
        }
        let greeting = Greeting::new("Linz");
        println!("{}!", greeting);
    }


    // Cool Rust example
    /*
     * implementation of std::mem::drop
     * pub fn drop<T>(_x: T) {
     * 
     * }
     */

     // Entry API
     {
        use std::collections::HashMap;

        let mut letters = HashMap::new();

        for ch in "Hello Linz!".chars() {
            let counter = letters.entry(ch).or_insert(0);
            *counter += 1;
        }
        
        println!("Entry API: number of l's is {}", letters.get(&'l').unwrap());
     }

     // Cool Rust Example

    /*
        Implementation of std::convert::Into

        impl<T,U> Into<U> for T
        where
            U: From<T>, 
        {
            fn into(self) -> U {
                U::from(self)
            }
        }
    */
}
