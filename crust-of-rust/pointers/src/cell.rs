use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>
}

// We need to tell the compiler that Cell is not
// sharable across threads!
//
// impl<T> !Sync for Cell<T> {}
//
// But, this is impled by UnsafeCell, as it is !Sync
// https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html#impl-Sync-for-UnsafeCell%3CT%3E

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {value: UnsafeCell::new(value)}
    }

    pub fn set(&self, value: T) {
        // SAFETY: no one is concurrently mutating the value (because !Sync)
        // SAFETY: we are never invalidating references, because we never give out any
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T 
    where
        T: Copy
    {
        // SAFETY: no one is concurrently mutating the value (because !Sync)
        // only this thread can mutate and it is executing this function
        unsafe {
            *self.value.get()
        }
    }
}





// Mutating from two threads should fail to compile 
// Doc test allow us a handy way to do compile fail tests
/// ```compile_fail
/// fn should_not_compile() {
///     use std::sync::Arc;
/// 
///     let x = Arc::new(Cell::new(42));
/// 
///     let x1 = Arc::clone(&x);
/// 
///     std::thread::spawn( || {
///         x1.set(43);
///     });
/// 
///     let x2 = Arc::clone(&x);
/// 
///     std::thread::spawn( || {
///         x2.set(41);
///     });
/// }
/// ```
#[allow(dead_code)]
struct CompileFailTest;
