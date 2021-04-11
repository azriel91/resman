use std::{any::TypeId, collections::HashMap, marker::PhantomData};

use crate::{Cell, CellRef, Entry, Ref, RefMut, Resource};

/// Map from `TypeId` to type.
#[derive(Default)]
pub struct Resources(HashMap<TypeId, Cell<Box<dyn Resource>>>);

macro_rules! fetch_panic {
    () => {
        panic!(
            "\
            Tried to fetch resource from the resources map, but the resource does not exist.\n\
            \n\
            Resource: `{resource_name_full}`\n\
            \n\
            You may ensure the resource exists!\
            ",
            resource_name_full = std::any::type_name::<R>(),
        )
    };
}

/// A [Resource] container, which provides methods to insert, access and manage
/// the contained resources.
///
/// Many methods take `&self` which works because everything
/// is stored with **interior mutability**. In case you violate
/// the borrowing rules of Rust (multiple reads xor one write),
/// you will get a panic.
///
/// # Resource Ids
///
/// Resources are identified by `TypeId`s, which consist of a `TypeId`.
impl Resources {
    /// Returns an entry for the resource with type `R`.
    pub fn entry<R>(&mut self) -> Entry<R>
    where
        R: Resource,
    {
        Entry::new(self.0.entry(TypeId::of::<R>()))
    }

    /// Inserts a resource into this container. If the resource existed before,
    /// it will be overwritten.
    ///
    /// # Examples
    ///
    /// Every type satisfying `Any + Send + Sync` automatically
    /// implements `Resource`, thus can be added:
    ///
    /// ```rust
    /// # #![allow(dead_code)]
    /// struct MyRes(i32);
    /// ```
    ///
    /// When you have a resource, simply insert it like this:
    ///
    /// ```rust
    /// # struct MyRes(i32);
    /// use resman::Resources;
    ///
    /// let mut resources = Resources::default();
    /// resources.insert(MyRes(5));
    /// ```
    pub fn insert<R>(&mut self, r: R)
    where
        R: Resource,
    {
        self.0.insert(TypeId::of::<R>(), Cell::new(Box::new(r)));
    }

    /// Removes a resource of type `R` from this container and returns its
    /// ownership to the caller. In case there is no such resource in this,
    /// container, `None` will be returned.
    ///
    /// Use this method with caution; other functions and systems might assume
    /// this resource still exists. Thus, only use this if you're sure no
    /// system will try to access this resource after you removed it (or else
    /// you will get a panic).
    pub fn remove<R>(&mut self) -> Option<R>
    where
        R: Resource,
    {
        self.0
            .remove(&TypeId::of::<R>())
            .map(Cell::into_inner)
            .map(|x: Box<dyn Resource>| x.downcast())
            .map(|x: Result<Box<R>, _>| x.ok().unwrap())
            .map(|x| *x)
    }

    /// Returns true if the specified resource type `R` exists in `self`.
    pub fn contains<R>(&self) -> bool
    where
        R: Resource,
    {
        self.0.contains_key(&TypeId::of::<R>())
    }

    /// Returns the `R` resource in the resource map.
    ///
    /// See [`try_borrow`] for a non-panicking version of this function.
    ///
    /// # Panics
    ///
    /// Panics if the resource doesn't exist.
    /// Panics if the resource is being accessed mutably.
    ///
    /// [`try_borrow`]: Self::try_borrow
    pub fn borrow<R>(&self) -> Ref<R>
    where
        R: Resource,
    {
        self.0
            .get(&TypeId::of::<R>())
            .map(|r| Ref {
                inner: CellRef::map(r.borrow(), Box::as_ref),
                phantom: PhantomData,
            })
            .unwrap_or_else(|| fetch_panic!())
    }

    /// Returns an immutable reference to `R` if it exists, `None` otherwise.
    pub fn try_borrow<R>(&self) -> Option<Ref<R>>
    where
        R: Resource,
    {
        self.0.get(&TypeId::of::<R>()).and_then(|r_cell| {
            r_cell.try_borrow().map(|r| Ref {
                inner: CellRef::map(r, Box::as_ref),
                phantom: PhantomData,
            })
        })
    }

    /// Returns a mutable reference to `R` if it exists, `None` otherwise.
    ///
    /// # Panics
    ///
    /// Panics if the resource doesn't exist.
    /// Panics if the resource is already accessed.
    pub fn borrow_mut<R>(&self) -> RefMut<R>
    where
        R: Resource,
    {
        self.0
            .get(&TypeId::of::<R>())
            .map(|r| RefMut {
                inner: r.borrow_mut().map(Box::as_mut),
                phantom: PhantomData,
            })
            .unwrap_or_else(|| fetch_panic!())
    }

    /// Returns a mutable reference to `R` if it exists, `None` otherwise.
    pub fn try_borrow_mut<R>(&self) -> Option<RefMut<R>>
    where
        R: Resource,
    {
        self.0.get(&TypeId::of::<R>()).and_then(|r_cell| {
            r_cell.try_borrow_mut().map(|ref_cell_mut| RefMut {
                inner: ref_cell_mut.map(Box::as_mut),
                phantom: PhantomData,
            })
        })
    }

    /// Retrieves a resource without fetching, which is cheaper, but only
    /// available with `&mut self`.
    pub fn get_mut<R: Resource>(&mut self) -> Option<&mut R> {
        self.get_resource_mut(TypeId::of::<R>())
            .map(|res| res.downcast_mut().unwrap())
    }

    /// Retrieves a resource without fetching, which is cheaper, but only
    /// available with `&mut self`.
    pub fn get_resource_mut(&mut self, id: TypeId) -> Option<&mut dyn Resource> {
        self.0.get_mut(&id).map(Cell::get_mut).map(Box::as_mut)
    }

    /// Get raw access to the underlying cell.
    pub fn get_raw(&self, id: &TypeId) -> Option<&Cell<Box<dyn Resource>>> {
        self.0.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::Resources;

    #[derive(Debug, Default, PartialEq)]
    struct Res;

    #[test]
    fn insert() {
        struct Foo;

        let mut resources = Resources::default();
        resources.insert(Res);

        assert!(resources.contains::<Res>());
        assert!(!resources.contains::<Foo>());
    }

    #[test]
    #[should_panic(expected = "but it was already borrowed")]
    fn read_write_fails() {
        let mut resources = Resources::default();
        resources.insert(Res);

        let _read = resources.borrow::<Res>();
        let _write = resources.borrow_mut::<Res>();
    }

    #[test]
    #[should_panic(expected = "but it was already borrowed mutably")]
    fn write_read_fails() {
        let mut resources = Resources::default();
        resources.insert(Res);

        let _write = resources.borrow_mut::<Res>();
        let _read = resources.borrow::<Res>();
    }

    #[test]
    fn remove_insert() {
        let mut resources = Resources::default();
        resources.insert(Res);

        assert!(resources.contains::<Res>());

        resources.remove::<Res>().unwrap();

        assert!(!resources.contains::<Res>());

        resources.insert(Res);

        assert!(resources.contains::<Res>());
    }

    #[test]
    fn borrow_mut_try_borrow_returns_none() {
        let mut resources = Resources::default();
        resources.insert(Res);

        let _res = resources.borrow_mut::<Res>();

        assert_eq!(None, resources.try_borrow::<Res>());
    }

    #[test]
    fn borrow_try_borrow_mut_returns_none() {
        let mut resources = Resources::default();
        resources.insert(Res);

        let _res = resources.borrow::<Res>();

        assert_eq!(None, resources.try_borrow_mut::<Res>());
    }

    #[test]
    fn borrow_mut_borrow_mut_returns_none() {
        let mut resources = Resources::default();
        resources.insert(Res);

        let _res = resources.borrow_mut::<Res>();

        assert_eq!(None, resources.try_borrow_mut::<Res>());
    }
}
