//! Simple builder for curl-rust Easy API.
//!
//! The struct helps to create and initialize an `Easy` handle with
//! major curl options, or to set some callback operations.

extern crate curl;
#[macro_use]
extern crate error_chain;

use std::error::Error;
use std::path::Path;
use std::time::Duration;
use curl::easy::{Easy, List};
use curl::easy::{IpResolve, ProxyType, SslVersion, TimeCondition};
use curl::easy::{ReadError, WriteError};

mod errors {
    error_chain! {
        types {
            BuildError, ErrorKind, ResultExt, BuildResult;
        }
        foreign_links {
            Curl(::curl::Error);
        }
    }
}

use errors::*;

pub struct EasyBuilder {
    easy: Easy,
    // FIXME: I'm not sure is it reasonable and effective.
    errors: Vec<curl::Error>,
}

macro_rules! option_setter {
    ( $meth:ident, $an:ident: $mt:ty ) => {
        pub fn $meth(&mut self, $an: $mt) -> &mut EasyBuilder {
            if let Err(e) = self.easy.$meth($an) {
                self.errors.push(e);
            }
            self
        }
    };
}

macro_rules! path_opt {
    ( $meth:ident, $an:ident ) => {
        pub fn $meth<P: AsRef<Path>>(&mut self, $an: P) -> &mut EasyBuilder {
            if let Err(e) = self.easy.$meth($an) {
                self.errors.push(e);
            }
            self
        }
    };
}

impl EasyBuilder {

    pub fn new() -> EasyBuilder {
        EasyBuilder {
            easy: Easy::new(),
            errors: Vec::new(),
        }
    }

    option_setter!(verbose, verbose: bool);
    option_setter!(show_header, show: bool);
    option_setter!(progress, progress: bool);
    option_setter!(signal, signal: bool);
    option_setter!(wildcard_match, m: bool);
    option_setter!(fail_on_error, fail: bool);
    option_setter!(url, url: &str);
    option_setter!(port, port: u16);
    option_setter!(proxy, url: &str);
    option_setter!(proxy_port, port: u16);
    option_setter!(proxy_type, kind: ProxyType);
    option_setter!(noproxy, skip: &str);
    option_setter!(http_proxy_tunnel, tunnel: bool);
    option_setter!(interface, interface: &str);
    option_setter!(set_local_port, port: u16);
    option_setter!(local_port_range, range: u16);
    option_setter!(dns_cache_timeout, dur: Duration);
    option_setter!(buffer_size, size: usize);
    option_setter!(tcp_nodelay, enable: bool);
    option_setter!(address_scope, scope: u32);
    option_setter!(username, user: &str);
    option_setter!(password, pass: &str);
    option_setter!(proxy_username, user: &str);
    option_setter!(proxy_password, pass: &str);
    option_setter!(autoreferer, enable: bool);
    option_setter!(accept_encoding, encoding: &str);
    option_setter!(transfer_encoding, enable: bool);
    option_setter!(follow_location, enable: bool);
    option_setter!(unrestricted_auth, enable: bool);
    option_setter!(max_redirections, max: u32);
    option_setter!(put, enable: bool);
    option_setter!(post, enable: bool);
    option_setter!(post_fields_copy, data: &[u8]);
    option_setter!(post_field_size, size: u64);
    option_setter!(referer, referer: &str);
    option_setter!(useragent, useragent: &str);
    option_setter!(http_headers, list: List);
    option_setter!(cookie, cookie: &str);
    path_opt!(cookie_file, file);
    path_opt!(cookie_jar, file);
    option_setter!(cookie_session, session: bool);
    option_setter!(cookie_list, cookie: &str);
    option_setter!(get, enable: bool);
    option_setter!(ignore_content_length, ignore: bool);
    option_setter!(http_content_decoding, enable: bool);
    option_setter!(http_transfer_decoding, enable: bool);
    option_setter!(range, range: &str);
    option_setter!(resume_from, from: u64);
    option_setter!(custom_request, request: &str);
    option_setter!(fetch_filetime, fetch: bool);
    option_setter!(nobody, enable: bool);
    option_setter!(in_filesize, size: u64);
    option_setter!(upload, enable: bool);
    option_setter!(max_filesize, size: u64);
    option_setter!(time_condition, cond: TimeCondition);
    option_setter!(time_value, val: i64);
    option_setter!(timeout, timeout: Duration);
    option_setter!(low_speed_limit, limit: u32);
    option_setter!(low_speed_time, dur: Duration);
    option_setter!(max_send_speed, speed: u64);
    option_setter!(max_recv_speed, speed: u64);
    option_setter!(max_connects, max: u32);
    option_setter!(fresh_connect, enable: bool);
    option_setter!(forbid_reuse, enable: bool);
    option_setter!(connect_timeout, timeout: Duration);
    option_setter!(ip_resolve, resolve: IpResolve);
    option_setter!(connect_only, enable: bool);
    path_opt!(ssl_cert, cert);
    option_setter!(ssl_cert_type, kind: &str);
    path_opt!(ssl_key, key);
    option_setter!(ssl_key_type, kind: &str);
    option_setter!(key_password, password: &str);
    option_setter!(ssl_engine, engine: &str);
    option_setter!(ssl_engine_default, enable: bool);
    option_setter!(ssl_version, version: SslVersion);
    option_setter!(ssl_verify_host, verify: bool);
    option_setter!(ssl_verify_peer, verify: bool);
    path_opt!(cainfo, path);
    path_opt!(issuer_cert, path);
    path_opt!(capath, path);
    path_opt!(crlfile, path);
    option_setter!(certinfo, enable: bool);
    path_opt!(random_file, p);
    path_opt!(egd_socket, p);
    option_setter!(ssl_cipher_list, ciphers: &str);
    option_setter!(ssl_sessionid_cache, enable: bool);

    pub fn on_write<F>(&mut self, f: F) -> &mut EasyBuilder
        where F: FnMut(&[u8]) -> Result<usize, WriteError> + Send + 'static {
        if let Err(e) = self.easy.write_function(f) {
            self.errors.push(e);
        }
        self
    }

    pub fn on_read<F>(&mut self, f: F) -> &mut EasyBuilder
        where F: FnMut(&mut [u8]) -> Result<usize, ReadError> + Send + 'static {
        if let Err(e) = self.easy.read_function(f) {
            self.errors.push(e);
        }
        self
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn result(&mut self) -> BuildResult<&mut Easy> {
        if !self.has_errors() {
            Ok(&mut self.easy)
        } else {
            let mut s = String::new();
            for e in &self.errors {
                s.push_str(e.description());
                s.push('\n');
            }
            Err(BuildError::from(s))
        }
    }
}

#[cfg(test)]
mod tests {

    use std::io::{self, stdout, Cursor, Read, Write};
    use curl::easy::*;
    use super::*;

    #[test]
    fn it_works() {
    }

    #[test]
    fn build_result() {
        let mut b = EasyBuilder::new();
        assert!(b.result().is_ok());
    }

    #[test]
    fn http_get() {
        let mut easy = EasyBuilder::new();
        let easy = easy.url("https://www.rust-lang.org/")
                       .on_write(|data| {
                           Ok(stdout().write(data).unwrap())
                       })
                       .result()
                       .unwrap();
        easy.perform().unwrap();
    }

    #[test]
    fn http_post() {
        let mut easy = EasyBuilder::new();
        let easy = easy.url("https://httpbin.org/post")
                       .post(true)
                       .on_read(|into| {
                           let mut cursor = Cursor::new(b"foobar"[..].to_vec());
                           Ok(cursor.read(into).unwrap())
                       })
                       .on_write(|data| {
                           Ok(stdout().write(data).unwrap())
                       })
                       .result()
                       .unwrap();
        easy.perform().unwrap();
    }
}
