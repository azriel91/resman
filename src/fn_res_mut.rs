use std::ops::DerefMut;

use rt_map::BorrowFail;

use crate::Resources;

/// Function that gets its arguments / parameters from a `Resources` map.
///
/// This allows consumers of this library to hold onto multiple *resource
/// functions* as `Box<dyn FnResMut>`, even though their arguments may be
/// different.
#[cfg(all(not(feature = "fn_res_once"), not(feature = "fn_meta")))]
pub trait FnResMut {
    /// Return type of the function.
    type Ret;

    /// Runs the function.
    fn call_mut(&mut self, resources: &Resources) -> Self::Ret;

    /// Runs the function.
    fn try_call_mut(&mut self, resources: &Resources) -> Result<Self::Ret, BorrowFail>;
}

/// Function that gets its arguments / parameters from a `Resources` map.
///
/// This allows consumers of this library to hold onto multiple *resource
/// functions* as `Box<dyn FnResMut>`, even though their arguments may be
/// different.
#[cfg(all(feature = "fn_res_once", not(feature = "fn_meta")))]
pub trait FnResMut: crate::FnResOnce {
    /// Runs the function.
    fn call_mut(&mut self, resources: &Resources) -> Self::Ret;

    /// Runs the function.
    fn try_call_mut(&mut self, resources: &Resources) -> Result<Self::Ret, BorrowFail>;
}

/// Function that gets its arguments / parameters from a `Resources` map.
///
/// This allows consumers of this library to hold onto multiple *resource
/// functions* as `Box<dyn FnResMut>`, even though their arguments may be
/// different.
#[cfg(all(not(feature = "fn_res_once"), feature = "fn_meta"))]
pub trait FnResMut: fn_meta::FnMeta + fn_meta::FnMetaDyn {
    /// Return type of the function.
    type Ret;

    /// Runs the function.
    fn call_mut(&mut self, resources: &Resources) -> Self::Ret;

    /// Runs the function.
    fn try_call_mut(&mut self, resources: &Resources) -> Result<Self::Ret, BorrowFail>;
}

/// Function that gets its arguments / parameters from a `Resources` map.
///
/// This allows consumers of this library to hold onto multiple *resource
/// functions* as `Box<dyn FnResMut>`, even though their arguments may be
/// different.
#[cfg(all(feature = "fn_res_once", feature = "fn_meta"))]
pub trait FnResMut: crate::FnResOnce + fn_meta::FnMeta + fn_meta::FnMetaDyn {
    /// Runs the function.
    fn call_mut(&mut self, resources: &Resources) -> Self::Ret;

    /// Runs the function.
    fn try_call_mut(&mut self, resources: &Resources) -> Result<Self::Ret, BorrowFail>;
}

#[cfg(all(not(feature = "fn_res_once"), not(feature = "fn_meta")))]
impl<T, Ret> FnResMut for Box<T>
where
    T: FnResMut<Ret = Ret>,
{
    type Ret = Ret;

    fn call_mut(&mut self, resources: &Resources) -> Self::Ret {
        self.deref_mut().call_mut(resources)
    }

    fn try_call_mut(&mut self, resources: &Resources) -> Result<Self::Ret, BorrowFail> {
        self.deref_mut().try_call_mut(resources)
    }
}

#[cfg(all(not(feature = "fn_res_once"), feature = "fn_meta"))]
impl<T, Ret> FnResMut for Box<T>
where
    T: FnResMut<Ret = Ret> + fn_meta::FnMeta + fn_meta::FnMetaDyn,
{
    type Ret = Ret;

    fn call_mut(&mut self, resources: &Resources) -> Self::Ret {
        self.deref_mut().call_mut(resources)
    }

    fn try_call_mut(&mut self, resources: &Resources) -> Result<Self::Ret, BorrowFail> {
        self.deref_mut().try_call_mut(resources)
    }
}

#[cfg(all(feature = "fn_res_once", not(feature = "fn_meta")))]
impl<T, Ret> FnResMut for Box<T>
where
    T: FnResMut<Ret = Ret>,
{
    fn call_mut(&mut self, resources: &Resources) -> Self::Ret {
        self.deref_mut().call_mut(resources)
    }

    fn try_call_mut(&mut self, resources: &Resources) -> Result<Self::Ret, BorrowFail> {
        self.deref_mut().try_call_mut(resources)
    }
}

#[cfg(all(feature = "fn_res_once", feature = "fn_meta"))]
impl<T, Ret> FnResMut for Box<T>
where
    T: FnResMut<Ret = Ret> + fn_meta::FnMeta + fn_meta::FnMetaDyn,
{
    fn call_mut(&mut self, resources: &Resources) -> Self::Ret {
        self.deref_mut().call_mut(resources)
    }

    fn try_call_mut(&mut self, resources: &Resources) -> Result<Self::Ret, BorrowFail> {
        self.deref_mut().try_call_mut(resources)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BorrowFail, FnResMut, IntoFnResource, Resources};

    #[cfg(feature = "fn_res_once")]
    use crate::FnResOnce;

    #[test]
    fn fn_res_mut_call_mut() {
        let mut resources = Resources::new();
        resources.insert(0u32);
        let mut fn_mut = fn_mut_fn();

        assert_eq!(2, fn_mut.call_mut(&resources));
        assert_eq!(1, *resources.borrow::<u32>());

        assert_eq!(3, fn_mut.call_mut(&resources));
        assert_eq!(3, *resources.borrow::<u32>());
    }

    #[test]
    fn fn_res_mut_try_call_mut() {
        let mut resources = Resources::new();
        resources.insert(0u32);
        let mut fn_mut = fn_mut_fn();

        assert_eq!(Ok(2), fn_mut.try_call_mut(&resources));
        assert_eq!(1, *resources.borrow::<u32>());

        assert_eq!(Ok(3), fn_mut.try_call_mut(&resources));
        assert_eq!(3, *resources.borrow::<u32>());

        let _borrow = resources.borrow::<u32>();
        assert_eq!(
            Err(BorrowFail::BorrowConflictMut),
            fn_mut.try_call_mut(&resources)
        );
    }

    #[cfg(feature = "fn_res_once")]
    #[test]
    fn fn_res_mut_call_once() {
        let mut resources = Resources::new();
        resources.insert(0u32);
        let fn_mut = fn_mut_fn();

        assert_eq!(2, fn_mut.call_once(&resources));
        assert_eq!(1, *resources.borrow::<u32>());
    }

    #[cfg(feature = "fn_res_once")]
    #[test]
    fn fn_res_mut_try_call_once() {
        let mut resources = Resources::new();
        resources.insert(0u32);
        let fn_mut = fn_mut_fn();

        assert_eq!(Ok(2), fn_mut.try_call_once(&resources));
        assert_eq!(1, *resources.borrow::<u32>());
    }

    // fn fn_mut_fn() -> Box<dyn FnResMut<Ret = u32>> {
    fn fn_mut_fn() -> impl FnResMut<Ret = u32> {
        let mut x = 1;
        (move |a: &mut u32| {
            *a += x;
            x += 1;

            x
        })
        .into_fn_resource()
    }
}
