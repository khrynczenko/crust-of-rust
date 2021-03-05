use crate::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Copy, Clone, PartialEq)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: crate::cell::Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        RefCell {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                // SAFETY: no exclusive references have been given out.
                //
                Some(Ref { refcell: self })
            }
            RefState::Shared(x) => {
                self.state.set(RefState::Shared(x + 1));
                // SAFETY: no exclusive references have been given out.
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if self.state.get() == RefState::Unshared {
            self.state.set(RefState::Exclusive);
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive => unreachable!(),
            RefState::Unshared => unreachable!(),
            RefState::Shared(1) => self.refcell.state.set(RefState::Unshared),
            RefState::Shared(x) => self.refcell.state.set(RefState::Shared(x - 1)),
        }
    }
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        // SAFETY: A ref is only created if no exclusive references have
        // been given out.
        unsafe { &*self.refcell.value.get() }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive => self.refcell.state.set(RefState::Unshared),
            RefState::Unshared => unreachable!(),
            RefState::Shared(_) => unreachable!(),
        }
    }
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        // SAFETY: A ref is only created if no exclusive references have
        // been given out.
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        // SAFETY: A ref is only created if no exclusive references have
        // been given out.
        unsafe { &mut *self.refcell.value.get() }
    }
}
