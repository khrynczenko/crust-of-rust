use std::cell::UnsafeCell;

pub struct Cell<T> {
    // We could do T: Copy here but in rust it  is idiomatic
    // to put bounds onyl where they are needed
    value: UnsafeCell<T>, // UnsafeCell is a special type that compiler has
                          // knowledge of and allows casting from shared to exclusive reference
}

//impl <T> !Sync for Cell<T> {}
// This syntax is in nightly only but UnsafeCell is !Sync so
// our Cell types is automatically !Sync
// If a type has !Sync implemented it means that there cannot be shared
// references to this type between different threads. To be exact this type
// can be referenced only within thread that it originated.

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know that no one else is concurrently mutating self.value
        // because !Sync is implemented.
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: Because the references to this type are shared only within
        // a one thread we can be sure that the self.value is not mutated/get
        // at the same time, hence this is safe.
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod tests {
    //use super::Cell;

    //fn bad() {
    //let x = std::sync::Arc::neW
    //}
}
