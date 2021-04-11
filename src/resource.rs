use std::any::Any;

use downcast_rs::DowncastSync;

/// Trait to represent any type that is `Send + Sync + 'static`.
///
/// A resource is a data slot which lives in the `World` can only be accessed
/// according to Rust's typical borrowing model (one writer xor multiple
/// readers).
pub trait Resource: DowncastSync + 'static {}
downcast_rs::impl_downcast!(sync Resource);

impl<T> Resource for T where T: Any + Send + Sync {}
