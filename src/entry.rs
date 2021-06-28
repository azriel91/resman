use std::{any::TypeId, marker::PhantomData};

use crate::{RefMut, Resource};

pub struct Entry<'a, R> {
    inner: rt_map::Entry<'a, TypeId, Box<dyn Resource>>,
    marker: PhantomData<R>,
}

/// An entry to a resource container.
///
/// This is similar to the Entry API found in the standard library.
///
/// ## Examples
///
/// ```rust
/// use rt_map::RtMap;
///
/// #[derive(Debug)]
/// struct Res(i32);
///
/// let mut rt_map = RtMap::<u32, Res>::default();
///
/// let value = rt_map.entry(0).or_insert(Res(4));
/// println!("{:?}", value.0 * 2);
/// ```
impl<'a, R> Entry<'a, R>
where
    R: Resource,
{
    /// Create new entry.
    pub fn new(inner: rt_map::Entry<'a, TypeId, Box<dyn Resource>>) -> Self {
        Self {
            inner,
            marker: PhantomData,
        }
    }

    /// Returns this entry's value, inserts and returns `v` otherwise.
    ///
    /// Please note that you should use `or_insert_with` in case the creation of
    /// the value is expensive.
    pub fn or_insert(self, v: R) -> RefMut<'a, R> {
        self.or_insert_with(move || v)
    }

    /// Returns this entry's value, inserts and returns the return value of `f`
    /// otherwise.
    pub fn or_insert_with<F>(self, f: F) -> RefMut<'a, R>
    where
        F: FnOnce() -> R,
    {
        let inner = self.inner.or_insert_with(move || Box::new(f()));

        RefMut::new(inner)
    }
}
