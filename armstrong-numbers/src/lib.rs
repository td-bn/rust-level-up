pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum_pow: u64 = 0;

    // Pow to raise to 
    let mut n = num;
    let mut p = 0;
    while n!=0 { n/=10; p+=1;}

    let mut n = num;
    while n != 0 {
        let i: u64 = (n as u64)%10;
        sum_pow += i.pow(p) ;
        n /= 10;
    }

    sum_pow == num as u64
}

