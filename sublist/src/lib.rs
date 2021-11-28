#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    use std::cmp::Ordering;

    match a.len().cmp(&b.len())  {
        Ordering::Equal => if a == b {return Equal} else {return Unequal},
        Ordering::Less => if contains(b, a) {return Sublist} else {return Unequal},
        Ordering::Greater => if contains(a, b) {return Superlist} else {return Unequal}
    }
}

fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    b.is_empty() || a.windows(b.len()).any(|x| x == b)
}
