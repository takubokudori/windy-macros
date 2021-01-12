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
#![cfg(windows)]

use std::str::FromStr;
use syn::{parse_macro_input, Lit};
use windy::*;

/// Returns [`String`].
fn lit_to_string(ast: Lit) -> String {
    let s = match ast {
        Lit::Str(x) => x.value(),
        Lit::Char(x) => x.value().to_string(),
        Lit::Int(x) => x.base10_digits().to_string(),
        Lit::Float(x) => x.base10_digits().to_string(),
        Lit::Bool(x) => x.value.to_string(),
        _ => panic!("Unsupported literal"),
    };

    s
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
            panic!(format!(
                concat!(
                    "{} couldn't be converted to ",
                    stringify!($x),
                    ": {:X?}"
                ),
                s, x
            ))
        });
        let bytes = s.to_bytes_with_nul();
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
/// use windy::WString;
/// use windy_macros::wstring;
///
/// let s: WString = wstring!("test");
/// println!("{:?}", s); // "test"
/// let s: WString = wstring!(4649);
/// println!("{:?}", s); // "4649"
/// ```
#[proc_macro]
pub fn wstring(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = lit_to_bs!(WString, ast);
    let ts = format!("unsafe {{ WString::new_nul_unchecked({}) }}", bs);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns [`windy::WString`].
///
/// # Example
///
/// ```
/// use windy::WString;
/// use windy_macros::wstring_lossy;
///
/// let s: WString = wstring_lossy!("test");
/// println!("{:?}", s); // "test"
/// let s: WString = wstring_lossy!(4649);
/// println!("{:?}", s); // "4649"
/// ```
#[proc_macro]
pub fn wstring_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = lit_to_bs_lossy!(WString, ast);
    let ts = format!("unsafe {{ WString::new_nul_unchecked({}) }}", bs);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns [`windy::AString`].
///
/// If an invalid value is passed, this macro will be panicked.
///
/// # Example
///
/// ```
/// use windy::AString;
/// use windy_macros::astring;
///
/// let s: AString = astring!("test");
/// println!("{:?}", s); // "test"
/// let s: AString = astring!(4649);
/// println!("{:?}", s); // "4649"
/// ```
#[proc_macro]
pub fn astring(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = lit_to_bs!(AString, ast);
    let ts = format!("unsafe {{ AString::new_nul_unchecked({}) }}", bs);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns [`windy::AString`].
///
/// # Example
///
/// ```
/// use windy::AString;
/// use windy_macros::astring_lossy;
///
/// let s: AString = astring_lossy!("test");
/// println!("{:?}", s); // "test"
/// let s: AString = astring_lossy!(4649);
/// println!("{:?}", s); // "4649"
/// ```
#[proc_macro]
pub fn astring_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = lit_to_bs_lossy!(AString, ast);
    let ts = format!("unsafe {{ AString::new_nul_unchecked({}) }}", bs);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns &[`windy::WStr`].
///
/// If an invalid value is passed, this macro will be panicked.
///
/// # Example
///
/// ```
/// use windy::{WStr, WString};
/// use windy_macros::wstr;
///
/// let x: &WStr = wstr!("test");
/// assert_eq!(
///     WString::from_str_lossy("test").to_bytes_with_nul(),
///     x.to_bytes_with_nul()
/// );
/// ```
#[proc_macro]
pub fn wstr(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = lit_to_bs!(WString, ast);
    let ts =
        format!("unsafe {{ WStr::from_bytes_with_nul_unchecked(&{}) }}", bs);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns &[`windy::WStr`].
///
/// # Example
///
/// ```
/// use windy::{WStr, WString};
/// use windy_macros::wstr_lossy;
///
/// let x: &WStr = wstr_lossy!("test");
/// assert_eq!(
///     WString::from_str_lossy("test").to_bytes_with_nul(),
///     x.to_bytes_with_nul()
/// );
/// ```
#[proc_macro]
pub fn wstr_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = lit_to_bs_lossy!(WString, ast);
    let ts =
        format!("unsafe {{ WStr::from_bytes_with_nul_unchecked(&{}) }}", bs);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns &[`windy::AStr`].
///
/// If an invalid value is passed, this macro will be panicked.
///
/// # Example
///
/// ```
/// use windy::{AStr, AString};
/// use windy_macros::astr;
///
/// let x: &AStr = astr!("test");
/// assert_eq!(
///     AString::from_str_lossy("test").to_bytes_with_nul(),
///     x.to_bytes_with_nul()
/// );
/// ```
#[proc_macro]
pub fn astr(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = lit_to_bs!(AString, ast);
    let ts =
        format!("unsafe {{ AStr::from_bytes_with_nul_unchecked(&{}) }}", bs);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}

/// Returns &[`windy::AStr`].
///
/// # Example
///
/// ```
/// use windy::{AStr, AString};
/// use windy_macros::astr_lossy;
///
/// let x: &AStr = astr_lossy!("test");
/// assert_eq!(
///     AString::from_str_lossy("test").to_bytes_with_nul(),
///     x.to_bytes_with_nul()
/// );
/// ```
#[proc_macro]
pub fn astr_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let bs = lit_to_bs_lossy!(AString, ast);
    let ts =
        format!("unsafe {{ AStr::from_bytes_with_nul_unchecked(&{}) }}", bs);

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
/// let b: &[u16] = &warr!("test");
/// assert_eq!(WString::from_str_lossy("test").to_bytes_with_nul(), b);
/// let b: &[u16] = &warr!(4649);
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
/// let b: &[u16] = &warr_lossy!("test");
/// assert_eq!(WString::from_str_lossy("test").to_bytes_with_nul(), b);
/// let b: &[u16] = &warr_lossy!(4649);
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
/// let b: &[u8] = &aarr!("test");
/// assert_eq!(AString::from_str_lossy("test").to_bytes_with_nul(), b);
/// let b: &[u8] = &aarr!(4649);
/// assert_eq!(AString::from_str_lossy("4649").to_bytes_with_nul(), b);
/// ```
#[proc_macro]
pub fn aarr(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let ts = lit_to_bs!(AString, ast);

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
/// let b: &[u8] = &aarr_lossy!("test");
/// assert_eq!(AString::from_str_lossy("test").to_bytes_with_nul(), b);
/// let b: &[u8] = &aarr_lossy!(4649);
/// assert_eq!(AString::from_str_lossy("4649").to_bytes_with_nul(), b);
/// ```
#[proc_macro]
pub fn aarr_lossy(ast: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(ast as Lit);

    let ts = lit_to_bs_lossy!(AString, ast);

    proc_macro::TokenStream::from_str(&ts).unwrap()
}
