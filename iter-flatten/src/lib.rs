// Implement extention trait for Iterator
pub trait IterExt: Iterator + Sized {
    fn our_flatten(self) -> Flatten<Self>
        where 
            Self::Item: IntoIterator;
}

impl<T> IterExt for T
    where 
        T: Iterator 
{
    fn our_flatten(self) -> Flatten<Self> 
        where 
            Self::Item: IntoIterator
    {
        flatten(self)
    }
}


// A function that takes an iterator 
//  where the item of the iterator is IntoIterator
//  which means that the item of the iter can also be converted into an iter
// Returns a struct Flatten
pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter> 
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

// Need outer and inner to keep track of iterators
pub struct Flatten<O> 
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    // Type of inner is an optional Item::IntoIter
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl <O> Flatten<O> 
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten { outer: iter , inner: None}
    }
}

// Implement iterator for our Flatten type
impl <O> Iterator for Flatten<O> 
where 
    O: Iterator,
    O::Item: IntoIterator,
{
    
     // Item is Item of inner iterator
     type Item = <O::Item as IntoIterator>::Item;
     // How do we get the next Item
     fn next(&mut self) -> Option<Self::Item> {
         loop {
             if let Some(ref mut inner_iter) = self.inner {
                 if let Some(i) = inner_iter.next() {
                     return Some(i);
                 }
                 self.inner = None;
             }

             let next_inner_iter = self.outer.next()?.into_iter();
             self.inner = Some(next_inner_iter);
         }
     }
}

#[test]
fn empty() {
    assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0)
}

#[test]
fn one_item() {
    assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1)
}

#[test]
fn flattens_vecs() {
    assert_eq!(flatten([[1,2], [3,4]]).count(), 4);
}

#[test]
fn flattens_vecs_iter() {
    // Test iter.flatten notation
    assert_eq!([[1,2], [3,4]].into_iter().our_flatten().count(), 4);
}

