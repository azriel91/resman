use std::{fmt, marker::PhantomData, ops::Deref};

use crate::Resource;

/// Reference to a resource.
#[derive(Clone)]
pub struct Ref<'a, R: 'a> {
    inner: rt_map::Ref<'a, Box<dyn Resource>>,
    phantom: PhantomData<&'a R>,
}

impl<'a, R> Ref<'a, R> {
    pub fn new(inner: rt_map::Ref<'a, Box<dyn Resource>>) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<R> Deref for Ref<'_, R>
where
    R: Resource,
{
    type Target = R;

    fn deref(&self) -> &R {
        (*self.inner)
            .downcast_ref::<R>()
            .unwrap_or_else(|| panic!("Failed to downcast to {}", std::any::type_name::<R>()))
    }
}

impl<'a, R> fmt::Debug for Ref<'a, R>
where
    R: Resource + fmt::Debug + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner: &R = self;
        f.debug_struct("Ref").field("inner", inner).finish()
    }
}

impl<'a, R> PartialEq for Ref<'a, R>
where
    R: Resource + PartialEq + 'a,
{
    fn eq(&self, other: &Self) -> bool {
        let r_self: &R = self;
        let r_other: &R = other;
        r_self == r_other
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{self, Write};

    use rt_map::Cell;

    use crate::Resource;

    use super::Ref;

    #[test]
    fn debug_includes_inner_field() -> fmt::Result {
        let value: Box<dyn Resource> = Box::new(A(1));
        let cell = Cell::new(value);
        let r#ref = Ref::<A>::new(rt_map::Ref::new(cell.borrow()));

        let mut debug_string = String::with_capacity(64);
        write!(&mut debug_string, "{:?}", r#ref)?;
        assert_eq!("Ref { inner: A(1) }", debug_string.as_str());

        Ok(())
    }

    #[test]
    fn partial_eq_compares_value() -> fmt::Result {
        let value_0: Box<dyn Resource> = Box::new(A(1));
        let cell_0 = Cell::new(value_0);
        let ref_0 = Ref::<A>::new(rt_map::Ref::new(cell_0.borrow()));

        let value_1: Box<dyn Resource> = Box::new(A(1));
        let cell_1 = Cell::new(value_1);
        let ref_1 = Ref::<A>::new(rt_map::Ref::new(cell_1.borrow()));

        assert_eq!(ref_1, ref_0);
        assert_eq!(Ref::<A>::new(rt_map::Ref::new(cell_0.borrow())), ref_0);

        Ok(())
    }

    #[derive(Debug, Clone, PartialEq)]
    struct A(usize);
}
