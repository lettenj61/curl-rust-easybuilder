extern crate curl;

use std::error::Error;
use std::io::SeekFrom;
use curl::easy::{Easy, Transfer, InfoType, SeekResult, ReadError, WriteError};

use super::errors::*;

type WriteFn<'d> = FnMut(&[u8]) -> Result<usize, WriteError> + 'd;
type ReadFn<'d> = FnMut(&mut [u8]) -> Result<usize, ReadError> + 'd;
type SeekFn<'d> = FnMut(SeekFrom) -> SeekResult + 'd;
type ProgFn<'d> = FnMut(f64, f64, f64, f64) -> bool + 'd;
type DebugFn<'d> = FnMut(InfoType, &[u8]) + 'd;
type HeaderFn<'d> = FnMut(&[u8]) -> bool + 'd;

pub struct TransferBuilder<'data> {
    write_cb: Option<Box<WriteFn<'data>>>,
    read_cb: Option<Box<ReadFn<'data>>>,
    seek_cb: Option<Box<SeekFn<'data>>>,
    progress_cb: Option<Box<ProgFn<'data>>>,
    debug_cb: Option<Box<DebugFn<'data>>>,
    header_cb: Option<Box<HeaderFn<'data>>>,
}

impl<'data> TransferBuilder<'data> {

    pub fn new() -> TransferBuilder<'data> {
        TransferBuilder {
            write_cb: None,
            read_cb: None,
            seek_cb: None,
            progress_cb: None,
            debug_cb: None,
            header_cb: None,
        }
    }

    pub fn write_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'data>
        where F: FnMut(&[u8]) -> Result<usize, WriteError> + 'data
    {
        self.write_cb = Some(Box::new(f));
        self
    }

    pub fn read_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'data>
        where F: FnMut(&mut [u8]) -> Result<usize, ReadError> + 'data
    {
        self.read_cb = Some(Box::new(f));
        self
    }

    pub fn seek_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'data>
        where F: FnMut(SeekFrom) -> SeekResult + 'data
    {
        self.seek_cb = Some(Box::new(f));
        self
    }

    pub fn progress_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'data>
        where F: FnMut(f64, f64, f64, f64) -> bool + 'data
    {
        self.progress_cb = Some(Box::new(f));
        self
    }

    pub fn debug_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'data>
        where F: FnMut(InfoType, &[u8]) + 'data
    {
        self.debug_cb = Some(Box::new(f));
        self
    }

    pub fn header_function<F>(&'data mut self, f: F) -> &mut TransferBuilder<'data>
        where F: FnMut(&[u8]) -> bool + 'data
    {
        self.header_cb = Some(Box::new(f));
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

        let tx = easy.transfer();

        if self.write_cb.is_some() {
            let res = tx.write_function(self.write_cb.unwrap());
        }

        Ok(easy.transfer())
    }
}
