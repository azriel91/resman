# 🗂️ resman

[![Crates.io](https://img.shields.io/crates/v/resman.svg)](https://crates.io/crates/resman)
![CI](https://github.com/azriel91/resman/workflows/CI/badge.svg)
[![Coverage Status](https://codecov.io/gh/azriel91/resman/branch/main/graph/badge.svg)](https://codecov.io/gh/azriel91/resman)

Runtime managed resource borrowing.

This library provides a map that can store one of any type, as well as mutable borrows to each type at the same time.

**Note:** This implementation is extracted from [`shred`], with the following differences:

* Uses [`downcast-rs`] instead of [`mopa`] for downcasting types.
* Adds `Debug` and `PartialEq` implementations for borrow types when the resource type implements those traits.
* Returns `None` instead of panicking for `try_borrow*` functions when the resource is already borrowed.

## Usage

Add the following to `Cargo.toml`

```toml
resman = "0.5.0"

# or
resman = { version = "0.5.0", features = ["debug"] }
```

In code:

```rust
use resman::Resources;

struct A(u32);
struct B(u32);

fn main() {
    let mut resources = Resources::default();

    resources.insert(A(1));
    resources.insert(B(2));

    // We can validly have two mutable borrows from the `Resources` map!
    let mut a = resources.borrow_mut::<A>();
    let mut b = resources.borrow_mut::<B>();
    a.0 = 2;
    b.0 = 3;

    // We need to explicitly drop the A and B borrows, because they are runtime
    // managed borrows, and rustc doesn't know to drop them before the immutable
    // borrows after this.
    drop(a);
    drop(b);

    // Multiple immutable borrows to the same resource are valid.
    let a_0 = resources.borrow::<A>();
    let _a_1 = resources.borrow::<A>();
    let b = resources.borrow::<B>();

    println!("A: {}", a_0.0);
    println!("B: {}", b.0);

    // Trying to mutably borrow a resource that is already borrowed (immutably
    // or mutably) returns `None`.
    let a_try_borrow_mut = resources.try_borrow_mut::<A>();
    let exists = if a_try_borrow_mut.is_some() {
        "Some(..)"
    } else {
        "None"
    };
    println!("a_try_borrow_mut: {}", exists); // prints "None"
}
```

### Features

* `"debug"`:

    The `Debug` implementation for `Resources` will use the `Debug` implementation for the values when printed. This requires that all `Resources` to also implement `Debug`.


    Given the following:

    ```rust
    let mut resources = Resources::default();
    resources.insert(1u32);
    println!("{:?}", resources);
    ```

    Without `"debug"` feature:

    ```rust
    {TypeId { t: 12849923012446332737 }: ".."}
    ```

    With `"debug"` feature:

    ```rust
    {TypeId { t: 12849923012446332737 }: 1}
    ```

## See Also

* [`anymap`]: Map of any type, without multiple mutable borrows.
* [`shred`]: Contains `Resources` type, plus a task dispatcher.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE] or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT] or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


[`anymap`]: https://github.com/chris-morgan/anymap
[`downcast-rs`]: https://github.com/marcianx/downcast-rs
[`mopa`]: https://github.com/chris-morgan/mopa
[`shred`]: https://github.com/amethyst/shred
[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT
