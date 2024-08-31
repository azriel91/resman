#[cfg(not(feature = "fn_res"))]
fn main() {}

#[cfg(feature = "fn_res")]
fn main() {
    use std::{env, io::Write, path::Path};

    use common::{generate_impls_for_n_args, ArgExprs};

    let out_dir = env::var_os("OUT_DIR").expect("Failed to read `OUT_DIR` environment variable.");
    let out_dir = Path::new(&out_dir);

    #[cfg(feature = "fn_res")]
    let mut fn_resource_impl = common::open_impl_file(out_dir, "fn_resource_impl.rs");
    #[cfg(feature = "fn_res_once")]
    let mut fn_res_once_impl = common::open_impl_file(out_dir, "fn_res_once_impl.rs");
    #[cfg(feature = "fn_res_mut")]
    let mut fn_res_mut_impl = common::open_impl_file(out_dir, "fn_res_mut_impl.rs");
    #[cfg(feature = "fn_res")]
    let mut fn_res_impl = common::open_impl_file(out_dir, "fn_res_impl.rs");

    #[cfg(feature = "fn_meta")]
    let mut fn_resource_meta_impl = common::open_impl_file(out_dir, "fn_resource_meta_impl.rs");

    let mut write_fn = |arg_exprs: ArgExprs<'_>| {
        #[cfg(feature = "fn_res")]
        fn_resource_impl::write_fn_resource_impl(&mut fn_resource_impl, arg_exprs);

        #[cfg(feature = "fn_res_once")]
        fn_res_once_impl::write_fn_res_once_impl(&mut fn_res_once_impl, arg_exprs);
        #[cfg(feature = "fn_res_mut")]
        fn_res_mut_impl::write_fn_res_mut_impl(&mut fn_res_mut_impl, arg_exprs);
        #[cfg(feature = "fn_res")]
        fn_res_impl::write_fn_res_impl(&mut fn_res_impl, arg_exprs);

        #[cfg(feature = "fn_meta")]
        fn_resource_meta_impl::write_fn_resource_meta_impl(&mut fn_resource_meta_impl, arg_exprs);
    };

    generate_impls_for_n_args::<_, 1>(&mut write_fn);
    generate_impls_for_n_args::<_, 2>(&mut write_fn);
    generate_impls_for_n_args::<_, 3>(&mut write_fn);
    generate_impls_for_n_args::<_, 4>(&mut write_fn);
    generate_impls_for_n_args::<_, 5>(&mut write_fn);
    generate_impls_for_n_args::<_, 6>(&mut write_fn);

    #[cfg(feature = "high_arg_count")]
    generate_impls_for_n_args::<_, 7>(&mut write_fn);
    #[cfg(feature = "high_arg_count")]
    generate_impls_for_n_args::<_, 8>(&mut write_fn);

    #[cfg(feature = "fn_res")]
    fn_resource_impl
        .flush()
        .expect("Failed to flush writer for fn_resource_impl.rs");

    #[cfg(feature = "fn_meta")]
    fn_resource_meta_impl
        .flush()
        .expect("Failed to flush writer for fn_resource_meta_impl.rs");

    println!("cargo:rerun-if-changed=build.rs");
}

#[cfg(any(feature = "fn_meta", feature = "fn_res"))]
mod common {
    use std::{
        fmt::Write as _,
        fs::{File, OpenOptions},
        io::BufWriter,
        path::Path,
    };

    #[derive(Clone, Copy, Debug)]
    pub struct ArgExprs<'s> {
        pub args_csv: &'s str,
        pub arg_refs_csv: &'s str,
        pub arg_bounds_list: &'s str,
        pub resource_arg_borrows: &'s str,
        pub resource_arg_try_borrows: &'s str,
        pub resource_arg_vars: &'s str,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Ref {
        Immutable,
        Mutable,
    }

    pub fn open_impl_file(out_dir: &Path, file_name: &str) -> BufWriter<File> {
        let fn_resource_impl_path = out_dir.join(file_name);
        let fn_resource_impl = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(fn_resource_impl_path)
            .unwrap_or_else(|e| panic!("Failed to open `{file_name}`. Error: {e}"));
        BufWriter::new(fn_resource_impl)
    }

    pub fn generate_impls_for_n_args<FnWrite, const N: usize>(fn_write: &mut FnWrite)
    where
        FnWrite: FnMut(ArgExprs<'_>),
    {
        // "A0, A1"
        let args_csv = args_csv::<N>();

        // "    A0: 'static,\n    A1: 'static,"
        let arg_bounds_list = arg_bounds_list::<N>();

        arg_refs_combinations::<N>().for_each(|arg_refs| {
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
                            Ref::Immutable => write!(&mut arg_refs_csv, ", &A{index}"),
                            Ref::Mutable => write!(&mut arg_refs_csv, ", &mut A{index}"),
                        })
                        .expect("Failed to append to `arg_refs_csv` string.");
                }

                arg_refs_csv
            };

            // let a0 = resources.borrow::<A0>();
            // let mut a1 = resources.borrow_mut::<A1>();
            // ..
            let resource_arg_borrows = resource_arg_borrows(arg_refs);
            let resource_arg_try_borrows = resource_arg_try_borrows(arg_refs);

            // &*a0, &mut *a1
            let resource_arg_vars = resource_arg_vars::<N>(arg_refs);

            let args_csv = args_csv.as_str();
            let arg_refs_csv = arg_refs_csv.as_str();
            let arg_bounds_list = arg_bounds_list.as_str();
            let resource_arg_borrows = resource_arg_borrows.as_str();
            let resource_arg_try_borrows = resource_arg_try_borrows.as_str();
            let resource_arg_vars = resource_arg_vars.as_str();

            let arg_exprs = ArgExprs {
                args_csv,
                arg_refs_csv,
                arg_bounds_list,
                resource_arg_borrows,
                resource_arg_try_borrows,
                resource_arg_vars,
            };

            fn_write(arg_exprs);
        })
    }

    fn resource_arg_vars<const N: usize>(arg_refs: [Ref; N]) -> String {
        let mut resource_arg_vars = String::with_capacity(N * 10);
        let mut arg_refs_iter = arg_refs.iter().copied().enumerate();
        if let Some((index, arg_ref)) = arg_refs_iter.next() {
            match arg_ref {
                Ref::Immutable => write!(&mut resource_arg_vars, "&*a{index}"),
                Ref::Mutable => write!(&mut resource_arg_vars, "&mut *a{index}"),
            }
            .expect("Failed to append to `resource_arg_vars` string.")
        }
        arg_refs_iter
            .try_for_each(|(index, arg_ref)| match arg_ref {
                Ref::Immutable => write!(&mut resource_arg_vars, ", &*a{index}"),
                Ref::Mutable => write!(&mut resource_arg_vars, ", &mut *a{index}"),
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
                ),
                Ref::Mutable => writeln!(
                    &mut resource_arg_borrows,
                    "let mut a{index} = resources.borrow_mut::<A{index}>();",
                ),
            })
            .expect("Failed to append to `resource_arg_borrows` string.");
        resource_arg_borrows
    }

    fn resource_arg_try_borrows<const N: usize>(arg_refs: [Ref; N]) -> String {
        let mut resource_arg_try_borrows = String::with_capacity(N * 44);
        let mut arg_refs_iter = arg_refs.iter().copied().enumerate();
        arg_refs_iter
            .try_for_each(|(index, arg_ref)| match arg_ref {
                Ref::Immutable => writeln!(
                    &mut resource_arg_try_borrows,
                    "let a{index} = resources.try_borrow::<A{index}>()?;",
                ),
                Ref::Mutable => writeln!(
                    &mut resource_arg_try_borrows,
                    "let mut a{index} = resources.try_borrow_mut::<A{index}>()?;",
                ),
            })
            .expect("Failed to append to `resource_arg_try_borrows` string.");
        resource_arg_try_borrows
    }

    fn arg_refs_combinations<const N: usize>() -> impl Iterator<Item = [Ref; N]> {
        (0..(2 << (N - 1))).map(|m| {
            // `m` is the combination variation count.
            // Whether an argument is immutable or mutable is bed on its corresponding bit
            // value of `m`.

            let mut arg_refs: [Ref; N] = [Ref::Immutable; N];
            arg_refs
                .iter_mut()
                .enumerate()
                .for_each(move |(n, arg_ref)| {
                    // for N = 5
                    // m can be 0..32
                    // if 31 >> 5 is 0

                    if m >> n & 1 == 0 {
                        *arg_ref = Ref::Immutable;
                    } else {
                        *arg_ref = Ref::Mutable;
                    }
                });

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
                "\n    A{n}: std::fmt::Debug + Send + Sync + 'static,"
            )
            .expect("Failed to append to args_csv string.");

            #[cfg(not(feature = "debug"))]
            write!(&mut arg_bounds_list, "\n    A{n}: Send + Sync + 'static,")
                .expect("Failed to append to args_csv string.");
            arg_bounds_list
        })
    }

    fn args_csv<const N: usize>() -> String {
        let mut args_csv = String::with_capacity(N * 4);
        args_csv.push_str("A0");
        (1..N).fold(args_csv, |mut args_csv, n| {
            write!(&mut args_csv, ", A{n}").expect("Failed to append to args_csv string.");
            args_csv
        })
    }
}

#[cfg(feature = "fn_res")]
mod fn_resource_impl {
    use std::{
        fs::File,
        io::{BufWriter, Write},
    };

    use super::common::ArgExprs;

    pub fn write_fn_resource_impl(fn_resource_impl: &mut BufWriter<File>, arg_exprs: ArgExprs<'_>) {
        let ArgExprs {
            args_csv,
            arg_refs_csv,
            arg_bounds_list,
            resource_arg_borrows,
            resource_arg_try_borrows,
            resource_arg_vars,
        } = arg_exprs;

        #[cfg(feature = "fn_res_once")]
        write!(
            fn_resource_impl,
            r#"
impl<Fun, Ret, {args_csv}> FnResource<Fun, Ret, ({arg_refs_csv})>
where
    Fun: FnOnce({arg_refs_csv}) -> Ret + 'static,
    Ret: 'static,
    {arg_bounds_list}
{{
    pub fn call_once(self, resources: &Resources) -> Ret {{
        {resource_arg_borrows}

        (self.func)({resource_arg_vars})
    }}

    pub fn try_call_once(self, resources: &Resources) -> Result<Ret, BorrowFail> {{
        {resource_arg_try_borrows}

        let ret_value = (self.func)({resource_arg_vars});
        Ok(ret_value)
    }}
}}
"#,
        )
        .expect("Failed to write to fn_resource_impl.rs");

        #[cfg(feature = "fn_res_mut")]
        write!(
            fn_resource_impl,
            r#"
impl<Fun, Ret, {args_csv}> FnResource<Fun, Ret, ({arg_refs_csv})>
where
    Fun: FnMut({arg_refs_csv}) -> Ret + 'static,
    Ret: 'static,
    {arg_bounds_list}
{{
    pub fn call_mut(&mut self, resources: &Resources) -> Ret {{
        {resource_arg_borrows}

        (self.func)({resource_arg_vars})
    }}

    pub fn try_call_mut(&mut self, resources: &Resources) -> Result<Ret, BorrowFail> {{
        {resource_arg_try_borrows}

        let ret_value = (self.func)({resource_arg_vars});
        Ok(ret_value)
    }}
}}
"#,
        )
        .expect("Failed to write to fn_resource_impl.rs");

        write!(
            fn_resource_impl,
            r#"
impl<Fun, Ret, {args_csv}> FnResource<Fun, Ret, ({arg_refs_csv})>
where
    Fun: Fn({arg_refs_csv}) -> Ret + 'static,
    Ret: 'static,
    {arg_bounds_list}
{{
    pub fn call(&self, resources: &Resources) -> Ret {{
        {resource_arg_borrows}

        (self.func)({resource_arg_vars})
    }}

    pub fn try_call(&self, resources: &Resources) -> Result<Ret, BorrowFail> {{
        {resource_arg_try_borrows}

        let ret_value = (self.func)({resource_arg_vars});
        Ok(ret_value)
    }}
}}
"#,
        )
        .expect("Failed to write to fn_resource_impl.rs");
    }
}

#[cfg(feature = "fn_res_once")]
mod fn_res_once_impl {
    use std::{
        fs::File,
        io::{BufWriter, Write},
    };

    use super::common::ArgExprs;

    pub fn write_fn_res_once_impl(fn_resource_impl: &mut BufWriter<File>, arg_exprs: ArgExprs<'_>) {
        let ArgExprs {
            args_csv,
            arg_refs_csv,
            arg_bounds_list,
            ..
        } = arg_exprs;

        write!(
            fn_resource_impl,
            r#"
impl<Fun, Ret, {args_csv}> FnResOnce for FnResource<Fun, Ret, ({arg_refs_csv})>
where
    Fun: FnOnce({arg_refs_csv}) -> Ret + 'static,
    Ret: 'static,
    {arg_bounds_list}
{{
    type Ret = Ret;

    fn call_once(self, resources: &Resources) -> Ret {{
        Self::call_once(self, resources)
    }}

    fn try_call_once(self, resources: &Resources) -> Result<Ret, BorrowFail> {{
        Self::try_call_once(self, resources)
    }}
}}
"#,
        )
        .expect("Failed to write to fn_res_once_impl.rs");
    }
}

#[cfg(feature = "fn_res_mut")]
mod fn_res_mut_impl {
    use std::{
        fs::File,
        io::{BufWriter, Write},
    };

    use super::common::ArgExprs;

    pub fn write_fn_res_mut_impl(fn_resource_impl: &mut BufWriter<File>, arg_exprs: ArgExprs<'_>) {
        let ArgExprs {
            args_csv,
            arg_refs_csv,
            arg_bounds_list,
            ..
        } = arg_exprs;

        #[cfg(not(feature = "fn_res_once"))]
        let ret_type_str = "    type Ret = Ret;\n";
        #[cfg(feature = "fn_res_once")]
        let ret_type_str = "";

        write!(
            fn_resource_impl,
            r#"
impl<Fun, Ret, {args_csv}> FnResMut for FnResource<Fun, Ret, ({arg_refs_csv})>
where
    Fun: FnMut({arg_refs_csv}) -> Ret + 'static,
    Ret: 'static,
    {arg_bounds_list}
{{
{ret_type_str}
    fn call_mut(&mut self, resources: &Resources) -> Ret {{
        Self::call_mut(self, resources)
    }}

    fn try_call_mut(&mut self, resources: &Resources) -> Result<Ret, BorrowFail> {{
        Self::try_call_mut(self, resources)
    }}
}}
"#,
        )
        .expect("Failed to write to fn_res_mut_impl.rs");
    }
}

#[cfg(feature = "fn_res")]
mod fn_res_impl {
    use std::{
        fs::File,
        io::{BufWriter, Write},
    };

    use super::common::ArgExprs;

    pub fn write_fn_res_impl(fn_resource_impl: &mut BufWriter<File>, arg_exprs: ArgExprs<'_>) {
        let ArgExprs {
            args_csv,
            arg_refs_csv,
            arg_bounds_list,
            ..
        } = arg_exprs;

        #[cfg(not(feature = "fn_res_mut"))]
        let ret_type_str = "    type Ret = Ret;\n";
        #[cfg(feature = "fn_res_mut")]
        let ret_type_str = "";

        write!(
            fn_resource_impl,
            r#"
impl<Fun, Ret, {args_csv}> FnRes for FnResource<Fun, Ret, ({arg_refs_csv})>
where
    Fun: Fn({arg_refs_csv}) -> Ret + 'static,
    Ret: 'static,
    {arg_bounds_list}
{{
{ret_type_str}
    fn call(&self, resources: &Resources) -> Ret {{
        Self::call(self, resources)
    }}

    fn try_call(&self, resources: &Resources) -> Result<Ret, BorrowFail> {{
        Self::try_call(self, resources)
    }}
}}
"#,
        )
        .expect("Failed to write to fn_res_impl.rs");
    }
}

#[cfg(all(feature = "fn_res", feature = "fn_meta"))]
mod fn_resource_meta_impl {
    use std::{
        fs::File,
        io::{BufWriter, Write},
    };

    use super::common::ArgExprs;

    pub fn write_fn_resource_meta_impl(
        fn_resource_meta_impl: &mut BufWriter<File>,
        arg_exprs: ArgExprs<'_>,
    ) {
        let ArgExprs {
            args_csv,
            arg_refs_csv,
            arg_bounds_list,
            resource_arg_borrows: _,
            resource_arg_try_borrows: _,
            resource_arg_vars: _,
        } = arg_exprs;

        write!(
            fn_resource_meta_impl,
            r#"
impl<Fun, Ret, {args_csv}> fn_meta::FnMeta for FnResource<Fun, Ret, ({arg_refs_csv})>
where
    Fun: FnOnce({arg_refs_csv}) -> Ret + 'static,
    Ret: 'static,
    {arg_bounds_list}
{{
    fn borrows() -> fn_meta::TypeIds {{
        <fn_meta::FnMetadata<Fun, Ret, ({arg_refs_csv})> as fn_meta::FnMeta>::borrows()
    }}

    fn borrow_muts() -> fn_meta::TypeIds {{
        <fn_meta::FnMetadata<Fun, Ret, ({arg_refs_csv})> as fn_meta::FnMeta>::borrow_muts()
    }}
}}

impl<Fun, Ret, {args_csv}> fn_meta::FnMetaDyn for FnResource<Fun, Ret, ({arg_refs_csv})>
where
    Fun: FnOnce({arg_refs_csv}) -> Ret + 'static,
    Ret: 'static,
    {arg_bounds_list}
{{
    fn borrows(&self) -> fn_meta::TypeIds {{
        fn_meta::FnMetaExt::meta(&self.func).borrows()
    }}

    fn borrow_muts(&self) -> fn_meta::TypeIds {{
        fn_meta::FnMetaExt::meta(&self.func).borrow_muts()
    }}
}}
"#,
        )
        .expect("Failed to write to fn_resource_meta_impl.rs");
    }
}
