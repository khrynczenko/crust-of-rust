use crate::cell::Cell;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });

        Rc {
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData::default(),
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        inner.refcount.set(inner.refcount.get() + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData::default(),
        }
    }
}

impl<T> std::ops::Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let references = inner.refcount.get();
        if references == 1 {
            // SAFETY: We are the last reference to T so we should also
            // drop the heap allocated value.
            drop(inner);
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // SAFETY: There are more references existing so we just decrement
            // the reference count;
            inner.refcount.set(references - 1);
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a box t aht is deallocated when the last Rc
        // goes away. We have an Rc, therefore Box has not been deallocated,
        // so its safe to return the reference to the value pointed at.
        let inner = unsafe { self.inner.as_ref() };
        inner.refcount.set(inner.refcount.get() + 1);

        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> std::ops::DerefMut for Rc<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: self.inner is a box t aht is deallocated when the last Rc
        // goes away. We have an Rc, therefore Box has not been deallocated,
        // so its safe to return the reference to the value pointed at.
        let inner = unsafe { self.inner.as_ref() };
        inner.refcount.set(inner.refcount.get() + 1);

        &mut unsafe { self.inner.as_mut() }.value
    }
}
