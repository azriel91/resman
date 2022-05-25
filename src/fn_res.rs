use std::ops::Deref;

use rt_map::BorrowFail;

use crate::Resources;

/// Function that gets its arguments / parameters from a `Resources` map.
///
/// This allows consumers of this library to hold onto multiple *resource
/// functions* as `Box<dyn FnRes>`, even though their arguments may be
/// different.
#[cfg(all(not(feature = "fn_res_mut"), not(feature = "fn_meta")))]
pub trait FnRes {
    /// Return type of the function.
    type Ret;

    /// Runs the function.
    fn call(&self, resources: &Resources) -> Self::Ret;

    /// Runs the function.
    fn try_call(&self, resources: &Resources) -> Result<Self::Ret, BorrowFail>;
}

/// Function that gets its arguments / parameters from a `Resources` map.
///
/// This allows consumers of this library to hold onto multiple *resource
/// functions* as `Box<dyn FnRes>`, even though their arguments may be
/// different.
#[cfg(all(feature = "fn_res_mut", not(feature = "fn_meta")))]
pub trait FnRes: crate::FnResMut {
    /// Runs the function.
    fn call(&self, resources: &Resources) -> Self::Ret;

    /// Runs the function.
    fn try_call(&self, resources: &Resources) -> Result<Self::Ret, BorrowFail>;
}

/// Function that gets its arguments / parameters from a `Resources` map.
///
/// This allows consumers of this library to hold onto multiple *resource
/// functions* as `Box<dyn FnRes>`, even though their arguments may be
/// different.
#[cfg(all(not(feature = "fn_res_mut"), feature = "fn_meta"))]
pub trait FnRes: fn_meta::FnMeta + fn_meta::FnMetaDyn {
    /// Return type of the function.
    type Ret;

    /// Runs the function.
    fn call(&self, resources: &Resources) -> Self::Ret;

    /// Runs the function.
    fn try_call(&self, resources: &Resources) -> Result<Self::Ret, BorrowFail>;
}

/// Function that gets its arguments / parameters from a `Resources` map.
///
/// This allows consumers of this library to hold onto multiple *resource
/// functions* as `Box<dyn FnRes>`, even though their arguments may be
/// different.
#[cfg(all(feature = "fn_res_mut", feature = "fn_meta"))]
pub trait FnRes: crate::FnResMut + fn_meta::FnMeta + fn_meta::FnMetaDyn {
    /// Runs the function.
    fn call(&self, resources: &Resources) -> Self::Ret;

    /// Runs the function.
    fn try_call(&self, resources: &Resources) -> Result<Self::Ret, BorrowFail>;
}

#[cfg(not(feature = "fn_res_mut"))]
impl<T, Ret> FnRes for Box<T>
where
    T: FnRes<Ret = Ret>,
{
    type Ret = Ret;

    fn call(&self, resources: &Resources) -> Self::Ret {
        self.deref().call(resources)
    }

    fn try_call(&self, resources: &Resources) -> Result<Self::Ret, BorrowFail> {
        self.deref().try_call(resources)
    }
}

#[cfg(feature = "fn_res_mut")]
impl<T, Ret> FnRes for Box<T>
where
    T: FnRes<Ret = Ret>,
{
    fn call(&self, resources: &Resources) -> Self::Ret {
        self.deref().call(resources)
    }

    fn try_call(&self, resources: &Resources) -> Result<Self::Ret, BorrowFail> {
        self.deref().try_call(resources)
    }
}

#[cfg(test)]
mod tests {
    use rt_map::BorrowFail;

    use crate::{IntoFnRes, Resources};

    #[test]
    fn multiple_fn_usage() {
        let fn_reses = [
            f_r1.into_fn_res(),
            f_r2.into_fn_res(),
            f_r3.into_fn_res(),
            f_r4.into_fn_res(),
            f_r5.into_fn_res(),
            f_r6.into_fn_res(),
            f_w1.into_fn_res(),
            f_w2.into_fn_res(),
            f_w3.into_fn_res(),
            f_w4.into_fn_res(),
            f_w5.into_fn_res(),
            f_w6.into_fn_res(),
            f_r1_w1.into_fn_res(),
            f_w1_r1.into_fn_res(),
            f_r1_w1_r1.into_fn_res(),
            f_w1_r1_w1.into_fn_res(),
            // closure
            (|s0: &S0, s1: &mut S1| {
                s1.0 += 1;
                s0.0 + s1.0
            })
            .into_fn_res(),
        ];

        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));
        resources.insert(S2(2));
        resources.insert(S3(3));
        resources.insert(S4(4));
        resources.insert(S5(5));

        let sum = fn_reses
            .iter()
            .fold(0, |sum, fn_res| sum + fn_res.call(&resources));

        assert_eq!(8, resources.borrow::<S0>().0);
        assert_eq!(214, sum);
    }

    #[test]
    fn try_call_no_overlap_returns_ok() -> Result<(), BorrowFail> {
        let fn_reses = [
            f_r1.into_fn_res(),
            f_r2.into_fn_res(),
            f_r3.into_fn_res(),
            f_r4.into_fn_res(),
            f_r5.into_fn_res(),
            f_r6.into_fn_res(),
            f_w1.into_fn_res(),
            f_w2.into_fn_res(),
            f_w3.into_fn_res(),
            f_w4.into_fn_res(),
            f_w5.into_fn_res(),
            f_w6.into_fn_res(),
            f_r1_w1.into_fn_res(),
            f_w1_r1.into_fn_res(),
            f_r1_w1_r1.into_fn_res(),
            f_w1_r1_w1.into_fn_res(),
            // closures
            // zero args
            (|| 0usize).into_fn_res(),
            // two args
            (|s0: &S0, s1: &mut S1| {
                s1.0 += 1;
                s0.0 + s1.0
            })
            .into_fn_res(),
        ];

        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));
        resources.insert(S2(2));
        resources.insert(S3(3));
        resources.insert(S4(4));
        resources.insert(S5(5));

        let sum = fn_reses.iter().try_fold(0, |sum, fn_res| {
            fn_res.try_call(&resources).map(|ret| sum + ret)
        })?;

        assert_eq!(8, resources.borrow::<S0>().0);
        assert_eq!(214, sum);

        Ok(())
    }

    #[test]
    fn try_call_with_overlap_returns_borrow_fail() -> Result<(), BorrowFail> {
        let fn_reses = [
            f_r1.into_fn_res(),
            (|s0: &S0, s1: &mut S1| {
                s1.0 += 1;
                s0.0 + s1.0
            })
            .into_fn_res(),
        ];

        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));

        let _s1_borrow = resources.borrow::<S1>();

        let result = fn_reses.iter().try_fold(0, |sum, fn_res| {
            fn_res.try_call(&resources).map(|ret| sum + ret)
        });

        assert_eq!(Err(BorrowFail::BorrowConflictMut), result);

        Ok(())
    }

    #[cfg(feature = "fn_meta")]
    #[test]
    fn fn_meta_integration() {
        use std::any::TypeId;

        let fn_res = f_w1_r1_w1.into_fn_res();
        let borrows = fn_res.borrows();
        let borrow_muts = fn_res.borrow_muts();

        assert_eq!(&[TypeId::of::<S1>()], borrows.as_slice());
        assert_eq!(
            &[TypeId::of::<S0>(), TypeId::of::<S2>(),],
            borrow_muts.as_slice()
        );
    }

    #[cfg(all(feature = "fn_meta", feature = "high_arg_count"))]
    #[test]
    fn fn_meta_integration_high_arg_count() {
        use std::any::TypeId;

        let fn_res = f_w2_r2_w2_r1.into_fn_res();
        let borrows = fn_res.borrows();
        let borrow_muts = fn_res.borrow_muts();

        assert_eq!(
            &[TypeId::of::<S2>(), TypeId::of::<S3>(), TypeId::of::<S6>()],
            borrows.as_slice()
        );
        assert_eq!(
            &[
                TypeId::of::<S0>(),
                TypeId::of::<S1>(),
                TypeId::of::<S4>(),
                TypeId::of::<S5>()
            ],
            borrow_muts.as_slice()
        );
    }

    fn f_r1(s0: &S0) -> usize {
        s0.0
    }
    fn f_r2(s0: &S0, s1: &S1) -> usize {
        s0.0 + s1.0
    }
    fn f_r3(s0: &S0, s1: &S1, s2: &S2) -> usize {
        s0.0 + s1.0 + s2.0
    }
    fn f_r4(s0: &S0, s1: &S1, s2: &S2, s3: &S3) -> usize {
        s0.0 + s1.0 + s2.0 + s3.0
    }
    fn f_r5(s0: &S0, s1: &S1, s2: &S2, s3: &S3, s4: &S4) -> usize {
        s0.0 + s1.0 + s2.0 + s3.0 + s4.0
    }
    fn f_r6(s0: &S0, s1: &S1, s2: &S2, s3: &S3, s4: &S4, s5: &S5) -> usize {
        s0.0 + s1.0 + s2.0 + s3.0 + s4.0 + s5.0
    }

    fn f_w1(s0: &mut S0) -> usize {
        s0.0 += 1;

        s0.0
    }
    fn f_w2(s0: &mut S0, s1: &mut S1) -> usize {
        s0.0 += 1;
        s1.0 += 1;

        s0.0 + s1.0
    }
    fn f_w3(s0: &mut S0, s1: &mut S1, s2: &mut S2) -> usize {
        s0.0 += 1;
        s1.0 += 1;
        s2.0 += 1;

        s0.0 + s1.0 + s2.0
    }
    fn f_w4(s0: &mut S0, s1: &mut S1, s2: &mut S2, s3: &mut S3) -> usize {
        s0.0 += 1;
        s1.0 += 1;
        s2.0 += 1;
        s3.0 += 1;

        s0.0 + s1.0 + s2.0 + s3.0
    }
    fn f_w5(s0: &mut S0, s1: &mut S1, s2: &mut S2, s3: &mut S3, s4: &mut S4) -> usize {
        s0.0 += 1;
        s1.0 += 1;
        s2.0 += 1;
        s3.0 += 1;
        s4.0 += 1;

        s0.0 + s1.0 + s2.0 + s3.0 + s4.0
    }
    fn f_w6(s0: &mut S0, s1: &mut S1, s2: &mut S2, s3: &mut S3, s4: &mut S4, s5: &mut S5) -> usize {
        s0.0 += 1;
        s1.0 += 1;
        s2.0 += 1;
        s3.0 += 1;
        s4.0 += 1;
        s5.0 += 1;

        s0.0 + s1.0 + s2.0 + s3.0 + s4.0 + s5.0
    }

    fn f_r1_w1(s0: &S0, s1: &mut S1) -> usize {
        s1.0 += 1;

        s0.0 + s1.0
    }
    fn f_w1_r1(s0: &mut S0, s1: &S1) -> usize {
        s0.0 += 1;

        s0.0 + s1.0
    }
    fn f_r1_w1_r1(s0: &S0, s1: &mut S1, s2: &S2) -> usize {
        s1.0 += 1;

        s0.0 + s1.0 + s2.0
    }
    fn f_w1_r1_w1(s0: &mut S0, s1: &S1, s2: &mut S2) -> usize {
        s0.0 += 1;
        s2.0 += 1;

        s0.0 + s1.0 + s2.0
    }
    #[cfg(feature = "high_arg_count")]
    fn f_w2_r2_w2_r1(
        s0: &mut S0,
        s1: &mut S1,
        s2: &S2,
        s3: &S3,
        s4: &mut S4,
        s5: &mut S5,
        s6: &S6,
    ) -> usize {
        s0.0 += 1;
        s1.0 += 1;
        s4.0 += 1;
        s5.0 += 1;

        s0.0 + s1.0 + s2.0 + s3.0 + s4.0 + s5.0 + s6.0
    }

    #[derive(Debug)]
    struct S0(usize);
    #[derive(Debug)]
    struct S1(usize);
    #[derive(Debug)]
    struct S2(usize);
    #[derive(Debug)]
    struct S3(usize);
    #[derive(Debug)]
    struct S4(usize);
    #[derive(Debug)]
    struct S5(usize);
    #[cfg(feature = "high_arg_count")]
    #[derive(Debug)]
    struct S6(usize);
}
