// Copyright takubokudori.
// This source code is licensed under the MIT or Apache-2.0 license.
//! # Windy-macros
//!
//! [![crates.io](https://img.shields.io/crates/v/windy-macros.svg)](https://crates.io/crates/windy-macros)
//! [![docs.rs](https://docs.rs/windy-macros/badge.svg)](https://docs.rs/windy-macros)
//!
//! Macros for [Windy](https://github.com/takubokudori/windy).
//!
//! # Features
//!
//! Converts UTF-8 literals to the following Windy string and byte-array types at compile time:
//!
//! - `WString` using `wstring!` or `wstring_lossy!`.
//! - `AString` using `astring!` or `astring_lossy!`.
//! - `&WStr` using `wstr!` or `wstr_lossy!`.
//! - `&AStr` using `astr!` or `astr_lossy!`.
//! - `[u8]` using `aarr!` or `aarr_lossy!`.
//! - `[u16]` using `warr!` or `warr_lossy!`.
//!
//! at compile time.
//!
//! # License
//!
//! This software is released under the MIT or Apache-2.0 License, see LICENSE-MIT or LICENSE-APACHE.
#[cfg(not(windows))]
compile_error!("windy-macros is Windows-host-only.");
use crate::convert::get_system_default_acp;
use std::str::FromStr;
use syn::{
    Lit, LitInt, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};
use windy::*;

mod convert;
mod raw;

#[allow(unused)]
pub(crate) const WC_ERR_INVALID_CHARS: u32 = 0x80;
#[allow(unused)]
pub(crate) const WC_NO_BEST_FIT_CHARS: u32 = 0x400;

/// Returns [`String`].
fn lit_to_string(ast: Lit) -> String {
    match ast {
        Lit::Str(x) => x.value(),
        Lit::Char(x) => x.value().to_string(),
        Lit::Int(x) => x.base10_digits().to_string(),
        Lit::Float(x) => x.base10_digits().to_string(),
        Lit::Bool(x) => x.value.to_string(),
        _ => panic!("Unsupported literal"),
    }
}

/// Returns `[u8]`.
macro_rules! lit_to_bs_lossy {
    ($x:ident, $ast:expr) => {{
        let s = lit_to_string($ast);
        let s = $x::from_utf8_lossy(&s);
        let bytes = s.as_bytes_with_nul();
        format!("{:?}", bytes)
    }};
    (@a $cp:expr, $ast:expr) => {{
        let s = lit_to_string($ast);
        let s =
            windy::DAString::try_from_utf8_lossy($cp, &s).unwrap_or_else(|x| {
                panic!(
                    "{:?} Couldn't be converted to code page {} : {:X?}",
                    s, $cp, x
                )
            });
        let bytes = s.as_bytes_with_nul();
        format!("{:?}", bytes)
    }};
}

/// Returns `[u8]`.
macro_rules! lit_to_bs {
    ($x:ident, $ast:expr) => {{
        let s = lit_to_string($ast);
        let s = $x::from_utf8(&s).unwrap_or_else(|x| {
            panic!(
                concat!(
                    "{:?} Couldn't be converted to ",
                    stringify!($x),
                    ": {:X?}"
                ),
                s, x
            )
        });
        let bytes = s.as_bytes_with_nul();
        format!("{:?}", bytes)
    }};
    (@a $cp:expr, $ast:expr) => {{
        let s = lit_to_string($ast);
        let s = windy::DAString::from_utf8($cp, &s).unwrap_or_else(|x| {
            panic!(
                "{:?} Couldn't be converted to code page {} : {:X?}",
                s, $cp, x
            )
        });
        let bytes = s.as_bytes_with_nul();
        format!("{:?}", bytes)
    }};
}

/// Returns [`windy::WString`].
///
/// If an invalid value is passed, this macro will be panicked.
///
/// # Example
///
/// ```
/// use windy_macros::wstring;
///
/// let s = wstring!("test");
/// println!("{:?}", s); // "test"
/// let s = wstring!(4649);
/// println!("{:?}", s); // "4649"
/// ```
#[proc_macro]
pub fn wstring(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = lit_to_bs!(WString, ast);
    let ts = format!(
        "unsafe {{ ::windy::WString::from_vec_with_nul_unchecked({}) }}",
        bs
    );

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns [`windy::WString`].
///
/// # Example
///
/// ```
/// use windy_macros::wstring_lossy;
///
/// let s = wstring_lossy!("test");
/// println!("{:?}", s); // "test"
/// let s = wstring_lossy!(4649);
/// println!("{:?}", s); // "4649"
/// ```
#[proc_macro]
pub fn wstring_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = lit_to_bs_lossy!(WString, ast);
    let ts = format!(
        "unsafe {{ ::windy::WString::from_vec_with_nul_unchecked({}) }}",
        bs
    );

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

fn code_page_literal_value(code_page: &LitInt) -> u32 {
    code_page
        .base10_parse::<u32>()
        .expect("code page must be a u32 integer literal")
}

struct AStringInput {
    code_page: LitInt,
    value: Lit,
}

impl Parse for AStringInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let code_page = input.parse()?;
        input.parse::<Token![,]>()?;
        let value = input.parse()?;

        if !input.is_empty() {
            return Err(input.error(
                "expected exactly two arguments: code page integer literal \
                 and literal",
            ));
        }

        Ok(Self { code_page, value })
    }
}

/// Returns [`windy::AString`] encoded with an explicit ANSI code page.
///
/// The first argument must be a `u32` integer literal such as `932`. Constant
/// paths are not accepted.
///
/// Panics during macro expansion if the literal cannot be represented in the
/// specified code page. Use [`astring_lossy!`] to replace unrepresentable
/// characters when possible.
///
/// # Example
///
/// ```
/// use windy_macros::astring;
///
/// let s = astring!(932, "test");
/// println!("{:?}", s); // "test"
/// ```
#[proc_macro]
pub fn astring(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(ast as AStringInput);

    let code_page = code_page_literal_value(&input.code_page);
    let code_page_arg = code_page.to_string();

    let bs = lit_to_bs!(@a code_page, input.value);
    let ts = format!(
        "unsafe {{ ::windy::AString::<{{ {} \
         }}>::from_vec_with_nul_unchecked({}) }}",
        code_page_arg, bs
    );

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns [`windy::AString`] encoded with an explicit ANSI code page, replacing
/// unrepresentable characters when possible.
///
/// The first argument must be a `u32` integer literal such as `932`. Constant
/// paths are not accepted.
///
/// Panics during macro expansion if the code page is invalid or the underlying
/// conversion API fails.
///
/// # Example
///
/// ```
/// use windy_macros::astring_lossy;
///
/// let s = astring_lossy!(932, "test");
/// println!("{:?}", s); // "test"
/// ```
#[proc_macro]
pub fn astring_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(ast as AStringInput);

    let code_page = code_page_literal_value(&input.code_page);
    let code_page_arg = code_page.to_string();

    let bs = lit_to_bs_lossy!(@a code_page, input.value);
    let ts = format!(
        "unsafe {{ ::windy::AString::<{{ {} \
         }}>::from_vec_with_nul_unchecked({}) }}",
        code_page_arg, bs
    );

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns [`windy::ACPString`] encoded with the build host's system default ACP.
///
/// The generated bytes depend on the build machine. Prefer [`astring!`] when
/// reproducible output is required.
///
/// If an invalid value is passed, this macro will be panicked.
///
/// # Example
///
/// ```
/// use windy_macros::acpstring;
///
/// let s = acpstring!("test");
/// println!("{:?}", s); // "test"
/// let s = acpstring!(4649);
/// println!("{:?}", s); // "4649"
/// ```
#[proc_macro]
pub fn acpstring(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let default_cp =
        get_system_default_acp().expect("Failed to get system default acp");
    let bs = lit_to_bs!(@a default_cp, ast);
    let ts = format!(
        "unsafe {{ ::windy::ACPString::from_vec_with_nul_unchecked({}) }}",
        bs
    );

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns [`windy::ACPString`] encoded with the build host's system default ACP,
/// replacing unrepresentable characters when possible.
///
/// The generated bytes depend on the build machine. Prefer [`astring_lossy!`]
/// when reproducible output is required.
///
/// # Example
///
/// ```
/// use windy_macros::acpstring_lossy;
///
/// let s = acpstring_lossy!("test");
/// println!("{:?}", s); // "test"
/// let s = acpstring_lossy!(4649);
/// println!("{:?}", s); // "4649"
/// ```
#[proc_macro]
pub fn acpstring_lossy(
    ast: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let default_cp =
        get_system_default_acp().expect("Failed to get system default acp");
    let bs = lit_to_bs_lossy!(@a default_cp, ast);
    let ts = format!(
        "unsafe {{ ::windy::ACPString::from_vec_with_nul_unchecked({}) }}",
        bs
    );

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns &[`windy::WStr`].
///
/// If an invalid value is passed, this macro will be panicked.
///
/// # Example
///
/// ```
/// use windy::WString;
/// use windy_macros::wstr;
///
/// let x = wstr!("test");
/// assert_eq!(
///     WString::from_utf8_lossy("test").as_bytes_with_nul(),
///     x.as_bytes_with_nul()
/// );
/// ```
#[proc_macro]
pub fn wstr(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = lit_to_bs!(WString, ast);
    let ts = format!(
        "unsafe {{ ::windy::WStr::from_bytes_with_nul_unchecked(&{}) }}",
        bs
    );

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns &[`windy::WStr`].
///
/// # Example
///
/// ```
/// use windy::WString;
/// use windy_macros::wstr_lossy;
///
/// let x = wstr_lossy!("test");
/// assert_eq!(
///     WString::from_utf8_lossy("test").as_bytes_with_nul(),
///     x.as_bytes_with_nul()
/// );
/// ```
#[proc_macro]
pub fn wstr_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = lit_to_bs_lossy!(WString, ast);
    let ts = format!(
        "unsafe {{ ::windy::WStr::from_bytes_with_nul_unchecked(&{}) }}",
        bs
    );

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns &[`windy::AStr`] encoded with an explicit ANSI code page.
///
/// The first argument must be a `u32` integer literal such as `932`. Constant
/// paths are not accepted.
///
/// Panics during macro expansion if the literal cannot be represented in the
/// specified code page. Use [`astr_lossy!`] to replace unrepresentable
/// characters when possible.
///
/// # Example
///
/// ```
/// use windy::AString;
/// use windy_macros::astr;
///
/// let x = astr!(932, "test");
/// assert_eq!(
///     AString::<932>::from_utf8_lossy("test").as_bytes_with_nul(),
///     x.as_bytes_with_nul()
/// );
/// ```
#[proc_macro]
pub fn astr(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(ast as AStringInput);

    let code_page = code_page_literal_value(&input.code_page);
    let code_page_arg = code_page.to_string();

    let bs = lit_to_bs!(@a code_page, input.value);
    let ts = format!(
        "unsafe {{ ::windy::AStr::<{{ {} \
         }}>::from_bytes_with_nul_unchecked(&{}) }}",
        code_page_arg, bs
    );

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns &[`windy::AStr`] encoded with an explicit ANSI code page, replacing
/// unrepresentable characters when possible.
///
/// The first argument must be a `u32` integer literal such as `932`. Constant
/// paths are not accepted.
///
/// Panics during macro expansion if the code page is invalid or the underlying
/// conversion API fails.
///
/// # Example
///
/// ```
/// use windy::AString;
/// use windy_macros::astr_lossy;
///
/// let x = astr_lossy!(932, "test");
/// assert_eq!(
///     AString::<932>::from_utf8_lossy("test").as_bytes_with_nul(),
///     x.as_bytes_with_nul()
/// );
/// ```
#[proc_macro]
pub fn astr_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(ast as AStringInput);

    let code_page = code_page_literal_value(&input.code_page);
    let code_page_arg = code_page.to_string();

    let bs = lit_to_bs_lossy!(@a code_page, input.value);
    let ts = format!(
        "unsafe {{ ::windy::AStr::<{{ {} \
         }}>::from_bytes_with_nul_unchecked(&{}) }}",
        code_page_arg, bs
    );

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns &[`windy::AStr`] encoded with the build host's system default ACP.
///
/// The generated bytes depend on the build machine. Prefer [`astr!`] when
/// reproducible output is required.
///
/// If an invalid value is passed, this macro will be panicked.
///
/// # Example
///
/// ```
/// use windy::ACPString;
/// use windy_macros::acpstr;
///
/// let x = acpstr!("test");
/// assert_eq!(
///     ACPString::from_utf8_lossy("test").as_bytes_with_nul(),
///     x.as_bytes_with_nul()
/// );
/// ```
#[proc_macro]
pub fn acpstr(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let default_cp =
        get_system_default_acp().expect("Failed to get system default acp");
    let bs = lit_to_bs!(@a default_cp, ast);
    let ts = format!(
        "unsafe {{ ::windy::AStr::<0>::from_bytes_with_nul_unchecked(&{}) }}",
        bs
    );

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns &[`windy::AStr`] encoded with the build host's system default ACP,
/// replacing unrepresentable characters when possible.
///
/// The generated bytes depend on the build machine. Prefer [`astr_lossy!`] when
/// reproducible output is required.
///
/// # Example
///
/// ```
/// use windy::ACPString;
/// use windy_macros::acpstr_lossy;
///
/// let x = acpstr_lossy!("test");
/// assert_eq!(
///     ACPString::from_utf8_lossy("test").as_bytes_with_nul(),
///     x.as_bytes_with_nul()
/// );
/// ```
#[proc_macro]
pub fn acpstr_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let default_cp =
        get_system_default_acp().expect("Failed to get system default acp");
    let bs = lit_to_bs_lossy!(@a default_cp, ast);
    let ts = format!(
        "unsafe {{ ::windy::AStr::<0>::from_bytes_with_nul_unchecked(&{}) }}",
        bs
    );

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns `[u16]`.
///
/// If an invalid value is passed, this macro will be panicked.
///
/// # Example
///
/// ```
/// use windy::WString;
/// use windy_macros::warr;
///
/// let b = &warr!("test");
/// assert_eq!(WString::from_utf8_lossy("test").as_bytes_with_nul(), b);
/// let b = &warr!(4649);
/// assert_eq!(WString::from_utf8_lossy("4649").as_bytes_with_nul(), b);
/// ```
#[proc_macro]
pub fn warr(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let ts = lit_to_bs!(WString, ast);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns `[u16]`.
///
/// # Example
///
/// ```
/// use windy::WString;
/// use windy_macros::warr_lossy;
///
/// let b = &warr_lossy!("test");
/// assert_eq!(WString::from_utf8_lossy("test").as_bytes_with_nul(), b);
/// let b = &warr_lossy!(4649);
/// assert_eq!(WString::from_utf8_lossy("4649").as_bytes_with_nul(), b);
/// ```
#[proc_macro]
pub fn warr_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let ts = lit_to_bs_lossy!(WString, ast);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns `[u8]` encoded with an explicit ANSI code page.
///
/// The first argument must be a `u32` integer literal such as `932`. Constant
/// paths are not accepted.
///
/// Panics during macro expansion if the literal cannot be represented in the
/// specified code page. Use [`aarr_lossy!`] to replace unrepresentable
/// characters when possible.
///
/// # Example
///
/// ```
/// use windy::AString;
/// use windy_macros::aarr;
///
/// let b = &aarr!(932, "test");
/// assert_eq!(
///     AString::<932>::from_utf8_lossy("test").as_bytes_with_nul(),
///     b
/// );
/// ```
#[proc_macro]
pub fn aarr(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(ast as AStringInput);

    let code_page = code_page_literal_value(&input.code_page);
    let ts = lit_to_bs!(@a code_page, input.value);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns `[u8]` encoded with an explicit ANSI code page, replacing
/// unrepresentable characters when possible.
///
/// The first argument must be a `u32` integer literal such as `932`. Constant
/// paths are not accepted.
///
/// Panics during macro expansion if the code page is invalid or the underlying
/// conversion API fails.
///
/// # Example
///
/// ```
/// use windy::AString;
/// use windy_macros::aarr_lossy;
///
/// let b = &aarr_lossy!(932, "test");
/// assert_eq!(
///     AString::<932>::from_utf8_lossy("test").as_bytes_with_nul(),
///     b
/// );
/// ```
#[proc_macro]
pub fn aarr_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(ast as AStringInput);

    let code_page = code_page_literal_value(&input.code_page);
    let ts = lit_to_bs_lossy!(@a code_page, input.value);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns `[u8]` encoded with the build host's system default ACP.
///
/// The generated bytes depend on the build machine. Prefer [`aarr!`] when
/// reproducible output is required.
///
/// If an invalid value is passed, this macro will be panicked.
///
/// # Example
///
/// ```
/// use windy::ACPString;
/// use windy_macros::acparr;
///
/// let b = &acparr!("test");
/// assert_eq!(ACPString::from_utf8_lossy("test").as_bytes_with_nul(), b);
/// let b = &acparr!(4649);
/// assert_eq!(ACPString::from_utf8_lossy("4649").as_bytes_with_nul(), b);
/// ```
#[proc_macro]
pub fn acparr(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let default_cp =
        get_system_default_acp().expect("Failed to get system default acp");
    let ts = lit_to_bs!(@a default_cp, ast);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns `[u8]` encoded with the build host's system default ACP, replacing
/// unrepresentable characters when possible.
///
/// The generated bytes depend on the build machine. Prefer [`aarr_lossy!`] when
/// reproducible output is required.
///
/// # Example
///
/// ```
/// use windy::ACPString;
/// use windy_macros::acparr_lossy;
///
/// let b = &acparr_lossy!("test");
/// assert_eq!(ACPString::from_utf8_lossy("test").as_bytes_with_nul(), b);
/// let b = &acparr_lossy!(4649);
/// assert_eq!(ACPString::from_utf8_lossy("4649").as_bytes_with_nul(), b);
/// ```
#[proc_macro]
pub fn acparr_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let default_cp =
        get_system_default_acp().expect("Failed to get system default acp");
    let ts = lit_to_bs_lossy!(@a default_cp, ast);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}
