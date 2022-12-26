pub trait IterExt: Iterator + Sized {
    fn my_map<F, R>(self, f: F) -> Map<Self, F>
        where 
            F: FnMut(Self::Item) -> R;
}

impl<T> IterExt for T 
    where T: Iterator 
{
    fn my_map<F, R>(self, f: F) -> Map<Self, F>
        where 
            F: FnMut(Self::Item) -> R 
    {
        map(self, f)
    }
}

pub fn map<I, R, F>(iter: I, func: F) -> Map<I, F> 
where
    I: Iterator,
    F: FnMut(I::Item) -> R,
{
    Map::new(iter,func)
}

pub struct Map<I, F> {
    input_iter: I,
    func: F,
}

impl<I, F, R> Map<I, F> 
where
    I: Iterator,
    F: FnMut(I::Item) -> R,
{
    pub fn new(iter: I, func: F) -> Self {
        Self {
            input_iter: iter,
            func,
        }
    }
}


impl<I, F, R> Iterator for Map<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> R,
{
    type Item = R;
    fn next(&mut self) -> Option<R> {
        if let Some(i) = self.input_iter.next() {
            return Some((self.func)(i));
        }
        None
    }
}


#[test]
fn works() {
    let a = [1, 2, 3];

    let doubled: Vec<i32> = map(a.iter(), |&x| x * 2)
                             .collect();

    assert_eq!(vec![2, 4, 6], doubled);
}

#[test]
fn dot_syntax_works() {
    let a = [1, 2, 3];

    let doubled: Vec<i32> = a.iter().my_map(|&x| x * 2)
                             .collect();

    assert_eq!(vec![2, 4, 6], doubled);
}

