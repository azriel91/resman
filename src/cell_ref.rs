use std::{
    mem,
    ops::Deref,
    sync::atomic::{AtomicUsize, Ordering},
};

/// An immutable reference to data in a `Cell`.
///
/// Access the value via `std::ops::Deref` (e.g. `*val`)
#[derive(Debug)]
pub struct CellRef<'a, T>
where
    T: ?Sized + 'a,
{
    pub flag: &'a AtomicUsize,
    pub value: &'a T,
}

impl<'a, T> CellRef<'a, T>
where
    T: ?Sized,
{
    /// Makes a new `CellRef` for a component of the borrowed data which
    /// preserves the existing borrow.
    ///
    /// The `Cell` is already immutably borrowed, so this cannot fail.
    ///
    /// This is an associated function that needs to be used as
    /// `CellRef::map(...)`. A method would interfere with methods of the
    /// same name on the contents of a `CellRef` used through `Deref`.
    /// Further this preserves the borrow of the value and hence does the
    /// proper cleanup when it's dropped.
    ///
    /// # Examples
    ///
    /// This can be used to avoid pointer indirection when a boxed item is
    /// stored in the `Cell`.
    ///
    /// ```rust
    /// use resman::{Cell, CellRef};
    ///
    /// let cb = Cell::new(Box::new(5));
    ///
    /// // Borrowing the cell causes the `CellRef` to store a reference to the `Box`, which is a
    /// // pointer to the value on the heap, not the actual value.
    /// let boxed_ref: CellRef<'_, Box<usize>> = cb.borrow();
    /// assert_eq!(**boxed_ref, 5); // Notice the double deref to get the actual value.
    ///
    /// // By using `map` we can let `CellRef` store a reference directly to the value on the heap.
    /// let pure_ref: CellRef<'_, usize> = CellRef::map(boxed_ref, Box::as_ref);
    ///
    /// assert_eq!(*pure_ref, 5);
    /// ```
    ///
    /// We can also use `map` to get a reference to a sub-part of the borrowed
    /// value.
    ///
    /// ```rust
    /// # use resman::{Cell, CellRef};
    ///
    /// let c = Cell::new((5, 'b'));
    /// let b1: CellRef<'_, (u32, char)> = c.borrow();
    /// let b2: CellRef<'_, u32> = CellRef::map(b1, |t| &t.0);
    /// assert_eq!(*b2, 5);
    /// ```
    pub fn map<U, F>(self, f: F) -> CellRef<'a, U>
    where
        F: FnOnce(&T) -> &U,
        U: ?Sized,
    {
        let flag = unsafe { &*(self.flag as *const _) };
        let value = unsafe { &*(self.value as *const _) };

        mem::forget(self);

        CellRef {
            flag,
            value: f(value),
        }
    }
}

impl<'a, T> Deref for CellRef<'a, T>
where
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.value
    }
}

impl<'a, T> Drop for CellRef<'a, T>
where
    T: ?Sized,
{
    fn drop(&mut self) {
        self.flag.fetch_sub(1, Ordering::Release);
    }
}

impl<'a, T> Clone for CellRef<'a, T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        self.flag.fetch_add(1, Ordering::Release);

        CellRef {
            flag: self.flag,
            value: self.value,
        }
    }
}
