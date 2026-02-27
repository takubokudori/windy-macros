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
//! Converts UTF-8 `&str` to:
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
use crate::convert::*;
use std::str::FromStr;
use syn::{Lit, parse_macro_input};
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
    ($x:ident, $ast:ident) => {{
        let s = lit_to_string($ast);
        let s = $x::from_str_lossy(&s);
        let bytes = s.to_bytes_with_nul();
        format!("{:?}", bytes)
    }};
}

/// Returns `[u8]`.
macro_rules! lit_to_bs {
    ($x:ident, $ast:ident) => {{
        let s = lit_to_string($ast);
        let s = $x::from_str(&s).unwrap_or_else(|x| {
            panic!(
                concat!(
                    "{} couldn't be converted to ",
                    stringify!($x),
                    ": {:X?}"
                ),
                s, x
            )
        });
        let bytes = s.to_bytes_with_nul();
        format!("{:?}", bytes)
    }};
}

/// When compiling Rust code, the default code page ends up being changed to `CP_UTF8`, which causes mojibake when converting to ANSI.
/// Therefore, we need to obtain the original code page from before the change and use it for conversion.
fn utf8_lit_to_ansi(ast: Lit) -> String {
    let s = lit_to_string(ast);
    let default_cp =
        get_system_default_acp().expect("Failed to get system default acp");
    // UTF-8 -> Unicode -> ANSI
    let s = utf8_to_wide(&s).unwrap();

    let mut v = wide_to_mb(default_cp, s.as_slice())
        .expect("Failed to convert Wide string to MultiByte string");
    v.reserve_exact(1);
    v.push(0);
    format!("{:?}", v)
}

fn utf8_lit_to_ansi_lossy(ast: Lit) -> String {
    let s = lit_to_string(ast);
    let default_cp =
        get_system_default_acp().expect("Failed to get system default acp");
    // UTF-8 -> Unicode -> ANSI
    let s = utf8_to_wide(&s).unwrap();

    let mut v = wide_to_mb_lossy(default_cp, s.as_slice())
        .expect("Failed to convert Wide string to MultiByte string");
    v.reserve_exact(1);
    v.push(0);
    format!("{:?}", v)
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
    let ts =
        format!("unsafe {{ ::windy::WString::new_nul_unchecked({}) }}", bs);

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
    let ts =
        format!("unsafe {{ ::windy::WString::new_nul_unchecked({}) }}", bs);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns [`windy::AString`].
///
/// If an invalid value is passed, this macro will be panicked.
///
/// # Example
///
/// ```
/// use windy_macros::astring;
///
/// let s = astring!("test");
/// println!("{:?}", s); // "test"
/// let s = astring!(4649);
/// println!("{:?}", s); // "4649"
/// ```
#[proc_macro]
pub fn astring(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = lit_to_bs!(AString, ast);
    let ts =
        format!("unsafe {{ ::windy::AString::new_nul_unchecked({}) }}", bs);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns [`windy::AString`].
///
/// # Example
///
/// ```
/// use windy_macros::astring_lossy;
///
/// let s = astring_lossy!("test");
/// println!("{:?}", s); // "test"
/// let s = astring_lossy!(4649);
/// println!("{:?}", s); // "4649"
/// ```
#[proc_macro]
pub fn astring_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = utf8_lit_to_ansi_lossy(ast);
    let ts =
        format!("unsafe {{ ::windy::AString::new_nul_unchecked({}) }}", bs);

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
///     WString::from_str_lossy("test").to_bytes_with_nul(),
///     x.to_bytes_with_nul()
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
///     WString::from_str_lossy("test").to_bytes_with_nul(),
///     x.to_bytes_with_nul()
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

/// Returns &[`windy::AStr`].
///
/// If an invalid value is passed, this macro will be panicked.
///
/// # Example
///
/// ```
/// use windy::AString;
/// use windy_macros::astr;
///
/// let x = astr!("test");
/// assert_eq!(
///     AString::from_str_lossy("test").to_bytes_with_nul(),
///     x.to_bytes_with_nul()
/// );
/// ```
#[proc_macro]
pub fn astr(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = utf8_lit_to_ansi(ast);
    let ts = format!(
        "unsafe {{ ::windy::AStr::from_bytes_with_nul_unchecked(&{}) }}",
        bs
    );

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns &[`windy::AStr`].
///
/// # Example
///
/// ```
/// use windy::AString;
/// use windy_macros::astr_lossy;
///
/// let x = astr_lossy!("test");
/// assert_eq!(
///     AString::from_str_lossy("test").to_bytes_with_nul(),
///     x.to_bytes_with_nul()
/// );
/// ```
#[proc_macro]
pub fn astr_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = utf8_lit_to_ansi_lossy(ast);
    let ts = format!(
        "unsafe {{ ::windy::AStr::from_bytes_with_nul_unchecked(&{}) }}",
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
/// assert_eq!(WString::from_str_lossy("test").to_bytes_with_nul(), b);
/// let b = &warr!(4649);
/// assert_eq!(WString::from_str_lossy("4649").to_bytes_with_nul(), b);
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
/// assert_eq!(WString::from_str_lossy("test").to_bytes_with_nul(), b);
/// let b = &warr_lossy!(4649);
/// assert_eq!(WString::from_str_lossy("4649").to_bytes_with_nul(), b);
/// ```
#[proc_macro]
pub fn warr_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let ts = lit_to_bs_lossy!(WString, ast);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns `[u8]`.
///
/// If an invalid value is passed, this macro will be panicked.
///
/// # Example
///
/// ```
/// use windy::AString;
/// use windy_macros::aarr;
///
/// let b = &aarr!("test");
/// assert_eq!(AString::from_str_lossy("test").to_bytes_with_nul(), b);
/// let b = &aarr!(4649);
/// assert_eq!(AString::from_str_lossy("4649").to_bytes_with_nul(), b);
/// ```
#[proc_macro]
pub fn aarr(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let ts = utf8_lit_to_ansi(ast);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns `[u8]`.
///
/// # Example
///
/// ```
/// use windy::AString;
/// use windy_macros::aarr_lossy;
///
/// let b = &aarr_lossy!("test");
/// assert_eq!(AString::from_str_lossy("test").to_bytes_with_nul(), b);
/// let b = &aarr_lossy!(4649);
/// assert_eq!(AString::from_str_lossy("4649").to_bytes_with_nul(), b);
/// ```
#[proc_macro]
pub fn aarr_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let ts = utf8_lit_to_ansi_lossy(ast);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}
