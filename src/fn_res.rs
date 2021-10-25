use rt_map::BorrowFail;

use crate::{FnResource, IntoFnResource, Resources};

/// Function that gets its arguments / parameters from a `Resources` map.
///
/// This allows consumers of this library to hold onto multiple *resource
/// functions* as `Box<dyn FnRes>`, even though their arguments may be
/// different.
pub trait FnRes {
    /// Return type of the function.
    type Ret;

    /// Runs the function.
    fn call<'f>(&self, resources: &Resources) -> Self::Ret;

    /// Runs the function.
    fn try_call<'f>(&self, resources: &Resources) -> Result<Self::Ret, BorrowFail>;
}

/// Extension to return `Box<dyn FnRes>` for a function.
pub trait IntoFnRes<Fun, Ret, Args> {
    /// Returns the function wrapped as a `Box<dyn FnRes>`.
    fn into_fn_res(self) -> Box<dyn FnRes<Ret = Ret>>;
}

impl<Fun, Ret> IntoFnRes<Fun, Ret, ()> for Fun
where
    Fun: FnRes<Ret = Ret> + 'static,
{
    fn into_fn_res(self) -> Box<dyn FnRes<Ret = Ret>> {
        Box::new(self)
    }
}

impl<Fun, Ret, A> IntoFnRes<Fun, Ret, (A,)> for Fun
where
    Fun: Fn(A) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    FnResource<Fun, Ret, (A,)>: FnRes<Ret = Ret>,
{
    fn into_fn_res(self) -> Box<dyn FnRes<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B> IntoFnRes<Fun, Ret, (A, B)> for Fun
where
    Fun: Fn(A, B) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    FnResource<Fun, Ret, (A, B)>: FnRes<Ret = Ret>,
{
    fn into_fn_res(self) -> Box<dyn FnRes<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B, C> IntoFnRes<Fun, Ret, (A, B, C)> for Fun
where
    Fun: Fn(A, B, C) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    FnResource<Fun, Ret, (A, B, C)>: FnRes<Ret = Ret>,
{
    fn into_fn_res(self) -> Box<dyn FnRes<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B, C, D> IntoFnRes<Fun, Ret, (A, B, C, D)> for Fun
where
    Fun: Fn(A, B, C, D) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    FnResource<Fun, Ret, (A, B, C, D)>: FnRes<Ret = Ret>,
{
    fn into_fn_res(self) -> Box<dyn FnRes<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B, C, D, E> IntoFnRes<Fun, Ret, (A, B, C, D, E)> for Fun
where
    Fun: Fn(A, B, C, D, E) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    FnResource<Fun, Ret, (A, B, C, D, E)>: FnRes<Ret = Ret>,
{
    fn into_fn_res(self) -> Box<dyn FnRes<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B, C, D, E, F> IntoFnRes<Fun, Ret, (A, B, C, D, E, F)> for Fun
where
    Fun: Fn(A, B, C, D, E, F) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    FnResource<Fun, Ret, (A, B, C, D, E, F)>: FnRes<Ret = Ret>,
{
    fn into_fn_res(self) -> Box<dyn FnRes<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B, C, D, E, F, G> IntoFnRes<Fun, Ret, (A, B, C, D, E, F, G)> for Fun
where
    Fun: Fn(A, B, C, D, E, F, G) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
    FnResource<Fun, Ret, (A, B, C, D, E, F, G)>: FnRes<Ret = Ret>,
{
    fn into_fn_res(self) -> Box<dyn FnRes<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

#[cfg(test)]
mod tests {
    use rt_map::BorrowFail;

    use super::IntoFnRes;
    use crate::Resources;

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
            f_w2_r2_w2_r1.into_fn_res(),
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
        resources.insert(S6(6));

        let sum = fn_reses
            .iter()
            .fold(0, |sum, fn_res| sum + fn_res.call(&resources));

        assert_eq!(9, resources.borrow::<S0>().0);
        assert_eq!(267, sum);
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
            f_w2_r2_w2_r1.into_fn_res(),
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
        resources.insert(S6(6));

        let sum = fn_reses.iter().try_fold(0, |sum, fn_res| {
            fn_res.try_call(&resources).map(|ret| sum + ret)
        })?;

        assert_eq!(9, resources.borrow::<S0>().0);
        assert_eq!(267, sum);

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
    #[derive(Debug)]
    struct S6(usize);
}
