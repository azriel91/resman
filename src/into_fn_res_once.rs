use crate::{FnResOnce, FnResource, IntoFnResource};

/// Extension to return `Box<dyn FnResOnce>` for a function.
pub trait IntoFnResOnce<Fun, Ret, Args> {
    /// Returns the function wrapped as a `Box<dyn FnResOnce>`.
    fn into_fn_res_once(self) -> Box<dyn FnResOnce<Ret = Ret>>;
}

impl<Fun, Ret> IntoFnResOnce<Fun, Ret, ()> for Fun
where
    Fun: FnOnce() -> Ret + 'static,
    Ret: 'static,
    FnResource<Fun, Ret, ()>: FnResOnce<Ret = Ret>,
{
    fn into_fn_res_once(self) -> Box<dyn FnResOnce<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A> IntoFnResOnce<Fun, Ret, (A,)> for Fun
where
    Fun: FnOnce(A) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    FnResource<Fun, Ret, (A,)>: FnResOnce<Ret = Ret>,
{
    fn into_fn_res_once(self) -> Box<dyn FnResOnce<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B> IntoFnResOnce<Fun, Ret, (A, B)> for Fun
where
    Fun: FnOnce(A, B) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    FnResource<Fun, Ret, (A, B)>: FnResOnce<Ret = Ret>,
{
    fn into_fn_res_once(self) -> Box<dyn FnResOnce<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B, C> IntoFnResOnce<Fun, Ret, (A, B, C)> for Fun
where
    Fun: FnOnce(A, B, C) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    FnResource<Fun, Ret, (A, B, C)>: FnResOnce<Ret = Ret>,
{
    fn into_fn_res_once(self) -> Box<dyn FnResOnce<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B, C, D> IntoFnResOnce<Fun, Ret, (A, B, C, D)> for Fun
where
    Fun: FnOnce(A, B, C, D) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    FnResource<Fun, Ret, (A, B, C, D)>: FnResOnce<Ret = Ret>,
{
    fn into_fn_res_once(self) -> Box<dyn FnResOnce<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B, C, D, E> IntoFnResOnce<Fun, Ret, (A, B, C, D, E)> for Fun
where
    Fun: FnOnce(A, B, C, D, E) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    FnResource<Fun, Ret, (A, B, C, D, E)>: FnResOnce<Ret = Ret>,
{
    fn into_fn_res_once(self) -> Box<dyn FnResOnce<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

impl<Fun, Ret, A, B, C, D, E, F> IntoFnResOnce<Fun, Ret, (A, B, C, D, E, F)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    FnResource<Fun, Ret, (A, B, C, D, E, F)>: FnResOnce<Ret = Ret>,
{
    fn into_fn_res_once(self) -> Box<dyn FnResOnce<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

#[cfg(feature = "high_arg_count")]
impl<Fun, Ret, A, B, C, D, E, F, G> IntoFnResOnce<Fun, Ret, (A, B, C, D, E, F, G)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F, G) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
    FnResource<Fun, Ret, (A, B, C, D, E, F, G)>: FnResOnce<Ret = Ret>,
{
    fn into_fn_res_once(self) -> Box<dyn FnResOnce<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

#[cfg(feature = "high_arg_count")]
impl<Fun, Ret, A, B, C, D, E, F, G, H> IntoFnResOnce<Fun, Ret, (A, B, C, D, E, F, G, H)> for Fun
where
    Fun: FnOnce(A, B, C, D, E, F, G, H) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
    H: 'static,
    FnResource<Fun, Ret, (A, B, C, D, E, F, G, H)>: FnResOnce<Ret = Ret>,
{
    fn into_fn_res_once(self) -> Box<dyn FnResOnce<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}

#[cfg(test)]
mod tests {
    use super::IntoFnResOnce;

    #[test]
    fn into_fn_res_once() {
        let d0 = D;
        let d1 = D;
        let d2 = D;
        let d3 = D;
        let d4 = D;
        let d5 = D;
        let d6 = D;

        let _ = (move || d0).into_fn_res_once();
        let _ = (move |_: &S0| d1).into_fn_res_once();
        let _ = (move |_: &S0, _: &S1| d2).into_fn_res_once();
        let _ = (move |_: &S0, _: &S1, _: &S2| d3).into_fn_res_once();
        let _ = (move |_: &S0, _: &S1, _: &S2, _: &S3| d4).into_fn_res_once();
        let _ = (move |_: &S0, _: &S1, _: &S2, _: &S3, _: &S4| d5).into_fn_res_once();
        let _ = (move |_: &S0, _: &S1, _: &S2, _: &S3, _: &S4, _: &S5| d6).into_fn_res_once();
    }

    #[cfg(feature = "high_arg_count")]
    #[test]
    fn into_fn_res_once_high_arg_count() {
        let d7 = D;
        let d8 = D;

        let _ =
            (move |_: &S0, _: &S1, _: &S2, _: &S3, _: &S4, _: &S5, _: &S6| d7).into_fn_res_once();
        let _ = (move |_: &S0, _: &S1, _: &S2, _: &S3, _: &S4, _: &S5, _: &S6, _: &S7| d8)
            .into_fn_res_once();
    }

    #[derive(Debug)]
    struct D;
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
