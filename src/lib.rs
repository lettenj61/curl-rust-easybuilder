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

mod easy_builder;
mod tx_builder;

pub use easy_builder::EasyBuilder;
pub use tx_builder::TransferBuilder;
