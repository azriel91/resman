use crate::{FnResMut, FnResource, IntoFnResource};

/// Extension to return `Box<dyn FnResMut>` for a function.
pub trait IntoFnResMut<Fun, Ret, Args> {
    /// Returns the function wrapped as a `Box<dyn FnResMut>`.
    fn into_fn_res_mut(self) -> Box<dyn FnResMut<Ret = Ret>>;
}

impl<Fun, Ret> IntoFnResMut<Fun, Ret, ()> for Fun
where
    Fun: FnMut() -> Ret + 'static,
    Ret: 'static,
    FnResource<Fun, Ret, ()>: FnResMut<Ret = Ret>,
{
    fn into_fn_res_mut(self) -> Box<dyn FnResMut<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A> IntoFnResMut<Fun, Ret, (A,)> for Fun
where
    Fun: FnMut(A) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    FnResource<Fun, Ret, (A,)>: FnResMut<Ret = Ret>,
{
    fn into_fn_res_mut(self) -> Box<dyn FnResMut<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B> IntoFnResMut<Fun, Ret, (A, B)> for Fun
where
    Fun: FnMut(A, B) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    FnResource<Fun, Ret, (A, B)>: FnResMut<Ret = Ret>,
{
    fn into_fn_res_mut(self) -> Box<dyn FnResMut<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B, C> IntoFnResMut<Fun, Ret, (A, B, C)> for Fun
where
    Fun: FnMut(A, B, C) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    FnResource<Fun, Ret, (A, B, C)>: FnResMut<Ret = Ret>,
{
    fn into_fn_res_mut(self) -> Box<dyn FnResMut<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B, C, D> IntoFnResMut<Fun, Ret, (A, B, C, D)> for Fun
where
    Fun: FnMut(A, B, C, D) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    FnResource<Fun, Ret, (A, B, C, D)>: FnResMut<Ret = Ret>,
{
    fn into_fn_res_mut(self) -> Box<dyn FnResMut<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B, C, D, E> IntoFnResMut<Fun, Ret, (A, B, C, D, E)> for Fun
where
    Fun: FnMut(A, B, C, D, E) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    FnResource<Fun, Ret, (A, B, C, D, E)>: FnResMut<Ret = Ret>,
{
    fn into_fn_res_mut(self) -> Box<dyn FnResMut<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B, C, D, E, F> IntoFnResMut<Fun, Ret, (A, B, C, D, E, F)> for Fun
where
    Fun: FnMut(A, B, C, D, E, F) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    FnResource<Fun, Ret, (A, B, C, D, E, F)>: FnResMut<Ret = Ret>,
{
    fn into_fn_res_mut(self) -> Box<dyn FnResMut<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

#[cfg(feature = "high_arg_count")]
impl<Fun, Ret, A, B, C, D, E, F, G> IntoFnResMut<Fun, Ret, (A, B, C, D, E, F, G)> for Fun
where
    Fun: FnMut(A, B, C, D, E, F, G) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
    FnResource<Fun, Ret, (A, B, C, D, E, F, G)>: FnResMut<Ret = Ret>,
{
    fn into_fn_res_mut(self) -> Box<dyn FnResMut<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

#[cfg(feature = "high_arg_count")]
impl<Fun, Ret, A, B, C, D, E, F, G, H> IntoFnResMut<Fun, Ret, (A, B, C, D, E, F, G, H)> for Fun
where
    Fun: FnMut(A, B, C, D, E, F, G, H) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
    H: 'static,
    FnResource<Fun, Ret, (A, B, C, D, E, F, G, H)>: FnResMut<Ret = Ret>,
{
    fn into_fn_res_mut(self) -> Box<dyn FnResMut<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

#[cfg(test)]
mod tests {
    use super::IntoFnResMut;

    #[test]
    fn into_fn_res_mut() {
        let mut d0 = 0usize;
        let mut d1 = 1usize;
        let mut d2 = 2usize;
        let mut d3 = 3usize;
        let mut d4 = 4usize;
        let mut d5 = 5usize;
        let mut d6 = 6usize;

        let _ = (move || {
            d0 += 1;
            d0
        })
        .into_fn_res_mut();
        let _ = (move |_: &S0| {
            d1 += 1;
            d1
        })
        .into_fn_res_mut();
        let _ = (move |_: &S0, _: &S1| {
            d2 += 1;
            d2
        })
        .into_fn_res_mut();
        let _ = (move |_: &S0, _: &S1, _: &S2| {
            d3 += 1;
            d3
        })
        .into_fn_res_mut();
        let _ = (move |_: &S0, _: &S1, _: &S2, _: &S3| {
            d4 += 1;
            d4
        })
        .into_fn_res_mut();
        let _ = (move |_: &S0, _: &S1, _: &S2, _: &S3, _: &S4| {
            d5 += 1;
            d5
        })
        .into_fn_res_mut();
        let _ = (move |_: &S0, _: &S1, _: &S2, _: &S3, _: &S4, _: &S5| {
            d6 += 1;
            d6
        })
        .into_fn_res_mut();
    }

    #[cfg(feature = "high_arg_count")]
    #[test]
    fn into_fn_res_mut_high_arg_count() {
        let mut d7 = 7usize;
        let mut d8 = 8usize;

        let _ = (move |_: &S0, _: &S1, _: &S2, _: &S3, _: &S4, _: &S5, _: &S6| {
            d7 += 1;
            d7
        })
        .into_fn_res_mut();
        let _ = (move |_: &S0, _: &S1, _: &S2, _: &S3, _: &S4, _: &S5, _: &S6, _: &S7| {
            d8 += 1;
            d8
        })
        .into_fn_res_mut();
    }

    #[derive(Debug)]
    struct S0;
    #[derive(Debug)]
    struct S1;
    #[derive(Debug)]
    struct S2;
    #[derive(Debug)]
    struct S3;
    #[derive(Debug)]
    struct S4;
    #[derive(Debug)]
    struct S5;
    #[cfg(feature = "high_arg_count")]
    #[derive(Debug)]
    struct S6;
    #[cfg(feature = "high_arg_count")]
    #[derive(Debug)]
    struct S7;
}
