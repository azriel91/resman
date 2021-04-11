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
