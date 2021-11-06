use crate::{FnRes, FnResource, IntoFnResource};

/// Extension to return `Box<dyn FnRes>` for a function.
pub trait IntoFnRes<Fun, Ret, Args> {
    /// Returns the function wrapped as a `Box<dyn FnRes>`.
    fn into_fn_res(self) -> Box<dyn FnRes<Ret = Ret>>;
}

impl<Fun, Ret> IntoFnRes<Fun, Ret, ()> for Fun
where
    Fun: Fn() -> Ret + 'static,
    Ret: 'static,
    FnResource<Fun, Ret, ()>: FnRes<Ret = Ret>,
{
    fn into_fn_res(self) -> Box<dyn FnRes<Ret = Ret>> {
        Box::new(self.into_fn_resource())
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

impl<Fun, Ret, A, B, C, D, E, F, G, H> IntoFnRes<Fun, Ret, (A, B, C, D, E, F, G, H)> for Fun
where
    Fun: Fn(A, B, C, D, E, F, G, H) -> Ret + 'static,
    Ret: 'static,
    A: 'static,
    B: 'static,
    C: 'static,
    D: 'static,
    E: 'static,
    F: 'static,
    G: 'static,
    H: 'static,
    FnResource<Fun, Ret, (A, B, C, D, E, F, G, H)>: FnRes<Ret = Ret>,
{
    fn into_fn_res(self) -> Box<dyn FnRes<Ret = Ret>> {
        Box::new(self.into_fn_resource())
    }
}
