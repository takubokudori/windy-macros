# Windy-macros

[![crates.io](https://img.shields.io/crates/v/windy-macros.svg)](https://crates.io/crates/windy-macros)
[![docs.rs](https://docs.rs/windy-macros/badge.svg)](https://docs.rs/windy-macros)

Macros for [Windy](https://github.com/takubokudori/windy).

# Features

Converts UTF-8 `&str` to:

- `WString` using `wstring!` or `wstring_lossy!`.
- `AString` using `astring!` or `astring_lossy!`.
- `&WStr` using `wstr!` or `wstr_lossy!`.
- `&AStr` using `astr!` or `astr_lossy!`.
- `[u8]` using `aarr!` or `aarr_lossy!`.
- `[u16]` using `warr!` or `warr_lossy!`.

at compile time.

# Example

```rust
use windy::WString;
use windy::macros::wstring;

fn main() {
    let x: WString = wstring!("test");
}
```

# License

This software is released under the MIT or Apache-2.0 License, see LICENSE-MIT or LICENSE-APACHE.
