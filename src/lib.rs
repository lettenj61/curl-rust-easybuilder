//! Simple builder for curl-rust Easy API.
//!
//! The struct helps to create and initialize an `Easy` handle with
//! major curl options, or to set some callback operations.

extern crate curl;

use std::time::Duration;
use curl::easy::{Easy, List};
use curl::easy::{IpResolve, ProxyType, SslVersion, TimeCondition};
use curl::easy::{WriteError};

type BuildResult<'a> = Result<&'a mut Easy, curl::Error>;

pub struct EasyBuilder {
    easy: Easy,
    // TODO consider use error-chain
    error: Option<curl::Error>,
}

macro_rules! option_setter {
    ( $meth:ident, $an:ident: $mt:ty ) => {
        pub fn $meth(&mut self, $an: $mt) -> &mut EasyBuilder {
            if let Err(e) = self.easy.$meth($an) {
                self.error = Some(e);
            }
            self
        }
    };
}

impl EasyBuilder {

    pub fn new() -> EasyBuilder {
        EasyBuilder {
            easy: Easy::new(),
            error: None,
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
    //option_setter!(cookie_file<P: AsRef<Path>>, file: P);
    //option_setter!(cookie_jar<P: AsRef<Path>>, file: P);
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
    //option_setter!(ssl_cert<P: AsRef<Path>>, cert: P);
    option_setter!(ssl_cert_type, kind: &str);
    //option_setter!(ssl_key<P: AsRef<Path>>, key: P);
    option_setter!(ssl_key_type, kind: &str);
    option_setter!(key_password, password: &str);
    option_setter!(ssl_engine, engine: &str);
    option_setter!(ssl_engine_default, enable: bool);
    option_setter!(ssl_version, version: SslVersion);
    option_setter!(ssl_verify_host, verify: bool);
    option_setter!(ssl_verify_peer, verify: bool);
    //option_setter!(cainfo<P: AsRef<Path>>, path: P);
    //option_setter!(issuer_cert<P: AsRef<Path>>, path: P);
    //option_setter!(capath<P: AsRef<Path>>, path: P);
    //option_setter!(crlfile<P: AsRef<Path>>, path: P);
    option_setter!(certinfo, enable: bool);
    //option_setter!(random_file<P: AsRef<Path>>, p: P);
    //option_setter!(egd_socket<P: AsRef<Path>>, p: P);
    option_setter!(ssl_cipher_list, ciphers: &str);
    option_setter!(ssl_sessionid_cache, enable: bool);

    pub fn on_write<F>(&mut self, f: F) -> &mut EasyBuilder
        where F: FnMut(&[u8]) -> Result<usize, WriteError> + Send + 'static {
        if let Err(e) = self.easy.write_function(f) {
            self.error = Some(e);
        }
        self
    }

    pub fn has_errors(&self) -> bool {
        self.error.is_some()
    }

    pub fn result(&mut self) -> BuildResult {
        match self.error {
            None        => Ok(&mut self.easy),
            Some(ref e) => Err(e.clone()),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::io::{self, stdout, Write};
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
}
