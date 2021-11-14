use rt_map::BorrowFail;

use crate::{FnResMut, FnResource, Resources};

#[cfg(not(feature = "fn_res_once"))]
impl<Fun, Ret> FnResMut for FnResource<Fun, Ret, ()>
where
    Fun: FnMut() -> Ret + 'static,
    Ret: 'static,
{
    type Ret = Ret;

    fn call_mut(&mut self, resources: &Resources) -> Ret {
        Self::call_mut(self, resources)
    }

    fn try_call_mut(&mut self, resources: &Resources) -> Result<Ret, BorrowFail> {
        Self::try_call_mut(self, resources)
    }
}

#[cfg(feature = "fn_res_once")]
impl<Fun, Ret> FnResMut for FnResource<Fun, Ret, ()>
where
    Fun: FnMut() -> Ret + 'static,
    Ret: 'static,
{
    fn call_mut(&mut self, resources: &Resources) -> Ret {
        Self::call_mut(self, resources)
    }

    fn try_call_mut(&mut self, resources: &Resources) -> Result<Ret, BorrowFail> {
        Self::try_call_mut(self, resources)
    }
}

// Unfortunately we have to `include!` instead of use a `#[path]` attribute.
// Pending: <https://github.com/rust-lang/rust/issues/48250>
include!(concat!(env!("OUT_DIR"), "/fn_res_mut_impl.rs"));
