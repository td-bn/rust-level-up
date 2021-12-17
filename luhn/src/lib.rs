/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    for ch in code.chars() {
        if !(ch.is_digit(10) || ch.is_ascii_whitespace()) {
            return false;
        }
    }

    let nums = code.chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>();

    if nums.chars().count() < 2 { return false; }

    nums.chars()
    .map(|c_digit| c_digit.to_digit(10).unwrap())
    .rev()
    .enumerate()
    .map(|(i, d)| {
        if i%2 != 0{
            if 2*d > 9 
                {2*d -9}
            else 
                {2*d}
        } else {d}
    }).sum::<u32>() % 10 == 0
}
