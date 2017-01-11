//! Simple builder for curl-rust Easy API.
//!
//! The struct helps to create and initialize an `Easy` handle with
//! major curl options, or to set some callback operations.

extern crate curl;
#[macro_use]
extern crate error_chain;

pub mod errors {
    error_chain! {
        types {
            BuildError, ErrorKind, ResultExt, BuildResult;
        }
        foreign_links {
            Curl(::curl::Error);
        }
    }
}

mod easy;

pub use easy::EasyBuilder;
