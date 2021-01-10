# Windy-macros

A macros for [Windy](https://github.com/takubokudori/windy).

# Features

Converts UTF-8 `&str` to:

- `WString` using `wstring!` or `wstring_lossy!`.
- `AString` using `astring!` or `astring_lossy!`.
- `&WStr` using `wstr!` or `wstr_lossy!`.
- `&AStr` using `astr!` or `astr_lossy!`.
- `[u8]` using `aarr!` or `aarr_lossy!`.
- `[u16]` using `warr!` or `warr_lossy!`.

at compile time.

# Installation

Add the following lines to your Cargo.toml:

```toml
[dependencies]
windy = "0.1.1"
windy-macros = "0.1.0"
```

All windy versions are compatible.

# Example

```rust
use windy::WString;
use windy_macros::wstring;

fn main() {
    let x: WString = wstring!("test");
}
```

# License

This software is released under the MIT or Apache-2.0 License, see LICENSE-MIT or LICENSE-APACHE.
