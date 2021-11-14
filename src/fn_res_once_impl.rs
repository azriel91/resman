use rt_map::BorrowFail;

use crate::{FnResOnce, FnResource, Resources};

impl<Fun, Ret> FnResOnce for FnResource<Fun, Ret, ()>
where
    Fun: FnOnce() -> Ret + 'static,
    Ret: 'static,
{
    type Ret = Ret;

    fn call_once(self, resources: &Resources) -> Ret {
        Self::call_once(self, resources)
    }

    fn try_call_once(self, resources: &Resources) -> Result<Ret, BorrowFail> {
        Self::try_call_once(self, resources)
    }
}

// Unfortunately we have to `include!` instead of use a `#[path]` attribute.
// Pending: <https://github.com/rust-lang/rust/issues/48250>
include!(concat!(env!("OUT_DIR"), "/fn_res_once_impl.rs"));
