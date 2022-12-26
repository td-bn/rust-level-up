#[macro_export]
macro_rules! avec {
     // $($elem:expr),+: 1 or more of this pattern
    // $(,)?: Zero or one trailing comma
    ($($elem:expr),* $(,)?) => {
        // We want this to be a block so that when we
        // do `let x = avec![..] cargo doesn't get mad
        {
            #[allow(unused_mut)]
            let mut vs = Vec::new();
            // Repeat inside these parens the same num of time
            // as the pattern that had 'elem' in it
            $(vs.push($elem);)*
            vs
        }
    };
    ($elem:expr; $count:expr) => {
        {
            // Image if $elem has a take(). It  would only work the first 
            // time around we DON"T want to evaluation the expression twice
            let count = $count;
            let mut vs = Vec::new();
            vs.resize(count, $elem);
            vs
        }
    }
}


#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}

#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert!(x.len() == 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42, 43];
    assert!(!x.is_empty());
    assert!(x.len() == 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn trailing() {
    // Imagine this is a super long list
    let _: Vec<&'static str> = avec![
    "kjdlkfjaklsdfjlaksdjfklaskdfjkalsdjfklasdjflkjaf",
    "kjdlkfjaklsdfjlaksdjfklaskdfjkalsdjfklasdjflkjaf",
    "kjdlkfjaklsdfjlaksdjfklaskdfjkalsdjfklasdjflkjaf",
    "kjdlkfjaklsdfjlaksdjfklaskdfjkalsdjfklasdjflkjaf",
    ];
}


#[test]
fn with_count() {
    let x: Vec<u32> = avec![0; 5];
    assert!(x.len() == 5);
    assert_eq!(x[0], 0);
    assert_eq!(x[4], 0);
}

#[test]
fn unwrap_with_count() {
    let mut y = Some(42);
    let x: Vec<u32> = avec![y.take().unwrap(); 2];
    assert!(x.len() == 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}


// Doc test allow us a handy way to do compile fail tests
/// ```compile_fail
/// let x: Vec<u32> = vecmac::avec![42, "foo"];
/// ```
#[allow(dead_code)]
struct CompileFailTest;

