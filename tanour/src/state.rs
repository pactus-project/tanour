use crate::{
    error::Result,
    provider_api::ProviderAPI,
    types::{Address, Bytes},
};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Debug)]
struct Page {
    offset: usize,
    length: usize,
    data: Bytes,
    updated: bool,
}
impl Page {
    pub fn new(offset: usize, length: usize, data: Bytes) -> Self {
        Page {
            offset,
            length,
            data,
            updated: false,
        }
    }
}

// TODO: Rename it to the state
#[derive(Debug)]
pub struct State<P> {
    provider: P,
    address: Address,
    page_size: usize,
    pages: HashMap<usize, Page>,
    readonly: bool,
}

impl<P> State<P>
where
    P: ProviderAPI,
{
    pub fn new(provider: P, address: Address, page_size: usize) -> Self {
        State {
            provider,
            address,
            page_size,
            pages: HashMap::new(),
            readonly: true,
        }
    }

    pub fn make_readonly(&mut self, readonly: bool ) {
        self.readonly = readonly;
    }

    fn get_page(&mut self, page_no: usize) -> Result<&mut Page> {
        let offset = page_no * self.page_size;

        let page = match self.pages.entry(page_no) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                let bytes = self
                    .provider
                    .read_storage(&self.address, offset, self.page_size)?;
                let page = Page::new(offset, self.page_size, bytes);
                v.insert(page)
            }
        };

        Ok(page)
    }

    fn read_storage(&mut self, offset: usize, length: usize) -> Result<Bytes> {
        let start = offset / self.page_size;
        let end = offset + length / self.page_size;
        let mut data = Vec::new();
        let mut read_offset = offset % self.page_size;
        let mut read_length = 0;

        for page_no in start..end + 1 {
            let mut len = length - read_length;
            if len > self.page_size - read_offset {
                len = self.page_size - read_offset
            }

            let page = self.get_page(page_no)?;
            data.extend_from_slice(&page.data[read_offset..read_offset+len]);

            read_offset = 0;
            read_length += len;
        }

        Ok(data)
    }

    fn write_storage(&mut self, offset: usize, value: &Bytes) -> Result<()> {
        let length = value.len();
        let start = offset / self.page_size;
        let end = offset + length / self.page_size;
        let mut write_offset = offset % self.page_size;
        let mut write_length = 0;

        for page_no in start..end + 1 {
            let mut len = length - write_length;
            if len > self.page_size - write_offset {
                len = self.page_size - write_offset
            }

            let page = self.get_page(page_no)?;
            let (_, right) = page.data.split_at_mut(write_offset);
            right.copy_from_slice(&value[len..]);

            write_offset = 0;
            write_length += len;
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "./state_test.rs"]
pub mod state_test;
