use std::{
    any::TypeId,
    fmt,
    ops::{Deref, DerefMut},
};

use rt_map::{BorrowFail, Cell, RtMap};

use crate::{Entry, Ref, RefMut, Resource};

/// Map from `TypeId` to type.
#[derive(Default)]
pub struct Resources(RtMap<TypeId, Box<dyn Resource>>);

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
    /// Creates an empty `Resources` map.
    ///
    /// The map is initially created with a capacity of 0, so it will not
    /// allocate until it is first inserted into.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use resman::Resources;
    /// let mut resources = Resources::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty `Resources` map with the specified capacity.
    ///
    /// The map will be able to hold at least capacity elements without
    /// reallocating. If capacity is 0, the map will not allocate.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use resman::Resources;
    /// let resources: Resources = Resources::with_capacity(10);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self(RtMap::with_capacity(capacity))
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// This number is a lower bound; the `Resources<K, V>` might be able to
    /// hold more, but is guaranteed to be able to hold at least this many.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use resman::Resources;
    /// let resources: Resources = Resources::with_capacity(100);
    /// assert!(resources.capacity() >= 100);
    /// ```
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Returns an entry for the resource with type `R`.
    pub fn entry<R>(&mut self) -> Entry<R>
    where
        R: Resource,
    {
        Entry::new(self.0.entry(TypeId::of::<R>()))
    }

    /// Inserts a resource into the map. If the resource existed before,
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
    /// # #[derive(Debug)]
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
        self.0.insert(TypeId::of::<R>(), Box::new(r));
    }

    /// Inserts an already boxed resource into the map.
    pub fn insert_raw(&mut self, type_id: TypeId, resource: Box<dyn Resource>) {
        self.0.insert(type_id, resource);
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
        self.try_borrow::<R>()
            .unwrap_or_else(Self::borrow_panic::<R, _>)
    }

    /// Returns an immutable reference to `R` if it exists, `None` otherwise.
    pub fn try_borrow<R>(&self) -> Result<Ref<R>, BorrowFail>
    where
        R: Resource,
    {
        self.0.try_borrow(&TypeId::of::<R>()).map(Ref::new)
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
        self.try_borrow_mut::<R>()
            .unwrap_or_else(Self::borrow_panic::<R, _>)
    }

    /// Returns a mutable reference to `R` if it exists, `None` otherwise.
    pub fn try_borrow_mut<R>(&self) -> Result<RefMut<R>, BorrowFail>
    where
        R: Resource,
    {
        self.0.try_borrow_mut(&TypeId::of::<R>()).map(RefMut::new)
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
        self.0.get_resource_mut(&id).map(|resource| &mut **resource)
    }

    /// Get raw access to the underlying cell.
    pub fn get_raw(&self, id: &TypeId) -> Option<&Cell<Box<dyn Resource>>> {
        self.0.get_raw(id)
    }

    fn borrow_panic<R, Ret>(borrow_fail: BorrowFail) -> Ret {
        let type_name = std::any::type_name::<R>();
        match borrow_fail {
            BorrowFail::ValueNotFound => {
                panic!(
                    "Expected to borrow `{type_name}`, but it does not exist.",
                    type_name = type_name
                )
            }
            BorrowFail::BorrowConflictImm => panic!(
                "Expected to borrow `{type_name}` immutably, but it was already borrowed mutably.",
                type_name = type_name
            ),
            BorrowFail::BorrowConflictMut => panic!(
                "Expected to borrow `{type_name}` mutably, but it was already borrowed mutably.",
                type_name = type_name
            ),
        }
    }
}

#[cfg(not(feature = "debug"))]
impl fmt::Debug for Resources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_map = f.debug_map();

        self.0.keys().for_each(|type_id| {
            let resource = &*self.0.borrow(type_id);
            let type_name = resource.as_ref().type_name();

            // At runtime, we are unable to determine if the resource is `Debug`.
            debug_map.entry(&type_name, &"..");
        });

        debug_map.finish()
    }
}

#[cfg(feature = "debug")]
impl fmt::Debug for Resources {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug_map = f.debug_map();

        self.0.keys().for_each(|type_id| {
            let resource = &*self.0.borrow(type_id);
            let type_name = resource.as_ref().type_name();

            debug_map.entry(&type_name, resource);
        });

        debug_map.finish()
    }
}

impl Deref for Resources {
    type Target = RtMap<TypeId, Box<dyn Resource>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Resources {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use std::any::TypeId;

    use super::Resources;
    use crate::BorrowFail;

    #[test]
    fn entry_or_insert_inserts_value() {
        #[derive(Debug, PartialEq)]
        struct A(usize);

        let mut resources = Resources::new();
        let mut a_ref = resources.entry::<A>().or_insert(A(1));

        assert_eq!(&A(1), &*a_ref);

        *a_ref = A(2);

        drop(a_ref);

        assert_eq!(&A(2), &*resources.borrow::<A>());
    }

    #[cfg(not(feature = "debug"))]
    #[test]
    fn debug_uses_placeholder_for_values() {
        let mut resources = Resources::new();

        resources.insert(1u32);
        resources.insert(2u64);

        let resources_dbg = format!("{:?}", resources);
        assert!(
            resources_dbg.contains(r#"u32: "..""#),
            r#"Expected `{}` to contain `u32: ".."`"#,
            resources_dbg
        );
        assert!(
            resources_dbg.contains(r#"u64: "..""#),
            r#"Expected `{}` to contain `u64: ".."`"#,
            resources_dbg
        );
    }

    #[cfg(feature = "debug")]
    #[test]
    fn debug_uses_debug_implementation_for_values() {
        let mut resources = Resources::new();

        resources.insert(1u32);
        resources.insert(2u64);

        let resources_dbg = format!("{:?}", resources);
        assert!(
            resources_dbg.contains(r#"u32: 1"#),
            r#"Expected `{}` to contain `u32: 1`"#,
            resources_dbg
        );
        assert!(
            resources_dbg.contains(r#"u64: 2"#),
            r#"Expected `{}` to contain `u64: 2`"#,
            resources_dbg
        );
    }

    #[test]
    fn with_capacity_reserves_enough_capacity() {
        let map = Resources::with_capacity(100);
        assert!(map.capacity() >= 100);
    }

    #[test]
    fn insert() {
        #[cfg_attr(feature = "debug", derive(Debug))]
        struct Foo;

        let mut resources = Resources::default();
        resources.insert(Res);

        assert!(resources.contains::<Res>());
        assert!(!resources.contains::<Foo>());
    }

    #[test]
    fn insert_raw() {
        #[cfg_attr(feature = "debug", derive(Debug))]
        struct Foo;

        let mut resources = Resources::default();
        resources.insert_raw(TypeId::of::<Res>(), Box::new(Res));

        assert!(resources.contains::<Res>());
        assert!(!resources.contains::<Foo>());
    }

    #[test]
    #[should_panic(
        expected = "Expected to borrow `resman::resources::tests::Res` mutably, but it was already borrowed mutably."
    )]
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
    #[should_panic(
        expected = "Expected to borrow `resman::resources::tests::Res`, but it does not exist."
    )]
    fn borrow_before_insert_panics() {
        let resources = Resources::default();

        resources.borrow::<Res>();
    }

    #[test]
    #[should_panic(
        expected = "Expected to borrow `resman::resources::tests::Res`, but it does not exist."
    )]
    fn borrow_mut_before_insert_panics() {
        let resources = Resources::default();

        resources.borrow_mut::<Res>();
    }

    #[test]
    fn borrow_mut_try_borrow_returns_err() {
        let mut resources = Resources::default();
        resources.insert(Res);

        let _res = resources.borrow_mut::<Res>();

        assert_eq!(
            Err(BorrowFail::BorrowConflictImm),
            resources.try_borrow::<Res>()
        );
    }

    #[test]
    fn borrow_try_borrow_mut_returns_err() {
        let mut resources = Resources::default();
        resources.insert(Res);

        let _res = resources.borrow::<Res>();

        assert_eq!(
            Err(BorrowFail::BorrowConflictMut),
            resources.try_borrow_mut::<Res>()
        );
    }

    #[test]
    fn borrow_mut_borrow_mut_returns_err() {
        let mut resources = Resources::default();
        resources.insert(Res);

        let _res = resources.borrow_mut::<Res>();

        assert_eq!(
            Err(BorrowFail::BorrowConflictMut),
            resources.try_borrow_mut::<Res>()
        );
    }

    #[test]
    fn get_mut_returns_ok() {
        let mut resources = Resources::default();
        resources.insert(Res);

        let _res = resources.get_mut::<Res>();

        assert!(resources.try_borrow_mut::<Res>().is_ok());
    }

    #[test]
    fn get_resource_mut_returns_some() {
        let mut resources = Resources::default();
        resources.insert(Res);

        assert!(resources.get_resource_mut(TypeId::of::<Res>()).is_some());
    }

    #[test]
    fn get_raw_returns_some() {
        let mut resources = Resources::default();
        resources.insert(Res);

        assert!(resources.get_raw(&TypeId::of::<Res>()).is_some());
    }

    #[derive(Debug, Default, PartialEq)]
    struct Res;
}
