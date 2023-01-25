use rand::{RngCore};

#[derive(Debug)]
enum RandomError {
    Rem1Error(u32),
    Rem2Error(u32),
}

impl std::fmt::Display for RandomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        match self {
            RandomError::Rem1Error(n) => write!(f, "Remainder is 1 % 5\nNumber is {}", n),
            RandomError::Rem2Error(n) => write!(f, "Remainder is 2|3|4 % 5\nNumber is {}", n),
        }
    }

}
impl std::error::Error for RandomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    } 
}

fn that_might_throw_an_error() -> Result<u32, RandomError> {
    let mut rng = rand::thread_rng();
    let random_number = rng.next_u32();
    
    match random_number % 5 {
        0 => return Ok(random_number),
        1 => return Err(RandomError::Rem1Error(random_number)),
        _ => return Err(RandomError::Rem2Error(random_number)),
    }
}

fn main() {
    match that_might_throw_an_error() {
        Ok(num) => {
            println!("Num: {}", num);
        },
        Err(e) => {
            println!("Err: {}", e);
        },
    }
}

