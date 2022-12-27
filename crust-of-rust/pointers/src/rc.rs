use crate::cell::Cell;
// Sort of like a *mut that we can use for Box::from_raw
use std::ptr::NonNull;
// Zero-sized type used to mark things that “act like” they own a T
// Why is it needed? 
//
// Something something drop check
// See: 
//  - https://doc.rust-lang.org/nomicon/phantom-data.html
//  - https://doc.rust-lang.org/nomicon/dropck.html
use std::marker::PhantomData;


// Shared state of the Rc 
// we need to keep track of refcount
struct SharedState<T> {
    value: T,
    refcount: Cell<usize>,
}

// has the raw pointer to the shared state
pub struct Rc<T> {
    state: NonNull<SharedState<T>>,
    // Tells the compiler to check to look for a drop
    // of type SharedState when dropping Rc
    _marker: PhantomData<SharedState<T>>,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let state = Box::new(SharedState {
            value: v,
            refcount: Cell::new(1),
        });
        Self { 
            // UNSAFE: Box does not give us a null pointer
            state: unsafe{ NonNull::new_unchecked(Box::into_raw(state)) },
            _marker: PhantomData,
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        // UNSAFE: shared state is not dropped as we still have a reference
        let state = unsafe { self.state.as_ref() };

        let refcount = state.refcount.get();
        state.refcount.set(refcount + 1);

        Self { 
            state: self.state,
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // UNSAFE: shared state is valid as we have a reference
        &unsafe {self.state.as_ref()}.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        // UNSAFE: shared state is valid as we have a reference
        let state = unsafe { self.state.as_ref()};
        let refcount = state.refcount.get();

        if refcount == 1 {
            // Drop reference
            drop(state);
            // If we use the following, we get:
            //
            // let _ = Box::from_raw(self.state);
            // if self.state = *const ...
            //
            // Found raw point *const, expected *mut
            // so we need to wrap it inside a NonNull

            // UNSAFE: We are converting from a non null pointer to a Box
            // so that we can drop
            let _ = unsafe{ Box::from_raw(self.state.as_ptr()) };
        } else {
            // There are other references, 
            // don't drop the Box
            state.refcount.set(refcount -1);
        }
        
    }
}

