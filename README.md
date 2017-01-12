# curl-rust-easybuilder

A builder pattern for [curl-rust](https://github.com/alexcrichton/curl-rust) crate

[![Build Status](https://travis-ci.org/lettenj61/curl-rust-easybuilder.svg?branch=master)](https://travis-ci.org/lettenj61/curl-rust-easybuilder)

## Add to your project
TODO: publish crate on Crates.io

## Usage

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
