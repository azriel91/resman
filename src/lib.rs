//! Runtime managed resource borrowing.
//!
//! This library provides a map that can store one of any type, as well as
//! mutable borrows to each type at the same time.
//!
//! **Note:** This implementation is extracted from [`shred`], with the
//! following differences:
//!
//! * `Debug` implementation prints out the type name instead of type ID for the
//!   key.
//! * Uses [`downcast-rs`] instead of [`mopa`] for downcasting types.
//! * Adds `Debug` and `PartialEq` implementations for borrow types when the
//!   resource type implements those traits.
//! * Returns `None` instead of panicking for `try_borrow*` functions when the
//!   resource is already borrowed.
//!
//! ## Usage
//!
//! Add the following to `Cargo.toml`
//!
//! ```toml
//! resman = "0.6.0"
//!
//! # or
//! resman = { version = "0.6.0", features = ["debug"] }
//! resman = { version = "0.6.0", features = ["fn_res"] }
//! ```
//!
//! In code:
//!
//! ```rust
//! use resman::Resources;
//!
//! #[derive(Debug)]
//! struct A(u32);
//! #[derive(Debug)]
//! struct B(u32);
//!
//! let mut resources = Resources::default();
//!
//! resources.insert(A(1));
//! resources.insert(B(2));
//!
//! // We can validly have two mutable borrows from the `Resources` map!
//! let mut a = resources.borrow_mut::<A>();
//! let mut b = resources.borrow_mut::<B>();
//! a.0 = 2;
//! b.0 = 3;
//!
//! // We need to explicitly drop the A and B borrows, because they are runtime
//! // managed borrows, and rustc doesn't know to drop them before the immutable
//! // borrows after this.
//! drop(a);
//! drop(b);
//!
//! // Multiple immutable borrows to the same resource are valid.
//! let a_0 = resources.borrow::<A>();
//! let _a_1 = resources.borrow::<A>();
//! let b = resources.borrow::<B>();
//!
//! println!("A: {}", a_0.0);
//! println!("B: {}", b.0);
//!
//! // Trying to mutably borrow a resource that is already borrowed (immutably
//! // or mutably) returns `Err`.
//! let a_try_borrow_mut = resources.try_borrow_mut::<A>();
//! let exists = if a_try_borrow_mut.is_ok() {
//!     "Ok(..)"
//! } else {
//!     "Err"
//! };
//! println!("a_try_borrow_mut: {}", exists); // prints "Err"
//! ```
//!
//! ### Features
//!
//! #### `"debug"`:
//!
//! The `Debug` implementation for `Resources` will use the `Debug`
//! implementation for the values when printed. This requires that all
//! `Resources` to also implement `Debug`.
//!
//! Example:
//!
//! ```rust
//! # use resman::Resources;
//! #
//! let mut resources = Resources::default();
//! resources.insert(1u32);
//! println!("{:?}", resources);
//!
//! // Without `"debug"` feature:
//! // {u32: ".."}
//!
//! // With `"debug"` feature:
//! // {u32: 1}
//! ```
//!
//! #### `"fn_res"`:
//!
//! Enables the [`FnRes`] trait, allowing dynamic functions invocation under a
//! generic function type.
//!
//! Usage of this API is as follows:
//!
//! 1. Define regular functions or closures to run.
//!
//!     - The functions should take `&T` or `&mut T` as parameters.
//!     - The return type of all functions should be the same.
//!
//!     Currently there is a limit of 7 parameters.
//!
//! 2. Call `my_function.into_fn_res()` to obtain a `Box<dyn FnRes>`.
//! 3. Call `fn_res.call(&resources)` to automatically borrow `T` from
//!    `resources` and invoke the function.
//!
//! Example:
//!
//! ```rust
//! use resman::{FnRes, IntoFnRes, Resources};
//!
//! /// Borrows `u32` mutably, and `u64` immutably.
//! fn f1(a: &mut u32, b: &u64) -> u64 {
//!     *a += 1;
//!     *a as u64 + *b
//! }
//!
//! /// Borrows `u32` immutably, and `u64` mutably.
//! fn f2(a: &u32, b: &mut u64) -> u64 {
//!     *b += 1;
//!     *a as u64 + *b
//! }
//!
//! let functions = [
//!     f1.into_fn_res(),
//!     f2.into_fn_res(),
//!     (|a: &u32, b: &u64| *a as u64 + *b).into_fn_res(),
//! ];
//!
//! let mut resources = Resources::default();
//! resources.insert(0u32);
//! resources.insert(0u64);
//!
//! let sum = functions
//!     .iter()
//!     .fold(0, |sum, fn_res| sum + fn_res.call(&resources));
//!
//! assert_eq!(5, sum); // 1 + 2 + 2
//!
//! let debug_str = format!("{:?}", resources);
//! assert!(debug_str.contains("u32: 1"));
//! assert!(debug_str.contains("u64: 1"));
//! ```
//!
//! Since `Resources` has internal mutability, care must be taken to not run
//! multiple functions that borrow the same value mutably from `Resources` at
//! the same time when using [`FnRes::call`], otherwise it will panic.
//!
//! Use [`FnRes::try_call`] for a non-panicking version, which will return a
//! [`BorrowFail`] error if there is an overlapping borrow conflict at runtime.
//!
//! ## See Also
//!
//! * [`anymap`]: Map of any type, without multiple mutable borrows.
//! * [`shred`]: Contains `Resources` type, plus a task dispatcher.
//!
//! [`anymap`]: https://github.com/chris-morgan/anymap
//! [`downcast-rs`]: https://github.com/marcianx/downcast-rs
//! [`mopa`]: https://github.com/chris-morgan/mopa
//! [`shred`]: https://github.com/amethyst/shred

pub use crate::{
    entry::Entry, r#ref::Ref, ref_mut::RefMut, resource::Resource, resources::Resources,
};

pub use rt_map::BorrowFail;

mod entry;
mod r#ref;
mod ref_mut;
mod resource;
mod resources;

#[cfg(feature = "fn_res")]
pub use crate::{
    fn_res::{FnRes, IntoFnRes},
    fn_resource::{FnResource, IntoFnResource},
};

#[cfg(feature = "fn_res")]
mod fn_res;
#[cfg(feature = "fn_res")]
mod fn_resource;
