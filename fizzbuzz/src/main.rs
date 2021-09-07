fn main() {
    let mut output;
    for i in 1..101 {
        output = "".to_string();
        if (i%3) == 0 {
            output.push_str("fizz");
        }
        if (i%5) == 0 {
            output.push_str("buzz");
        }
        if output == "" {
            println!("{}", i);
        }else {
            println!("{}", output);
        }
    }
}
