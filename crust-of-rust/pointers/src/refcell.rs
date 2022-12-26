use std::cell::UnsafeCell;
use crate::cell::Cell;

#[derive(Copy, Clone)]
enum RefState {
    UnShared,
    Shared(usize),
    Exclusive
}
pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

// impl<T> !Sync for Cell<T> {}
//
// But, this is impled by UnsafeCell, as it is !Sync

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::UnShared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::UnShared => {
                self.state.set(RefState::Shared(1));
                Some(Ref { refcell: self })
            },
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n+1));
                Some(Ref { refcell: self })
            },
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>>
    where
        T: Copy
    {
        if let RefState::UnShared = self.state.get() {
            self.state.set(RefState::Exclusive);
            Some( RefMut{ refcell: self })
        } else {
            None 
        }
    }
}

// NOTE: We can get away with this signature for borrow and borrow_mut:
//    pub fn borrow(&self) -> Option<&T> 
//    pub fn borrow_mut(&self) -> Option<&mut T>
//
// We can increase the count of references or set them,
// but we are not decreasing or resetting them? How does that work?
//
// That is why we need the following


// Here Ref is pointer to an inner type that has additional sematics when we 
// drop it. Which is basically what a smart pointer is. 
pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>
}

// This is how Ref is a pointer to the value within
// Anytime we implement Deref, we can use the `.` operator
impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: 
        // a Ref is created if no Exlusive reference has been given out
        // once it is given out state is set to Shared, so no Exclusive is given out
        // So deref into a shared ref is fine
        unsafe{& *self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::UnShared | RefState::Exclusive => unreachable!(),
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::UnShared);
            },
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n-1));
            },
        }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: 
        // see deref_mut
        unsafe{& *self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: 
        // a RefMut is created if no Exlusive reference has been given out
        // once it is given out state is set to Exclusive, so no future
        // references are given out
        // So mutable deref fine
        unsafe{&mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::UnShared | RefState::Shared(_) => unreachable!(),
            RefState::Exclusive => {
                self.refcell.state.set(RefState::UnShared);
            },
        }
    }
}

