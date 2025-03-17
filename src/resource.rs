use std::any::{Any, TypeId};

use downcast_rs::DowncastSync;

/// Trait to represent any type that is `Send + Sync`.
///
/// A resource is a data slot which lives in the `World` can only be accessed
/// according to Rust's typical borrowing model (one writer xor multiple
/// readers).
#[cfg(not(feature = "debug"))]
pub trait Resource: DowncastSync {
    fn type_id(&self) -> TypeId;
    fn type_name(&self) -> TypeNameLit;
}

#[cfg(not(feature = "debug"))]
impl<T> Resource for T
where
    T: Any + Send + Sync,
{
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn type_name(&self) -> TypeNameLit {
        TypeNameLit(std::any::type_name::<T>())
    }
}

/// Trait to represent any type that is `Send + Sync`.
///
/// A resource is a data slot which lives in the `World` can only be accessed
/// according to Rust's typical borrowing model (one writer xor multiple
/// readers).
#[cfg(feature = "debug")]
pub trait Resource: DowncastSync + std::fmt::Debug {
    fn type_id(&self) -> TypeId;
    fn type_name(&self) -> TypeNameLit;
}

#[cfg(feature = "debug")]
impl<T> Resource for T
where
    T: Any + std::fmt::Debug + Send + Sync,
{
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn type_name(&self) -> TypeNameLit {
        TypeNameLit(std::any::type_name::<T>())
    }
}

downcast_rs::impl_downcast!(sync Resource);

use std::fmt;

pub struct TypeNameLit(&'static str);

impl fmt::Debug for TypeNameLit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
