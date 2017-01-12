# curl-rust-easybuilder

A builder pattern for [curl-rust](https://github.com/alexcrichton/curl-rust) crate

[![Crates.io](https://img.shields.io/crates/v/curl-easybuilder.svg)](https://crates.io/crates/curl-easybuilder)
[![Build Status](https://travis-ci.org/lettenj61/curl-rust-easybuilder.svg?branch=master)](https://travis-ci.org/lettenj61/curl-rust-easybuilder)

## Usage
```toml
[dependencies]
curl-easybuilder = "0.1"
```

## Example

```rust
extern crate curl_easybuilder;

use std::io::{stdout, Write};
use curl_easybuilder::EasyBuilder;

fn main() {
    let mut easy = EasyBuilder::new();
    let easy = easy.url("https://www.rust-lang.org/")
                   .write_function(|data| Ok(stdout().write(data).unwrap()))
                   .result()
                   .unwrap();
    easy.perform().unwrap();
}
```

## License
The library licensed under MIT license. See `LICENSE` file for further information.
