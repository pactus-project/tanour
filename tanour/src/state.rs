use crate::{error::Result, page::Page, provider_api::ProviderAPI};
#[cfg(test)]
use mockall::{automock, predicate::*};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[cfg_attr(test, automock)]
pub trait StateTrait: Send + Sync {
    fn read_storage(&mut self, offset: usize, len: usize) -> Result<Vec<u8>>;
    fn write_storage(&mut self, offset: usize, data: &[u8]) -> Result<()>;
}

#[derive(Debug)]
pub struct State<P> {
    provider: P,
    page_size: usize,
    pages: HashMap<usize, Page>,
}

impl<P> State<P>
where
    P: ProviderAPI,
{
    pub fn new(provider: P, page_size: usize) -> Self {
        State {
            provider,
            page_size,
            pages: HashMap::new(),
        }
    }

    fn read_page(&mut self, page_no: usize) -> Result<&mut Page> {
        println!("fn: read_page, page_no: {}", page_no);
        let offset = page_no * self.page_size;

        let page = match self.pages.entry(page_no) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                println!(
                    "Try to read the storage. offset: {}, page_size: {}",
                    offset, self.page_size
                );
                let bytes = self.provider.read_storage(offset, self.page_size)?;
                let page = Page::new(offset, self.page_size, bytes);
                v.insert(page)
            }
        };

        Ok(page)
    }
}

impl<P> StateTrait for State<P>
where
    P: ProviderAPI,
{
    fn read_storage(&mut self, offset: usize, length: usize) -> Result<Vec<u8>> {
        println!("fn: read_storage, offset: {}, length: {}", offset, length);
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
            data.extend_from_slice(&page.data[read_offset..read_offset + len]);

            read_offset = 0;
            read_length += len;
        }

        Ok(data)
    }

    fn write_storage(&mut self, offset: usize, data: &[u8]) -> Result<()> {
        let length = data.len();
        let first_page = offset / self.page_size;
        let last_page = (offset + length) / self.page_size;
        let mut write_length = 0;
        let page_size = self.page_size;
        let mut page_start_offset = offset % page_size;

        for page_no in first_page..last_page + 1 {
            let page = self.read_page(page_no)?;

            let (_, right) = page.data.split_at_mut(page_start_offset);
            let mut buffer = right;

            let mut len = length - write_length;
            if len > page_size - page_start_offset {
                len = page_size - page_start_offset;
            } else {
                let (left, _) = buffer.split_at_mut(len);
                buffer = left;
            }

            let d = &data[write_length..write_length + len];
            buffer.copy_from_slice(d);

            page_start_offset = 0;
            write_length += len;
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "./state_test.rs"]
mod tests;
