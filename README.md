# Windy-macros

Windy macros

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
use windy_macros::wstring;
use windy::WString;

fn main() {
    let x: WString = wstring!("test");
}
```

# License

This software is released under the MIT or Apache-2.0 License, see LICENSE-MIT or LICENSE-APACHE.
