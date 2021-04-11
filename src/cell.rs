use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{CellRef, CellRefMut};

macro_rules! borrow_panic {
    ($s:expr) => {{
        panic!(
            "Tried to fetch data of type {:?}, but it was already borrowed{}.",
            ::std::any::type_name::<T>(),
            $s,
        )
    }};
}

/// A custom cell container that is a `RefCell` with thread-safety.
#[derive(Debug)]
pub struct Cell<T> {
    flag: AtomicUsize,
    inner: UnsafeCell<T>,
}

impl<T> Cell<T> {
    /// Create a new cell, similar to `RefCell::new`
    pub fn new(inner: T) -> Self {
        Cell {
            flag: AtomicUsize::new(0),
            inner: UnsafeCell::new(inner),
        }
    }

    /// Consumes this cell and returns ownership of `T`.
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }

    /// Get an immutable reference to the inner data.
    ///
    /// Absence of write accesses is checked at run-time.
    ///
    /// # Panics
    ///
    /// This function will panic if there is a mutable reference to the data
    /// already in use.
    pub fn borrow(&self) -> CellRef<T> {
        if !self.check_flag_read() {
            borrow_panic!(" mutably");
        }

        CellRef {
            flag: &self.flag,
            value: unsafe { &*self.inner.get() },
        }
    }

    /// Get an immutable reference to the inner data.
    ///
    /// Absence of write accesses is checked at run-time. If access is not
    /// possible, `None` is returned.
    pub fn try_borrow(&self) -> Option<CellRef<T>> {
        if self.check_flag_read() {
            Some(CellRef {
                flag: &self.flag,
                value: unsafe { &*self.inner.get() },
            })
        } else {
            None
        }
    }

    /// Get a mutable reference to the inner data.
    ///
    /// Exclusive access is checked at run-time.
    ///
    /// # Panics
    ///
    /// This function will panic if there are any references to the data already
    /// in use.
    pub fn borrow_mut(&self) -> CellRefMut<T> {
        if !self.check_flag_write() {
            borrow_panic!("");
        }

        CellRefMut {
            flag: &self.flag,
            value: unsafe { &mut *self.inner.get() },
        }
    }

    /// Get a mutable reference to the inner data.
    ///
    /// Exclusive access is checked at run-time. If access is not possible,
    /// `None` is returned.
    pub fn try_borrow_mut(&self) -> Option<CellRefMut<T>> {
        if self.check_flag_write() {
            Some(CellRefMut {
                flag: &self.flag,
                value: unsafe { &mut *self.inner.get() },
            })
        } else {
            None
        }
    }

    /// Gets exclusive access to the inner value, bypassing the Cell.
    ///
    /// Exclusive access is checked at compile time.
    pub fn get_mut(&mut self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }

    /// Make sure we are allowed to aquire a read lock, and increment the read
    /// count by 1
    fn check_flag_read(&self) -> bool {
        loop {
            let val = self.flag.load(Ordering::Acquire);

            if val == usize::MAX {
                return false;
            }

            if self
                .flag
                .compare_exchange(val, val + 1, Ordering::AcqRel, Ordering::Acquire)
                == Ok(val)
            {
                return true;
            }
        }
    }

    /// Make sure we are allowed to aquire a write lock, and then set the write
    /// lock flag.
    fn check_flag_write(&self) -> bool {
        self.flag
            .compare_exchange(0, usize::MAX, Ordering::AcqRel, Ordering::Acquire)
            == Ok(0)
    }
}

unsafe impl<T> Sync for Cell<T> where T: Sync {}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::Cell;
    use crate::{CellRef, CellRefMut};

    #[test]
    fn allow_multiple_reads() {
        let cell = Cell::new(5);

        let a = cell.borrow();
        let b = cell.borrow();

        assert_eq!(10, *a + *b);
    }

    #[test]
    fn allow_clone_reads() {
        let cell = Cell::new(5);

        let a = cell.borrow();
        let b = a.clone();

        assert_eq!(10, *a + *b);
    }

    #[test]
    fn allow_single_write() {
        let cell = Cell::new(5);

        {
            let mut a = cell.borrow_mut();
            *a += 2;
            *a += 3;
        }

        assert_eq!(10, *cell.borrow());
    }

    #[test]
    #[should_panic(expected = "but it was already borrowed mutably")]
    fn panic_write_and_read() {
        let cell = Cell::new(5);

        let mut a = cell.borrow_mut();
        *a = 7;

        assert_eq!(7, *cell.borrow());
    }

    #[test]
    #[should_panic(expected = "but it was already borrowed")]
    fn panic_write_and_write() {
        let cell = Cell::new(5);

        let mut a = cell.borrow_mut();
        *a = 7;

        assert_eq!(7, *cell.borrow_mut());
    }

    #[test]
    #[should_panic(expected = "Tried to fetch data of type \"i32\", but it was already borrowed.")]
    fn panic_read_and_write() {
        let cell = Cell::new(5);

        let _a = cell.borrow();

        assert_eq!(7, *cell.borrow_mut());
    }

    #[test]
    fn try_write_and_read() {
        let cell = Cell::new(5);

        let mut a = cell.try_borrow_mut().unwrap();
        *a = 7;

        assert!(cell.try_borrow().is_none());

        *a = 8;
    }

    #[test]
    fn try_write_and_write() {
        let cell = Cell::new(5);

        let mut a = cell.try_borrow_mut().unwrap();
        *a = 7;

        assert!(cell.try_borrow_mut().is_none());

        *a = 8;
    }

    #[test]
    fn try_read_and_write() {
        let cell = Cell::new(5);

        let _a = cell.try_borrow().unwrap();

        assert!(cell.try_borrow_mut().is_none());
    }

    #[test]
    fn cloned_borrow_does_not_allow_write() {
        let cell = Cell::new(5);

        let a = cell.borrow();
        let b = a.clone();

        drop(a);

        assert!(cell.try_borrow_mut().is_none());
        assert_eq!(5, *b);
    }

    #[test]
    fn ref_with_non_sized() {
        let r: CellRef<'_, [i32]> = CellRef {
            flag: &AtomicUsize::new(1),
            value: &[2, 3, 4, 5][..],
        };

        assert_eq!(&*r, &[2, 3, 4, 5][..]);
    }

    #[test]
    fn ref_with_non_sized_clone() {
        let r: CellRef<'_, [i32]> = CellRef {
            flag: &AtomicUsize::new(1),
            value: &[2, 3, 4, 5][..],
        };
        let rr = r.clone();

        assert_eq!(&*r, &[2, 3, 4, 5][..]);
        assert_eq!(r.flag.load(Ordering::SeqCst), 2);

        assert_eq!(&*rr, &[2, 3, 4, 5][..]);
        assert_eq!(rr.flag.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn ref_with_trait_obj() {
        let ra: CellRef<'_, dyn std::any::Any> = CellRef {
            flag: &AtomicUsize::new(1),
            value: &2i32,
        };

        assert_eq!(ra.downcast_ref::<i32>().unwrap(), &2i32);
    }

    #[test]
    fn ref_mut_with_non_sized() {
        let mut r: CellRefMut<'_, [i32]> = CellRefMut {
            flag: &AtomicUsize::new(1),
            value: &mut [2, 3, 4, 5][..],
        };

        assert_eq!(&mut *r, &mut [2, 3, 4, 5][..]);
    }

    #[test]
    fn ref_mut_with_trait_obj() {
        let mut ra: CellRefMut<'_, dyn std::any::Any> = CellRefMut {
            flag: &AtomicUsize::new(1),
            value: &mut 2i32,
        };

        assert_eq!(ra.downcast_mut::<i32>().unwrap(), &mut 2i32);
    }

    #[test]
    fn ref_map_box() {
        let cell = Cell::new(Box::new(10));

        let r: CellRef<'_, Box<usize>> = cell.borrow();
        assert_eq!(&**r, &10);

        let rr: CellRef<'_, usize> = cell.borrow().map(Box::as_ref);
        assert_eq!(&*rr, &10);
    }

    #[test]
    fn ref_map_preserves_flag() {
        let cell = Cell::new(Box::new(10));

        let r: CellRef<'_, Box<usize>> = cell.borrow();
        assert_eq!(cell.flag.load(Ordering::SeqCst), 1);
        let _nr: CellRef<'_, usize> = r.map(Box::as_ref);
        assert_eq!(cell.flag.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn ref_map_retains_borrow() {
        let cell = Cell::new(Box::new(10));

        let _r: CellRef<'_, usize> = cell.borrow().map(Box::as_ref);
        assert_eq!(cell.flag.load(Ordering::SeqCst), 1);

        let _rr: CellRef<'_, usize> = cell.borrow().map(Box::as_ref);
        assert_eq!(cell.flag.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn ref_map_drops_borrow() {
        let cell = Cell::new(Box::new(10));

        let r: CellRef<'_, usize> = cell.borrow().map(Box::as_ref);

        assert_eq!(cell.flag.load(Ordering::SeqCst), 1);
        drop(r);
        assert_eq!(cell.flag.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn ref_mut_map_box() {
        let cell = Cell::new(Box::new(10));

        {
            let mut r: CellRefMut<'_, Box<usize>> = cell.borrow_mut();
            assert_eq!(&mut **r, &mut 10);
        }
        {
            let mut rr: CellRefMut<'_, usize> = cell.borrow_mut().map(Box::as_mut);
            assert_eq!(&mut *rr, &mut 10);
        }
    }

    #[test]
    fn ref_mut_map_preserves_flag() {
        let cell = Cell::new(Box::new(10));

        let r: CellRefMut<'_, Box<usize>> = cell.borrow_mut();
        assert_eq!(cell.flag.load(Ordering::SeqCst), std::usize::MAX);
        let _nr: CellRefMut<'_, usize> = r.map(Box::as_mut);
        assert_eq!(cell.flag.load(Ordering::SeqCst), std::usize::MAX);
    }

    #[test]
    #[should_panic(
        expected = "Tried to fetch data of type \"alloc::boxed::Box<usize>\", but it was already borrowed."
    )]
    fn ref_mut_map_retains_mut_borrow() {
        let cell = Cell::new(Box::new(10));

        let _rr: CellRefMut<'_, usize> = cell.borrow_mut().map(Box::as_mut);

        let _ = cell.borrow_mut();
    }

    #[test]
    fn ref_mut_map_drops_borrow() {
        let cell = Cell::new(Box::new(10));

        let r: CellRefMut<'_, usize> = cell.borrow_mut().map(Box::as_mut);

        assert_eq!(cell.flag.load(Ordering::SeqCst), std::usize::MAX);
        drop(r);
        assert_eq!(cell.flag.load(Ordering::SeqCst), 0);
    }
}
