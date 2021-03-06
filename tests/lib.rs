extern crate curl;
extern crate curl_easybuilder;

use std::io::{stdout, Write};
use curl_easybuilder::*;

#[test]
fn http_get() {
    let mut easy = EasyBuilder::new();
    let easy = easy.url("https://www.rust-lang.org/")
        .write_function(|data| Ok(stdout().write(data).unwrap()))
        .result()
        .unwrap();
    easy.perform().unwrap();
}

#[test]
fn http_post() {
    let mut easy = EasyBuilder::new();
    let easy = easy.url("https://httpbin.org/post")
        .post(true)
        .post_fields_copy(&b"name=foobar"[..])
        .write_function(|data| Ok(stdout().write(data).unwrap()))
        .result()
        .unwrap();
    easy.perform().unwrap();
}
