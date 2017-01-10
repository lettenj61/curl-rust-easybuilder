extern crate curl;
// #[macro_use] extern crate nickel;

extern crate curl_easybuilder;

use std::io::{stdout, Write};
//use nickel::{Nickel, HttpRouter};

use curl::easy::Easy;
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

#[test]
fn start_transfer() {
    let mut easy = Easy::new();
    let mut tx = TransferBuilder::with_session(&mut easy);
    let tx = tx.write_function(|data| Ok(stdout().write(data).unwrap()))
               .result()
               .unwrap();
    tx.perform().unwrap();
}
