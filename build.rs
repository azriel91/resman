#[cfg(not(feature = "fn_res"))]
fn main() {}

#[cfg(feature = "fn_res")]
fn main() {
    fn_resource_impl::run();
}

#[cfg(feature = "fn_res")]
mod fn_resource_impl {
    use std::{env, fmt::Write as _, fs::OpenOptions, io::Write, mem::MaybeUninit, path::Path};

    pub fn run() {
        let out_dir =
            env::var_os("OUT_DIR").expect("Failed to read `OUT_DIR` environment variable.");
        let fn_resource_impl_path = Path::new(&out_dir).join("fn_resource_impl.rs");
        let mut fn_resource_impl = OpenOptions::new()
            .create(true)
            .write(true)
            .open(fn_resource_impl_path)
            .expect("Failed to open `fn_resource_impl.rs`.");

        write!(fn_resource_impl, "{}", generate_impls_for_n_args::<1>())
            .expect("Failed to write to fn_resource_impl.rs");
        write!(fn_resource_impl, "{}", generate_impls_for_n_args::<2>())
            .expect("Failed to write to fn_resource_impl.rs");
        write!(fn_resource_impl, "{}", generate_impls_for_n_args::<3>())
            .expect("Failed to write to fn_resource_impl.rs");
        write!(fn_resource_impl, "{}", generate_impls_for_n_args::<4>())
            .expect("Failed to write to fn_resource_impl.rs");
        write!(fn_resource_impl, "{}", generate_impls_for_n_args::<5>())
            .expect("Failed to write to fn_resource_impl.rs");
        write!(fn_resource_impl, "{}", generate_impls_for_n_args::<6>())
            .expect("Failed to write to fn_resource_impl.rs");
        write!(fn_resource_impl, "{}", generate_impls_for_n_args::<7>())
            .expect("Failed to write to fn_resource_impl.rs");

        fn_resource_impl
            .flush()
            .expect("Failed to flush writer for fn_resource_impl.rs");

        println!("cargo:rerun-if-changed=build.rs");
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Ref {
        Immutable,
        Mutable,
    }

    fn generate_impls_for_n_args<const N: usize>() -> String {
        // "A0, A1"
        let args_csv = args_csv::<N>();

        // "    A0: 'static,\n    A1: 'static,"
        let arg_bounds_list = arg_bounds_list::<N>();

        arg_refs_combinations::<N>().fold(
            String::with_capacity(N * (256 + N * 64)),
            |mut impls_buffer, arg_refs| {
                let mut arg_refs_iter = arg_refs.iter().copied().enumerate();

                // &mut A0, &A1
                let arg_refs_csv = {
                    let mut arg_refs_csv = String::with_capacity(N * 8);
                    if let Some((_index, arg_ref_first)) = arg_refs_iter.next() {
                        match arg_ref_first {
                            Ref::Immutable => arg_refs_csv.push_str("&A0"),
                            Ref::Mutable => arg_refs_csv.push_str("&mut A0"),
                        }
                    }

                    if N == 1 {
                        arg_refs_csv.push(',');
                    } else {
                        arg_refs_iter
                            .try_for_each(|(index, arg_ref)| match arg_ref {
                                Ref::Immutable => write!(&mut arg_refs_csv, ", &A{}", index),
                                Ref::Mutable => write!(&mut arg_refs_csv, ", &mut A{}", index),
                            })
                            .expect("Failed to append to `arg_refs_csv` string.");
                    }

                    arg_refs_csv
                };

                // let a0 = resources.borrow::<A0>();
                // let mut a1 = resources.borrow_mut::<A1>();
                // ..
                let resource_arg_borrows = resource_arg_borrows(arg_refs);

                // &*a0, &mut *a1
                let resource_arg_vars = resource_arg_vars::<N>(arg_refs);

                write!(
                    impls_buffer,
                    r#"
impl<Fun, Ret, {args_csv}> FnResource<Fun, Ret, ({arg_refs_csv})>
where
    Fun: Fn({arg_refs_csv}) -> Ret,
    {arg_bounds_list}
{{
    pub fn call<'f>(&self, resources: &Resources) -> Ret {{
        {resource_arg_borrows}

        (self.func)({resource_arg_vars})
    }}
}}

impl<Fun, Ret, {args_csv}> FnRes for FnResource<Fun, Ret, ({arg_refs_csv})>
where
    Fun: Fn({arg_refs_csv}) -> Ret,
    {arg_bounds_list}
{{
    type Ret = Ret;

    fn call<'f>(&self, resources: &Resources) -> Ret {{
        Self::call(self, resources)
    }}
}}
        "#,
                    args_csv = args_csv,
                    arg_refs_csv = arg_refs_csv,
                    arg_bounds_list = arg_bounds_list,
                    resource_arg_borrows = resource_arg_borrows,
                    resource_arg_vars = resource_arg_vars,
                )
                .expect("Failed to append to impls_buffer.");

                impls_buffer
            },
        )
    }

    fn resource_arg_vars<const N: usize>(arg_refs: [Ref; N]) -> String {
        let mut resource_arg_vars = String::with_capacity(N * 10);
        let mut arg_refs_iter = arg_refs.iter().copied().enumerate();
        if let Some((index, arg_ref)) = arg_refs_iter.next() {
            match arg_ref {
                Ref::Immutable => write!(&mut resource_arg_vars, "&*a{}", index),
                Ref::Mutable => write!(&mut resource_arg_vars, "&mut *a{}", index),
            }
            .expect("Failed to append to `resource_arg_vars` string.")
        }
        arg_refs_iter
            .try_for_each(|(index, arg_ref)| match arg_ref {
                Ref::Immutable => write!(&mut resource_arg_vars, ", &*a{}", index),
                Ref::Mutable => write!(&mut resource_arg_vars, ", &mut *a{}", index),
            })
            .expect("Failed to append to `resource_arg_vars` string.");
        resource_arg_vars
    }

    fn resource_arg_borrows<const N: usize>(arg_refs: [Ref; N]) -> String {
        let mut resource_arg_borrows = String::with_capacity(N * 44);
        let mut arg_refs_iter = arg_refs.iter().copied().enumerate();
        arg_refs_iter
            .try_for_each(|(index, arg_ref)| match arg_ref {
                Ref::Immutable => writeln!(
                    &mut resource_arg_borrows,
                    "let a{index} = resources.borrow::<A{index}>();",
                    index = index
                ),
                Ref::Mutable => writeln!(
                    &mut resource_arg_borrows,
                    "let mut a{index} = resources.borrow_mut::<A{index}>();",
                    index = index
                ),
            })
            .expect("Failed to append to `resource_arg_borrows` string.");
        resource_arg_borrows
    }

    fn arg_refs_combinations<const N: usize>() -> impl Iterator<Item = [Ref; N]> {
        (0..(2 << (N - 1))).map(|m| {
            // `m` is the combination variation count.
            // Whether an argument is immutable or mutable is bed on its corresponding bit
            // value of `m`.

            // Create an uninitialized array of `MaybeUninit`. The `assume_init` is safe
            // because the type we are claiming to have initialized here is a bunch of
            // `MaybeUninit`s, which do not require initialization.
            //
            // https://doc.rust-lang.org/stable/std/mem/union.MaybeUninit.html#initializing-an-array-element-by-element
            //
            // Switch this to `MaybeUninit::uninit_array` once it is stable.
            let mut arg_refs: [MaybeUninit<Ref>; N] =
                unsafe { MaybeUninit::uninit().assume_init() };

            arg_refs
                .iter_mut()
                .enumerate()
                .for_each(move |(arg_n, arg_ref_mem)| {
                    // for N = 5
                    // m can be 0..32
                    // if 31 >> 5 is 0

                    if m >> arg_n & 1 == 0 {
                        arg_ref_mem.write(Ref::Immutable);
                    } else {
                        arg_ref_mem.write(Ref::Mutable);
                    }
                });

            // Everything is initialized. Transmute the array to the initialized type.
            // Unfortunately we cannot use this, see the following issues:
            //
            // * <https://github.com/rust-lang/rust/issues/61956>
            // * <https://github.com/rust-lang/rust/issues/80908>
            //
            // let arg_refs = unsafe { mem::transmute::<_, [Ref;
            // N]>(arg_refs) };

            #[allow(clippy::let_and_return)] // for clarity with `unsafe`
            let arg_refs = {
                let ptr = &mut arg_refs as *mut _ as *mut [Ref; N];
                let array = unsafe { ptr.read() };

                // We don't have to `mem::forget` the original because `Ref` is `Copy`.
                // mem::forget(arg_refs);

                array
            };

            arg_refs
        })
    }

    fn arg_bounds_list<const N: usize>() -> String {
        let mut arg_bounds_list = String::with_capacity(N * 50);
        #[cfg(feature = "debug")]
        arg_bounds_list.push_str("    A0: std::fmt::Debug + Send + Sync + 'static,");

        #[cfg(not(feature = "debug"))]
        arg_bounds_list.push_str("    A0: Send + Sync + 'static,");
        (1..N).fold(arg_bounds_list, |mut arg_bounds_list, n| {
            #[cfg(feature = "debug")]
            write!(
                &mut arg_bounds_list,
                "\n    A{}: std::fmt::Debug + Send + Sync + 'static,",
                n
            )
            .expect("Failed to append to args_csv string.");

            #[cfg(not(feature = "debug"))]
            write!(&mut arg_bounds_list, "\n    A{}: Send + Sync + 'static,", n)
                .expect("Failed to append to args_csv string.");
            arg_bounds_list
        })
    }

    fn args_csv<const N: usize>() -> String {
        let mut args_csv = String::with_capacity(N * 4);
        args_csv.push_str("A0");
        (1..N).fold(args_csv, |mut args_csv, n| {
            write!(&mut args_csv, ", A{}", n).expect("Failed to append to args_csv string.");
            args_csv
        })
    }
}
