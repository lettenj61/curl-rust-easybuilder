extern crate curl;

use curl::easy::{Easy, List};

type BuildResult<'a> = Result<&'a mut Easy, curl::Error>;

pub struct EasyBuilder {
    easy: Easy,
    errors: Vec<curl::Error>,
}

macro_rules! modifier {
    ( $meth:ident, $mt:ty ) => {
        pub fn $meth(&mut self, opt: $mt) -> &mut EasyBuilder {
            if let Err(e) = self.easy.$meth(opt) {
                self.errors.push(e);
            }
            self
        }
    };
}

impl EasyBuilder {

    pub fn init() -> EasyBuilder {
        EasyBuilder {
            easy: Easy::new(),
            errors: Vec::new(),
        }
    }

    modifier!(verbose, bool);
    modifier!(progress, bool);
    modifier!(signal, bool);
    modifier!(wildcard_match, bool);
    modifier!(fail_on_error, bool);
    modifier!(url, &str);
    modifier!(port, u16);
    modifier!(proxy, &str);
    modifier!(proxy_port, u16);
    //modifier!(proxy_type, ProxyType);
    modifier!(noproxy, &str);
    modifier!(http_proxy_tunnel, bool);

    pub fn show_header(&mut self, show: bool) -> &mut EasyBuilder {
        if let Err(e) = self.easy.show_header(show) {
            self.errors.push(e);
        }
        self
    }

    pub fn has_errors(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn result(&mut self) -> BuildResult {
        Ok(&mut self.easy)
    }
}

#[cfg(test)]
mod tests {

    use curl::easy::*;
    use super::*;

    #[test]
    fn it_works() {
    }

    #[test]
    fn build_result() {
        let mut b = EasyBuilder::init();
        assert!(b.result().is_ok());
    }
}
