use rt_map::BorrowFail;

use crate::{FnRes, FnResource, Resources};

#[cfg(not(feature = "fn_res_mut"))]
impl<Fun, Ret> FnRes for FnResource<Fun, Ret, ()>
where
    Fun: Fn() -> Ret + 'static,
    Ret: 'static,
{
    type Ret = Ret;

    fn call(&self, resources: &Resources) -> Ret {
        Self::call(self, resources)
    }

    fn try_call(&self, resources: &Resources) -> Result<Ret, BorrowFail> {
        Self::try_call(self, resources)
    }
}

#[cfg(feature = "fn_res_mut")]
impl<Fun, Ret> FnRes for FnResource<Fun, Ret, ()>
where
    Fun: Fn() -> Ret + 'static,
    Ret: 'static,
{
    fn call(&self, resources: &Resources) -> Ret {
        Self::call(self, resources)
    }

    fn try_call(&self, resources: &Resources) -> Result<Ret, BorrowFail> {
        Self::try_call(self, resources)
    }
}

// Unfortunately we have to `include!` instead of use a `#[path]` attribute.
// Pending: <https://github.com/rust-lang/rust/issues/48250>
include!(concat!(env!("OUT_DIR"), "/fn_res_impl.rs"));
