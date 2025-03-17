use core::marker::PhantomData;

use rt_map::BorrowFail;

use crate::Resources;

/// Function that gets its arguments / parameters from a `Resources` map.
pub struct FnResource<Fun, Ret, Args> {
    /// The actual function.
    pub func: Fun,
    /// Marker.
    marker: PhantomData<(Fun, Ret, Args)>,
}

#[cfg(feature = "fn_res_once")]
impl<Fun, Ret> FnResource<Fun, Ret, ()>
where
    Fun: FnOnce() -> Ret,
{
    pub fn call_once(self, _resources: &Resources) -> Ret {
        (self.func)()
    }

    pub fn try_call_once(self, _resources: &Resources) -> Result<Ret, BorrowFail> {
        let ret_value = (self.func)();
        Ok(ret_value)
    }
}

#[cfg(feature = "fn_res_mut")]
impl<Fun, Ret> FnResource<Fun, Ret, ()>
where
    Fun: FnMut() -> Ret,
{
    pub fn call_mut(&mut self, _resources: &Resources) -> Ret {
        (self.func)()
    }

    pub fn try_call_mut(&mut self, _resources: &Resources) -> Result<Ret, BorrowFail> {
        let ret_value = (self.func)();
        Ok(ret_value)
    }
}

impl<Fun, Ret> FnResource<Fun, Ret, ()>
where
    Fun: Fn() -> Ret,
{
    pub fn call(&self, _resources: &Resources) -> Ret {
        (self.func)()
    }

    pub fn try_call(&self, _resources: &Resources) -> Result<Ret, BorrowFail> {
        let ret_value = (self.func)();
        Ok(ret_value)
    }
}

#[cfg(feature = "fn_meta")]
impl<Fun, Ret> fn_meta::FnMeta for FnResource<Fun, Ret, ()>
where
    Fun: FnOnce() -> Ret,
{
    fn borrows() -> fn_meta::TypeIds {
        <fn_meta::FnMetadata<Fun, Ret, ()> as fn_meta::FnMeta>::borrows()
    }

    fn borrow_muts() -> fn_meta::TypeIds {
        <fn_meta::FnMetadata<Fun, Ret, ()> as fn_meta::FnMeta>::borrow_muts()
    }
}

#[cfg(feature = "fn_meta")]
impl<Fun, Ret> fn_meta::FnMetaDyn for FnResource<Fun, Ret, ()>
where
    Fun: FnOnce() -> Ret,
{
    fn borrows(&self) -> fn_meta::TypeIds {
        fn_meta::FnMetaExt::meta(&self.func).borrows()
    }

    fn borrow_muts(&self) -> fn_meta::TypeIds {
        fn_meta::FnMetaExt::meta(&self.func).borrow_muts()
    }
}

/// Extension to return [`FnResource`] for a function.
pub trait IntoFnResource<Fun, Ret, Args> {
    /// Returns the function wrapped as a `FnResource`.
    fn into_fn_resource(self) -> FnResource<Fun, Ret, Args>;
}

impl<Fun, Ret> IntoFnResource<Fun, Ret, ()> for Fun
where
    Fun: FnOnce() -> Ret,
{
    fn into_fn_resource(self) -> FnResource<Fun, Ret, ()> {
        FnResource {
            func: self,
            marker: PhantomData,
        }
    }
}

impl<Fun, Ret, A> IntoFnResource<Fun, Ret, (A,)> for Fun
where
    Fun: FnOnce(A) -> Ret,
{
    fn into_fn_resource(self) -> FnResource<Fun, Ret, (A,)> {
        FnResource {
            func: self,
            marker: PhantomData,
        }
    }
}

impl<Fun, Ret, A, B> IntoFnResource<Fun, Ret, (A, B)> for Fun
where
    Fun: FnOnce(A, B) -> Ret,
{
    fn into_fn_resource(self) -> FnResource<Fun, Ret, (A, B)> {
        FnResource {
            func: self,
            marker: PhantomData,
        }
    }
}

impl<Fun, Ret, A, B, C> IntoFnResource<Fun, Ret, (A, B, C)> for Fun
where
    Fun: FnOnce(A, B, C) -> Ret,
{
    fn into_fn_resource(self) -> FnResource<Fun, Ret, (A, B, C)> {
        FnResource {
            func: self,
            marker: PhantomData,
        }
    }
}

impl<Fun, Ret, A, B, C, D> IntoFnResource<Fun, Ret, (A, B, C, D)> for Fun
where
    Fun: FnOnce(A, B, C, D) -> Ret,
{
    fn into_fn_resource(self) -> FnResource<Fun, Ret, (A, B, C, D)> {
        FnResource {
            func: self,
            marker: PhantomData,
        }
    }
}

impl<Fun, Ret, A, B, C, D, E> IntoFnResource<Fun, Ret, (A, B, C, D, E)> for Fun
where
    Fun: FnOnce(A, B, C, D, E) -> Ret,
{
    fn into_fn_resource(self) -> FnResource<Fun, Ret, (A, B, C, D, E)> {
        FnResource {
            func: self,
            marker: PhantomData,
        }
    }
}

impl<Fun, Ret, A, B, C, D, E, F> IntoFnResource<Fun, Ret, (A, B, C, D, E, F)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F) -> Ret,
{
    fn into_fn_resource(self) -> FnResource<Fun, Ret, (A, B, C, D, E, F)> {
        FnResource {
            func: self,
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "high_arg_count")]
impl<Fun, Ret, A, B, C, D, E, F, G> IntoFnResource<Fun, Ret, (A, B, C, D, E, F, G)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F, G) -> Ret,
{
    #[allow(clippy::type_complexity)]
    fn into_fn_resource(self) -> FnResource<Fun, Ret, (A, B, C, D, E, F, G)> {
        FnResource {
            func: self,
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "high_arg_count")]
impl<Fun, Ret, A, B, C, D, E, F, G, H> IntoFnResource<Fun, Ret, (A, B, C, D, E, F, G, H)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F, G, H) -> Ret,
{
    #[allow(clippy::type_complexity)]
    fn into_fn_resource(self) -> FnResource<Fun, Ret, (A, B, C, D, E, F, G, H)> {
        FnResource {
            func: self,
            marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{IntoFnResource, Resources};

    #[test]
    fn read_1() {
        let fn_res = f_r1.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(1));

        let sum = fn_res.call(&resources);

        assert_eq!(1, resources.borrow::<S0>().0);
        assert_eq!(1, sum);
    }

    #[test]
    fn read_2() {
        let fn_res = f_r2.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));

        let sum = fn_res.call(&resources);

        assert_eq!(0, resources.borrow::<S0>().0);
        assert_eq!(1, resources.borrow::<S1>().0);
        assert_eq!(1, sum);
    }

    #[test]
    fn read_3() {
        let fn_res = f_r3.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));
        resources.insert(S2(2));

        let sum = fn_res.call(&resources);

        assert_eq!(0, resources.borrow::<S0>().0);
        assert_eq!(1, resources.borrow::<S1>().0);
        assert_eq!(2, resources.borrow::<S2>().0);
        assert_eq!(3, sum);
    }

    #[test]
    fn read_4() {
        let fn_res = f_r4.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));
        resources.insert(S2(2));
        resources.insert(S3(3));

        let sum = fn_res.call(&resources);

        assert_eq!(0, resources.borrow::<S0>().0);
        assert_eq!(1, resources.borrow::<S1>().0);
        assert_eq!(2, resources.borrow::<S2>().0);
        assert_eq!(3, resources.borrow::<S3>().0);
        assert_eq!(6, sum);
    }

    #[test]
    fn read_5() {
        let fn_res = f_r5.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));
        resources.insert(S2(2));
        resources.insert(S3(3));
        resources.insert(S4(4));

        let sum = fn_res.call(&resources);

        assert_eq!(0, resources.borrow::<S0>().0);
        assert_eq!(1, resources.borrow::<S1>().0);
        assert_eq!(2, resources.borrow::<S2>().0);
        assert_eq!(3, resources.borrow::<S3>().0);
        assert_eq!(4, resources.borrow::<S4>().0);
        assert_eq!(10, sum);
    }

    #[test]
    fn read_6() {
        let fn_res = f_r6.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));
        resources.insert(S2(2));
        resources.insert(S3(3));
        resources.insert(S4(4));
        resources.insert(S5(5));

        let sum = fn_res.call(&resources);

        assert_eq!(0, resources.borrow::<S0>().0);
        assert_eq!(1, resources.borrow::<S1>().0);
        assert_eq!(2, resources.borrow::<S2>().0);
        assert_eq!(3, resources.borrow::<S3>().0);
        assert_eq!(4, resources.borrow::<S4>().0);
        assert_eq!(5, resources.borrow::<S5>().0);
        assert_eq!(15, sum);
    }

    #[test]
    fn write_1() {
        let fn_res = f_w1.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(1));

        let sum = fn_res.call(&resources);

        assert_eq!(2, resources.borrow::<S0>().0);
        assert_eq!(2, sum);
    }

    #[test]
    fn write_2() {
        let fn_res = f_w2.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));

        let sum = fn_res.call(&resources);

        assert_eq!(1, resources.borrow::<S0>().0);
        assert_eq!(2, resources.borrow::<S1>().0);
        assert_eq!(3, sum);
    }

    #[test]
    fn write_3() {
        let fn_res = f_w3.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));
        resources.insert(S2(2));

        let sum = fn_res.call(&resources);

        assert_eq!(1, resources.borrow::<S0>().0);
        assert_eq!(2, resources.borrow::<S1>().0);
        assert_eq!(3, resources.borrow::<S2>().0);
        assert_eq!(6, sum);
    }

    #[test]
    fn write_4() {
        let fn_res = f_w4.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));
        resources.insert(S2(2));
        resources.insert(S3(3));

        let sum = fn_res.call(&resources);

        assert_eq!(1, resources.borrow::<S0>().0);
        assert_eq!(2, resources.borrow::<S1>().0);
        assert_eq!(3, resources.borrow::<S2>().0);
        assert_eq!(4, resources.borrow::<S3>().0);
        assert_eq!(10, sum);
    }

    #[test]
    fn write_5() {
        let fn_res = f_w5.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));
        resources.insert(S2(2));
        resources.insert(S3(3));
        resources.insert(S4(4));

        let sum = fn_res.call(&resources);

        assert_eq!(1, resources.borrow::<S0>().0);
        assert_eq!(2, resources.borrow::<S1>().0);
        assert_eq!(3, resources.borrow::<S2>().0);
        assert_eq!(4, resources.borrow::<S3>().0);
        assert_eq!(5, resources.borrow::<S4>().0);
        assert_eq!(15, sum);
    }

    #[test]
    fn write_6() {
        let fn_res = f_w6.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));
        resources.insert(S2(2));
        resources.insert(S3(3));
        resources.insert(S4(4));
        resources.insert(S5(5));

        let sum = fn_res.call(&resources);

        assert_eq!(1, resources.borrow::<S0>().0);
        assert_eq!(2, resources.borrow::<S1>().0);
        assert_eq!(3, resources.borrow::<S2>().0);
        assert_eq!(4, resources.borrow::<S3>().0);
        assert_eq!(5, resources.borrow::<S4>().0);
        assert_eq!(6, resources.borrow::<S5>().0);
        assert_eq!(21, sum);
    }

    #[test]
    fn read_1_write_1() {
        let fn_res = f_r1_w1.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));

        let sum = fn_res.call(&resources);

        assert_eq!(0, resources.borrow::<S0>().0);
        assert_eq!(2, resources.borrow::<S1>().0);
        assert_eq!(2, sum);
    }

    #[test]
    fn read_1_write_1_closure() {
        let fn_res = (|s0: &S0, s1: &mut S1| {
            s1.0 += 1;
            s0.0 + s1.0
        })
        .into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));

        let sum = fn_res.call(&resources);

        assert_eq!(2, resources.borrow::<S1>().0);
        assert_eq!(2, sum);
    }

    #[test]
    fn write_1_read_1() {
        let fn_res = f_w1_r1.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));

        let sum = fn_res.call(&resources);

        assert_eq!(1, resources.borrow::<S0>().0);
        assert_eq!(1, resources.borrow::<S1>().0);
        assert_eq!(2, sum);
    }

    #[test]
    fn write_1_read_1_closure() {
        let fn_res = (|s0: &mut S0, s1: &S1| {
            s0.0 += 1;

            s0.0 + s1.0
        })
        .into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));

        let sum = fn_res.call(&resources);

        assert_eq!(1, resources.borrow::<S0>().0);
        assert_eq!(1, resources.borrow::<S1>().0);
        assert_eq!(2, sum);
    }

    #[test]
    fn read_1_write_1_read_1() {
        let fn_res = f_r1_w1_r1.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));
        resources.insert(S2(2));

        let sum = fn_res.call(&resources);

        assert_eq!(0, resources.borrow::<S0>().0);
        assert_eq!(2, resources.borrow::<S1>().0);
        assert_eq!(2, resources.borrow::<S2>().0);
        assert_eq!(4, sum);
    }

    #[test]
    fn read_1_write_1_read_1_closure() {
        let fn_res = (|s0: &S0, s1: &mut S1, s2: &S2| {
            s1.0 += 1;

            s0.0 + s1.0 + s2.0
        })
        .into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));
        resources.insert(S2(2));

        let sum = fn_res.call(&resources);

        assert_eq!(0, resources.borrow::<S0>().0);
        assert_eq!(2, resources.borrow::<S1>().0);
        assert_eq!(2, resources.borrow::<S2>().0);
        assert_eq!(4, sum);
    }

    #[test]
    fn write_1_read_1_write_1() {
        let fn_res = f_w1_r1_w1.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));
        resources.insert(S2(2));

        let sum = fn_res.call(&resources);

        assert_eq!(1, resources.borrow::<S0>().0);
        assert_eq!(1, resources.borrow::<S1>().0);
        assert_eq!(3, resources.borrow::<S2>().0);
        assert_eq!(5, sum);
    }

    #[cfg(feature = "high_arg_count")]
    #[test]
    fn write_2_read_2_write_2_read_1() {
        let fn_res = f_w2_r2_w2_r1.into_fn_resource();
        let mut resources = Resources::new();
        resources.insert(S0(0));
        resources.insert(S1(1));
        resources.insert(S2(2));
        resources.insert(S3(3));
        resources.insert(S4(4));
        resources.insert(S5(5));
        resources.insert(S6(6));

        let sum = fn_res.call(&resources);

        assert_eq!(1, resources.borrow::<S0>().0);
        assert_eq!(2, resources.borrow::<S1>().0);
        assert_eq!(2, resources.borrow::<S2>().0);
        assert_eq!(3, resources.borrow::<S3>().0);
        assert_eq!(5, resources.borrow::<S4>().0);
        assert_eq!(6, resources.borrow::<S5>().0);
        assert_eq!(6, resources.borrow::<S6>().0);
        assert_eq!(25, sum);
    }

    #[cfg(all(feature = "fn_meta", feature = "high_arg_count"))]
    #[test]
    fn fn_meta_integration() {
        use std::any::TypeId;

        use fn_meta::FnMetaDyn;

        let fn_res = f_w2_r2_w2_r1.into_fn_resource();
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
