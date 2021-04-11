use std::{
    mem,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicUsize, Ordering},
};

/// A mutable reference to data in a `Cell`.
///
/// Access the value via `std::ops::DerefMut` (e.g. `*val`)
#[derive(Debug)]
pub struct CellRefMut<'a, T>
where
    T: ?Sized + 'a,
{
    pub flag: &'a AtomicUsize,
    pub value: &'a mut T,
}

impl<'a, T> CellRefMut<'a, T>
where
    T: ?Sized,
{
    /// Makes a new `CellRefMut` for a component of the borrowed data which
    /// preserves the existing borrow.
    ///
    /// The `Cell` is already mutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as
    /// `CellRefMut::map(...)`. A method would interfere with methods of the
    /// same name on the contents of a `CellRefMut` used through `DerefMut`.
    /// Further this preserves the borrow of the value and hence does the
    /// proper cleanup when it's dropped.
    ///
    /// # Examples
    ///
    /// This can also be used to avoid pointer indirection when a boxed item is
    /// stored in the `Cell`.
    ///
    /// ```
    /// use resman::{Cell, CellRefMut};
    ///
    /// let cb = Cell::new(Box::new(5));
    ///
    /// // Borrowing the cell causes the `CellRefMut` to store a reference to the `Box`, which is a
    /// // pointer to the value on the heap, and not a reference directly to the value.
    /// let boxed_ref: CellRefMut<'_, Box<usize>> = cb.borrow_mut();
    /// assert_eq!(**boxed_ref, 5); // Notice the double deref to get the actual value.
    ///
    /// // By using `map` we can let `CellRefMut` store a reference directly to the value on the heap.
    /// let pure_ref: CellRefMut<'_, usize> = CellRefMut::map(boxed_ref, Box::as_mut);
    ///
    /// assert_eq!(*pure_ref, 5);
    /// ```
    ///
    /// We can also use `map` to get a reference to a sub-part of the borrowed
    /// value.
    ///
    /// ```rust
    /// # use resman::{Cell, CellRefMut};
    ///
    /// let c = Cell::new((5, 'b'));
    ///
    /// let b1: CellRefMut<'_, (u32, char)> = c.borrow_mut();
    /// let b2: CellRefMut<'_, u32> = CellRefMut::map(b1, |t| &mut t.0);
    /// assert_eq!(*b2, 5);
    /// ```
    pub fn map<U, F>(self, f: F) -> CellRefMut<'a, U>
    where
        F: FnOnce(&mut T) -> &mut U,
        U: ?Sized,
    {
        let flag = unsafe { &*(self.flag as *const _) };
        let value = unsafe { &mut *(self.value as *mut _) };

        mem::forget(self);

        CellRefMut {
            flag,
            value: f(value),
        }
    }
}

impl<'a, T> Deref for CellRefMut<'a, T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.value
    }
}

impl<'a, T> DerefMut for CellRefMut<'a, T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<'a, T> Drop for CellRefMut<'a, T>
where
    T: ?Sized,
{
    fn drop(&mut self) {
        self.flag.store(0, Ordering::Release)
    }
}
