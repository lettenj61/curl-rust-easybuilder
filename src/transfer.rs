extern crate curl;

use std::error::Error;
use std::io::SeekFrom;
use curl::easy::{Easy, Transfer, InfoType, SeekResult, ReadError, WriteError};

use super::errors::*;

#[derive(Default)]
struct Callbacks<'d> {
    write: Option<Box<FnMut(&[u8]) -> Result<usize, WriteError> + 'd>>,
    read: Option<Box<FnMut(&mut [u8]) -> Result<usize, ReadError> + 'd>>,
    seek: Option<Box<FnMut(SeekFrom) -> SeekResult + 'd>>,
    debug: Option<Box<FnMut(InfoType, &[u8]) + 'd>>,
    header: Option<Box<FnMut(&[u8]) -> bool + 'd>>,
    progress: Option<Box<FnMut(f64, f64, f64, f64) -> bool + 'd>>,
}

pub struct TransferBuilder<'data> {
    callbacks: Callbacks<'data>,
}

impl<'data> TransferBuilder<'data> {

    pub fn new() -> TransferBuilder<'data> {
        TransferBuilder {
            callbacks: Default::default(),
        }
    }

    pub fn write_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'data>
        where F: FnMut(&[u8]) -> Result<usize, WriteError> + 'data
    {
        self.callbacks.write = Some(Box::new(f));
        self
    }

    pub fn read_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'data>
        where F: FnMut(&mut [u8]) -> Result<usize, ReadError> + 'data
    {
        self.callbacks.read = Some(Box::new(f));
        self
    }

    pub fn seek_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'data>
        where F: FnMut(SeekFrom) -> SeekResult + 'data
    {
        self.callbacks.seek = Some(Box::new(f));
        self
    }

    pub fn progress_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'data>
        where F: FnMut(f64, f64, f64, f64) -> bool + 'data
    {
        self.callbacks.progress = Some(Box::new(f));
        self
    }

    pub fn debug_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'data>
        where F: FnMut(InfoType, &[u8]) + 'data
    {
        self.callbacks.debug = Some(Box::new(f));
        self
    }

    pub fn header_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'data>
        where F: FnMut(&[u8]) -> bool + 'data
    {
        self.callbacks.header = Some(Box::new(f));
        self
    }

    /*
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
    */

    pub fn result<'easy>(&mut self, easy: &'easy mut Easy) -> BuildResult<Transfer<'easy, 'data>> {
        let _errors = Vec::<curl::Error>::new();

        let tx: Transfer<'easy, 'data> = easy.transfer();

        if self.callbacks.write.is_some() {
            let cb = self.callbacks.write.unwrap();
            tx.write_function(*cb);
        }

        Ok(tx)
    }
}
