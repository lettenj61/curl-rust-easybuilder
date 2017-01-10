extern crate curl;

use std::error::Error;
use std::io::SeekFrom;
use curl::easy::{Easy, Transfer, InfoType, SeekResult, ReadError, WriteError};

use super::errors::*;

pub struct TransferBuilder<'easy, 'data> {
    transfer: Transfer<'easy, 'data>,
    errors: Vec<curl::Error>,
}

impl<'easy, 'data> TransferBuilder<'easy, 'data> {

    pub fn new(easy: &'easy mut Easy) -> TransferBuilder<'easy, 'data> {
        TransferBuilder {
            transfer: easy.transfer(),
            errors: Vec::new(),
        }
    }

    pub fn write_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'easy, 'data>
        where F: FnMut(&[u8]) -> Result<usize, WriteError> + 'data
    {
        if let Err(e) = self.transfer.write_function(f) {
            self.errors.push(e);
        }
        self
    }

    pub fn read_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'easy, 'data>
        where F: FnMut(&mut [u8]) -> Result<usize, ReadError> + 'data
    {
        if let Err(e) = self.transfer.read_function(f) {
            self.errors.push(e);
        }
        self
    }

    pub fn seek_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'easy, 'data>
        where F: FnMut(SeekFrom) -> SeekResult + 'data
    {
        if let Err(e) = self.transfer.seek_function(f) {
            self.errors.push(e);
        }
        self
    }

    pub fn progress_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'easy, 'data>
        where F: FnMut(f64, f64, f64, f64) -> bool + 'data
    {
        if let Err(e) = self.transfer.progress_function(f) {
            self.errors.push(e);
        }
        self
    }

    pub fn debug_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'easy, 'data>
        where F: FnMut(InfoType, &[u8]) + 'data
    {
        if let Err(e) = self.transfer.debug_function(f) {
            self.errors.push(e);
        }
        self
    }

    pub fn header_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'easy, 'data>
        where F: FnMut(&[u8]) -> bool + 'data
    {
        if let Err(e) = self.transfer.header_function(f) {
            self.errors.push(e);
        }
        self
    }

    pub fn has_errors(&mut self) -> bool {
        self.errors.is_empty()
    }

    pub fn result(&'data mut self) -> BuildResult<&'data mut Transfer<'easy, 'data>> {
        if !self.has_errors() {
            Ok(&mut self.transfer)
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
