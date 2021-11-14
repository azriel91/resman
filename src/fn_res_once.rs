use rt_map::BorrowFail;

use crate::Resources;

/// Function that gets its arguments / parameters from a `Resources` map.
///
/// This allows consumers of this library to hold onto multiple *resource
/// functions* as `Box<dyn FnResOnce>`, even though their arguments may be
/// different.
#[cfg(not(feature = "fn_meta"))]
pub trait FnResOnce {
    /// Return type of the function.
    type Ret;

    /// Runs the function.
    fn call_once(self, resources: &Resources) -> Self::Ret;

    /// Runs the function.
    fn try_call_once(self, resources: &Resources) -> Result<Self::Ret, BorrowFail>;
}

/// Function that gets its arguments / parameters from a `Resources` map.
///
/// This allows consumers of this library to hold onto multiple *resource
/// functions* as `Box<dyn FnResOnce>`, even though their arguments may be
/// different.
#[cfg(feature = "fn_meta")]
pub trait FnResOnce: fn_meta::FnMeta {
    /// Return type of the function.
    type Ret;

    /// Runs the function.
    fn call_once(self, resources: &Resources) -> Self::Ret;

    /// Runs the function.
    fn try_call_once(self, resources: &Resources) -> Result<Self::Ret, BorrowFail>;
}

impl<T, Ret> FnResOnce for Box<T>
where
    T: FnResOnce<Ret = Ret>,
{
    type Ret = Ret;

    fn call_once(self, resources: &Resources) -> Self::Ret {
        let fn_res_once = Box::into_inner(self);
        fn_res_once.call_once(resources)
    }

    fn try_call_once(self, resources: &Resources) -> Result<Self::Ret, BorrowFail> {
        let fn_res_once = Box::into_inner(self);
        fn_res_once.try_call_once(resources)
    }
}
