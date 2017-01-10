extern crate curl;
// #[macro_use] extern crate nickel;

extern crate curl_easybuilder;

use std::io::{stdout, Write};
//use nickel::{Nickel, HttpRouter};

use curl_easybuilder::*;

#[test]
fn build_result() {
    let mut b = EasyBuilder::new();
    assert!(b.result().is_ok());
}

#[test]
fn http_get() {
    let mut easy = EasyBuilder::new();
    let easy = easy.url("https://www.rust-lang.org/")
        .on_write(|data| Ok(stdout().write(data).unwrap()))
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
        .on_write(|data| Ok(stdout().write(data).unwrap()))
        .result()
        .unwrap();
    easy.perform().unwrap();
}
