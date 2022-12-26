
pub struct StrSplit<'a, 'delim> {
    remainder: Option<&'a str>,
    delimiter: &'delim str,
}


// I'll give you a StrSplit that lasts for as long as the lifetime of the &str you give me
impl<'a, 'delim> StrSplit<'a, 'delim> {
    pub fn new(haystack: &'a str, delim: &'delim str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter: delim
        }
    }
}

impl<'a, 'delim> Iterator for StrSplit<'a, 'delim> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delim = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delim)
        } else {
            self.remainder.take()
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}

#[test]
fn it_works_for_chars() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, &format!("{}", ' ')).collect();

    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}


#[test]
fn fails() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
