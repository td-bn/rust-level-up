
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}


// I'll give you a StrSplit that lasts for as long as the lifetime of the &str you give me
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delim: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter: delim
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
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
fn fails() {
    let haystack = "a b c d ";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();

    assert_eq!(letters, vec!["a", "b", "c", "d", ""]);
}
