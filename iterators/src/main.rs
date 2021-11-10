struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

fn fib() -> Fibonacci {
    Fibonacci{curr: 0, next: 1}
}

fn main() {
    // `take(n)` method reduces an `Iterator` to its first `n` terms.
    for i in fib().take(5) {
        println!("{}", i)
    }    

    for i in fib().skip(20).take(5) {
        println!("{}", i)
    } 

    // `0..3` is an `Iterator` that generates: 0, 1, and 2.
    for i in 0..3 {
        println!("> {}", i);
    }
}