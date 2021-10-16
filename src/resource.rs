use std::any::Any;

use downcast_rs::DowncastSync;

/// Trait to represent any type that is `Send + Sync + 'static`.
///
/// A resource is a data slot which lives in the `World` can only be accessed
/// according to Rust's typical borrowing model (one writer xor multiple
/// readers).
#[cfg(not(feature = "debug"))]
pub trait Resource: DowncastSync + 'static {
    fn type_name(&self) -> &'static str;
}

#[cfg(not(feature = "debug"))]
impl<T> Resource for T
where
    T: Any + Send + Sync,
{
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

/// Trait to represent any type that is `Send + Sync + 'static`.
///
/// A resource is a data slot which lives in the `World` can only be accessed
/// according to Rust's typical borrowing model (one writer xor multiple
/// readers).
#[cfg(feature = "debug")]
pub trait Resource: DowncastSync + std::fmt::Debug + 'static {
    fn type_name(&self) -> &'static str;
}

#[cfg(feature = "debug")]
impl<T> Resource for T
where
    T: Any + std::fmt::Debug + Send + Sync,
{
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

downcast_rs::impl_downcast!(sync Resource);
