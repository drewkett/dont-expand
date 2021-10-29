/*!
This library provides macro implementations for macros found in the rust std
library. The purpose of the macros in this library is to do nothing. The 
macros are intended to be imported as needed to shadow the std macros. This
is useful when using `cargo expand` to see what a macro might be doing as it
will effectively hide the typical expansion of the std macros which typically
just pollute the output.

Included macros are: 
    assert, assert_eq, assert_ne,
    cfg, column, compile_error, concat, dbg, 
    debug_assert, debug_assert_eq, debug_assert_ne,
    env, eprint, eprintln, file, format, format_args,
    include, include_bytes, include_str, is_x86_feature_detected,
    line, matches, module_path, option_env, panic, print, println,
    thread_local, todo, unimplemented, unreachable,
    vec, write, writeln

And the following derive macros:
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord

The following example would only show the expansion of the `Default` 
macro
```
use dont_expand::{Debug, Clone};

#[derive(Debug, Clone, Default)]
struct Foo;
```

Glob imports don't work with dont-expand as that throws an error by the 
compiler since they shadow the std macros. 
*/
use proc_macro::TokenStream;
use paste::paste;

#[macro_use]
mod macros {
    macro_rules! gen_dummy_derive {
        ($($name: ident),*) => {
            paste! {
                $(
                    #[proc_macro_derive($name)]
                    pub fn [< $name:lower _derive_macro >](_inp: TokenStream) -> TokenStream {
                        TokenStream::new()
                    }
                )*
            }
        }
    }

    macro_rules! gen_dummy_proc_macro {
        ($($name: ident),*) => {
            paste! {
                $(
                    #[proc_macro]
                    pub fn $name(inp: TokenStream) -> TokenStream {
                        let mut out = String::new();
                        out.push_str(r###"r##""###);
                        out.push_str(stringify!([<$name:lower>]));
                        out.push_str("!(");
                        out.push_str(&inp.to_string());
                        out.push_str(r###")"##"###);
                        match out.parse() {
                            Ok(s) => s,
                            Err(_) => {
                                use std::io::Write;
                                let _ = std::io::stderr().write_all(b"failed to generate dummy string in hiding call to $name");
                                TokenStream::new()
                            }
                        }
                    }
                )*
            }
        }
    }
}

gen_dummy_derive!(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord);
gen_dummy_proc_macro!(
    assert, assert_eq, assert_ne,
    cfg, column, compile_error, concat, dbg, 
    debug_assert, debug_assert_eq, debug_assert_ne,
    env, eprint, eprintln, file, format, format_args,
    include, include_bytes, include_str, is_x86_feature_detected,
    line, matches, module_path, option_env, panic, print, println,
    thread_local, todo, unimplemented, unreachable,
    vec, write, writeln
);
