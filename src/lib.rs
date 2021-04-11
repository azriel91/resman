//! Runtime managed resource borrowing.
//!
//! This library provides a map that can store one of any type, as well as
//! mutable borrows to each type at the same time.
//!
//! **Note:** This implementation is extracted from [`shred`], with the
//! following differences:
//!
//! * Uses [`downcast-rs`] instead of [`mopa`] for downcasting types.
//! * Adds `Debug` and `PartialEq` implementations for borrow types when the
//!   resource type implements those traits.
//! * Returns `None` instead of panicking for `try_borrow*` functions when the
//!   resource is already borrowed.
//!
//! ## Usage
//!
//! Add the following to `Cargo.toml`
//!
//! ```toml
//! resman = "0.1.0"
//! ```
//!
//! In code:
//!
//! ```rust
//! use resman::Resources;
//!
//! struct A(u32);
//! struct B(u32);
//!
//! fn main() {
//!     let mut resources = Resources::default();
//!
//!     resources.insert(A(1));
//!     resources.insert(B(2));
//!
//!     // We can validly have two mutable borrows from the `Resources` map!
//!     let mut a = resources.borrow_mut::<A>();
//!     let mut b = resources.borrow_mut::<B>();
//!     a.0 = 2;
//!     b.0 = 3;
//!
//!     // We need to explicitly drop the A and B borrows, because they are runtime
//!     // managed borrows, and rustc doesn't know to drop them before the immutable
//!     // borrows after this.
//!     drop(a);
//!     drop(b);
//!
//!     // Multiple immutable borrows to the same resource are valid.
//!     let a_0 = resources.borrow::<A>();
//!     let _a_1 = resources.borrow::<A>();
//!     let b = resources.borrow::<B>();
//!
//!     println!("A: {}", a_0.0);
//!     println!("B: {}", b.0);
//!
//!     // Trying to mutably borrow a resource that is already borrowed (immutably
//!     // or mutably) returns `None`.
//!     let a_try_borrow_mut = resources.try_borrow_mut::<A>();
//!     let exists = if a_try_borrow_mut.is_some() {
//!         "Some(..)"
//!     } else {
//!         "None"
//!     };
//!     println!("a_try_borrow_mut: {}", exists); // prints "None"
//! }
//! ```
//!
//! ## See Also
//!
//! * [`anymap`]: Map of any type, without multiple mutable borrows.
//! * [`shred`]: Contains `Resources` type, plus a task dispatcher.
//!
//! [`anymap`]: https://github.com/chris-morgan/anymap
//! [`downcast-rs`]: https://github.com/marcianx/downcast-rs
//! [`mopa`]: https://github.com/chris-morgan/mopa
//! [`shred`]: https://github.com/amethyst/shred

use std::{
    cmp::PartialEq,
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub use crate::{
    cell::Cell, cell_ref::CellRef, cell_ref_mut::CellRefMut, entry::Entry, resource::Resource,
    resources::Resources,
};

pub struct Ref<'a, R: 'a> {
    inner: CellRef<'a, dyn Resource>,
    phantom: PhantomData<&'a R>,
}

impl<'a, R> Ref<'a, R> {
    pub fn new(inner: CellRef<'a, dyn Resource>) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<'a, R> Deref for Ref<'a, R>
where
    R: Resource,
{
    type Target = R;

    fn deref(&self) -> &R {
        (*self.inner)
            .downcast_ref::<R>()
            .unwrap_or_else(|| panic!("Failed to downcast to {}", std::any::type_name::<R>()))
    }
}

impl<'a, R> fmt::Debug for Ref<'a, R>
where
    R: Resource + fmt::Debug + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner: &R = &*self;
        f.debug_struct("Ref").field("inner", inner).finish()
    }
}

impl<'a, R> PartialEq for Ref<'a, R>
where
    R: Resource + PartialEq + 'a,
{
    fn eq(&self, other: &Self) -> bool {
        let r_self: &R = &*self;
        let r_other: &R = &*other;
        r_self == r_other
    }
}

impl<'a, R> Clone for Ref<'a, R> {
    fn clone(&self) -> Self {
        Ref {
            inner: self.inner.clone(),
            phantom: PhantomData,
        }
    }
}

pub struct RefMut<'a, R: 'a> {
    inner: CellRefMut<'a, dyn Resource>,
    phantom: PhantomData<&'a R>,
}

impl<'a, R> fmt::Debug for RefMut<'a, R>
where
    R: Resource + fmt::Debug + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner: &R = &*self;
        f.debug_struct("RefMut").field("inner", inner).finish()
    }
}

impl<'a, R> PartialEq for RefMut<'a, R>
where
    R: Resource + PartialEq + 'a,
{
    fn eq(&self, other: &Self) -> bool {
        let r_self: &R = &*self;
        let r_other: &R = &*other;
        r_self == r_other
    }
}

impl<'a, R> RefMut<'a, R> {
    pub fn new(inner: CellRefMut<'a, dyn Resource>) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<'a, R> Deref for RefMut<'a, R>
where
    R: Resource,
{
    type Target = R;

    fn deref(&self) -> &R {
        self.inner
            .downcast_ref::<R>()
            .unwrap_or_else(|| panic!("Failed to downcast to {}", std::any::type_name::<R>()))
    }
}

impl<'a, R> DerefMut for RefMut<'a, R>
where
    R: Resource,
{
    fn deref_mut(&mut self) -> &mut R {
        self.inner
            .downcast_mut::<R>()
            .unwrap_or_else(|| panic!("Failed to downcast to {}", std::any::type_name::<R>()))
    }
}

mod cell;
mod cell_ref;
mod cell_ref_mut;
mod entry;
mod resource;
mod resources;
