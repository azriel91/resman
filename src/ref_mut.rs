use std::{
    cmp::PartialEq,
    fmt,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

pub use crate::Resource;

/// Mutable reference to a resource.
pub struct RefMut<'a, R: 'a> {
    inner: rt_map::RefMut<'a, Box<dyn Resource>>,
    phantom: PhantomData<&'a R>,
}

impl<'a, R> fmt::Debug for RefMut<'a, R>
where
    R: Resource + fmt::Debug + 'a,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner: &R = &*self;
        f.debug_struct("RefMut").field("inner", inner).finish()
    }
}

impl<'a, R> PartialEq for RefMut<'a, R>
where
    R: Resource + PartialEq + 'a,
{
    fn eq(&self, other: &Self) -> bool {
        let r_self: &R = &*self;
        let r_other: &R = &*other;
        r_self == r_other
    }
}

impl<'a, R> RefMut<'a, R> {
    pub fn new(inner: rt_map::RefMut<'a, Box<dyn Resource>>) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<'a, R> Deref for RefMut<'a, R>
where
    R: Resource,
{
    type Target = R;

    fn deref(&self) -> &R {
        self.inner
            .downcast_ref::<R>()
            .unwrap_or_else(|| panic!("Failed to downcast to {}", std::any::type_name::<R>()))
    }
}

impl<'a, R> DerefMut for RefMut<'a, R>
where
    R: Resource,
{
    fn deref_mut(&mut self) -> &mut R {
        self.inner
            .downcast_mut::<R>()
            .unwrap_or_else(|| panic!("Failed to downcast to {}", std::any::type_name::<R>()))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fmt::{self, Write},
        sync::atomic::AtomicUsize,
    };

    use rt_map::CellRefMut;

    use crate::Resource;

    use super::RefMut;

    #[test]
    fn debug_includes_inner_field() -> fmt::Result {
        let flag = AtomicUsize::new(0);
        let mut value: Box<dyn Resource> = Box::new(A(1));
        let ref_mut = RefMut::<A>::new(rt_map::RefMut::new(CellRefMut {
            flag: &flag,
            value: &mut value,
        }));

        let mut debug_string = String::with_capacity(64);
        write!(&mut debug_string, "{:?}", ref_mut)?;
        assert_eq!("RefMut { inner: A(1) }", debug_string.as_str());

        Ok(())
    }

    #[test]
    fn partial_eq_compares_value() -> fmt::Result {
        let flag = AtomicUsize::new(0);
        let mut value: Box<dyn Resource> = Box::new(A(1));
        let mut value_clone: Box<dyn Resource> = Box::new(A(1));
        let ref_mut = RefMut::<A>::new(rt_map::RefMut::new(CellRefMut {
            flag: &flag,
            value: &mut value,
        }));

        assert_eq!(
            RefMut::<A>::new(rt_map::RefMut::new(CellRefMut {
                flag: &flag,
                value: &mut value_clone,
            })),
            ref_mut
        );
        assert_ne!(
            RefMut::<A>::new(rt_map::RefMut::new(CellRefMut {
                flag: &flag,
                value: &mut (Box::new(A(2)) as Box<dyn Resource>),
            })),
            ref_mut
        );

        Ok(())
    }

    #[test]
    fn deref_mut_returns_value() -> fmt::Result {
        let flag = AtomicUsize::new(0);
        let mut value: Box<dyn Resource> = Box::new(A(1));
        let mut ref_mut = RefMut::<A>::new(rt_map::RefMut::new(CellRefMut {
            flag: &flag,
            value: &mut value,
        }));

        assert_eq!(
            RefMut::<A>::new(rt_map::RefMut::new(CellRefMut {
                flag: &flag,
                value: &mut (Box::new(A(1)) as Box<dyn Resource>),
            })),
            ref_mut
        );

        ref_mut.0 = 2;

        assert_eq!(
            RefMut::<A>::new(rt_map::RefMut::new(CellRefMut {
                flag: &flag,
                value: &mut (Box::new(A(2)) as Box<dyn Resource>),
            })),
            ref_mut
        );

        Ok(())
    }

    #[derive(Debug, Clone, PartialEq)]
    struct A(usize);
}
