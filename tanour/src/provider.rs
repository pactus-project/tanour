use crate::blockchain_api::BlockchainAPI;
use crate::error::Result;
use crate::page::Page;

use std::collections::{hash_map::Entry, HashMap};

#[cfg(test)]
use mockall::{automock, predicate::*};

#[cfg_attr(test, automock)]
pub trait Provider: Send {
    fn read_storage(&mut self, offset: u32, len: u32) -> Result<Vec<u8>>;
    fn write_storage(&mut self, offset: u32, data: &[u8]) -> Result<()>;
}

pub struct ProviderAdaptor {
    api: Box<dyn BlockchainAPI>,
    page_size: u32,
    pages: HashMap<u32, Page>,
}

impl ProviderAdaptor {
    pub fn new(api: Box<dyn BlockchainAPI>) -> Result<Self> {
        Ok(ProviderAdaptor {
            page_size: api.page_size()?,
            pages: HashMap::new(),
            api,
        })
    }
    fn read_page(&mut self, page_no: u32) -> Result<&mut Page> {
        println!("fn: read_page, page_no: {page_no}");
        let offset = page_no * self.page_size;

        let page = match self.pages.entry(page_no) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                println!(
                    "Try to read the storage. offset: {offset}, page_size: {}",
                    self.page_size
                );
                let bytes = self.api.read_page(page_no)?;
                let page = Page::new(offset, self.page_size, bytes);
                v.insert(page)
            }
        };

        Ok(page)
    }
}

impl Provider for ProviderAdaptor {
    fn read_storage(&mut self, offset: u32, length: u32) -> Result<Vec<u8>> {
        println!("fn: read_storage, offset: {offset}, length: {length}");
        let first_page = offset / self.page_size;
        let last_page = (offset + length) / self.page_size;
        let mut data = Vec::new();
        let mut read_offset = offset % self.page_size;
        let mut read_length = 0;

        for page_no in first_page..last_page + 1 {
            let mut len = length - read_length;
            if len > self.page_size - read_offset {
                len = self.page_size - read_offset
            }

            let page = self.read_page(page_no)?;
            data.extend_from_slice(&page.data[read_offset as usize..(read_offset + len) as usize]);

            read_offset = 0;
            read_length += len;
        }

        Ok(data)
    }

    fn write_storage(&mut self, offset: u32, data: &[u8]) -> Result<()> {
        let length = data.len() as u32;
        let first_page = offset / self.page_size;
        let last_page = (offset + length) / self.page_size;
        let mut write_length = 0;
        let page_size = self.page_size;
        let mut page_start_offset = offset % page_size;

        for page_no in first_page..last_page + 1 {
            let page = self.read_page(page_no)?;

            let (_, right) = page.data.split_at_mut(page_start_offset as usize);
            let mut buffer = right;

            let mut len = length - write_length;
            if len > page_size - page_start_offset {
                len = page_size - page_start_offset;
            } else {
                let (left, _) = buffer.split_at_mut(len as usize);
                buffer = left;
            }

            let d = &data[write_length as usize..(write_length + len) as usize];
            buffer.copy_from_slice(d);

            page_start_offset = 0;
            write_length += len;
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "./provider_test.rs"]
mod tests;
