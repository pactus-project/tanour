use crate::{error::Result, page::Page, Address};
use std::{
    collections::{hash_map::Entry, HashMap},
    io::Write,
    mem::transmute,
};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom},
};

#[derive(Debug, Clone)]
#[repr(C)]
struct Header {
    bom: u16,
    version: u8,
    owner: Address,
    created_at: u32,
    valid_until: u32,
    code_offset: u32,
    code_length: u32,
    data_offset: u32,
    reserved: [u8; 84],
}

const PAGE_SIZE: u32 = 1024 * 1024; // 1 MB
pub struct StorageFile {
    file: File,
    header: Header,
    pages: HashMap<u32, Page>,
}

impl StorageFile {
    pub fn create(
        file_path: &str,
        file_size_in_mb: u32,
        owner: Address,
        created_at: u32,
        valid_until: u32,
        code: &[u8],
    ) -> Result<Self> {
        let file_size = file_size_in_mb * PAGE_SIZE;
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        let zeros = vec![0; file_size as usize];
        file.write_all(&zeros)?;

        let code_offset = std::mem::size_of::<Header>() as u32;
        let code_length = code.len() as u32;
        let data_offset = code_offset + code_length + (128 - (code_length % 128));

        let header = Header {
            bom: 0x7374,
            version: 1,
            owner,
            created_at,
            valid_until,
            code_offset,
            code_length,
            data_offset,
            reserved: [0; 84],
        };

        file.rewind()?;
        unsafe {
            let p_bytes = transmute::<Header, [u8; std::mem::size_of::<Header>()]>(header.clone());
            file.write_all(&p_bytes)?;
        }

        file.seek(SeekFrom::Start(code_offset as u64))?;
        file.write_all(code)?;

        Ok(Self {
            file,
            header,
            pages: HashMap::new(),
        })
    }

    pub fn load(file_path: &str) -> Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .open(file_path)?;

        let mut buffer = [0u8; std::mem::size_of::<Header>()];
        file.read_exact(&mut buffer)?;
        let header =
            unsafe { std::mem::transmute::<[u8; std::mem::size_of::<Header>()], Header>(buffer) };

        Ok(Self {
            file,
            header,
            pages: HashMap::new(),
        })
    }

    pub fn read_code(&mut self) -> Result<Vec<u8>> {
        let mut code = vec![0u8; self.header.code_length as usize];
        self.file
            .seek(SeekFrom::Start(self.header.code_offset as u64))?;
        self.file.read_exact(&mut code)?;
        Ok(code)
    }

    pub fn read_storage(&mut self, offset: u32, length: u32) -> Result<Vec<u8>> {
        log::debug!("read_storage, offset: {offset}, length: {length}");
        let first_page = offset / PAGE_SIZE;
        let last_page = (offset + length) / PAGE_SIZE;
        let mut data = Vec::new();
        let mut read_offset = offset % PAGE_SIZE;
        let mut read_length = 0;

        for page_no in first_page..last_page + 1 {
            let mut len = length - read_length;
            if len > PAGE_SIZE - read_offset {
                len = PAGE_SIZE - read_offset
            }

            let page = self.read_page(page_no)?;
            data.extend_from_slice(&page.data[read_offset as usize..(read_offset + len) as usize]);

            read_offset = 0;
            read_length += len;
        }

        Ok(data)
    }

    pub fn write_storage(&mut self, offset: u32, data: &[u8]) -> Result<()> {
        log::debug!("write_storage, offset: {offset}, length: {}", data.len());
        let length = data.len() as u32;
        let first_page = offset / PAGE_SIZE;
        let last_page = (offset + length) / PAGE_SIZE;
        let mut write_length = 0;
        let page_size = PAGE_SIZE;
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

    fn read_page(&mut self, page_no: u32) -> Result<&mut Page> {
        log::debug!("read_page, page_no: {page_no}");
        let offset = page_no * PAGE_SIZE;

        let page = match self.pages.entry(page_no) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                log::debug!("try to read the storage. offset: {offset}");

                self.file.seek(SeekFrom::Start(offset as u64))?;
                let mut buf = vec![0; PAGE_SIZE as usize];
                self.file.read_exact(&mut buf)?;

                let page = Page::new(offset, PAGE_SIZE, buf);
                v.insert(page)
            }
        };

        Ok(page)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use tempfile::NamedTempFile;

    #[test]
    fn test_header_size() {
        assert_eq!(128, std::mem::size_of::<Header>());
    }

    #[test]
    fn test_read() {
        // TODO: better code
        let owner = [0; 21];
        let created_at = 1;
        let valid_until = 1000;
        let code = vec![1, 2, 3, 4];
        let tmpfile = NamedTempFile::new().unwrap();
        let tmpfile_path = tmpfile.path().to_str().unwrap();
        let mut storage_file =
            StorageFile::create(tmpfile_path, 1, owner, created_at, valid_until, &code).unwrap();

        let data = storage_file.read_storage(3, 12).unwrap();
        assert_eq!(data, vec![0; 12]);
    }

    #[test]
    fn test_write() {
        let owner = [0; 21];
        let created_at = 1;
        let valid_until = 1000;
        let code = vec![1, 2, 3, 4];
        let tmpfile = NamedTempFile::new().unwrap();
        let tmpfile_path = tmpfile.path().to_str().unwrap();
        let mut storage_file =
            StorageFile::create(tmpfile_path, 1, owner, created_at, valid_until, &code).unwrap();

        let data = vec![1, 2, 3];
        storage_file.write_storage(3, &data).unwrap();

        let expected = storage_file.read_storage(3, 3).unwrap();
        assert_eq!(data, expected);
    }

    #[quickcheck]
    fn prop_test_read_write(mut offset: u32, mut length: u32, mut data: Vec<u8>) {
        offset %= PAGE_SIZE;
        length %= PAGE_SIZE;
        data.truncate(length as usize);

        let size_in_mb = ((offset + data.len() as u32) / PAGE_SIZE) + 1;
        let owner = [0; 21];
        let created_at = 1;
        let valid_until = 1000;
        let code = vec![1, 2, 3, 4];
        let tmpfile = NamedTempFile::new().unwrap();
        let tmpfile_path = tmpfile.path().to_str().unwrap();
        let mut storage_file = StorageFile::create(
            tmpfile_path,
            size_in_mb,
            owner,
            created_at,
            valid_until,
            &code,
        )
        .unwrap();

        storage_file.write_storage(offset, &data).unwrap();

        let expected = storage_file
            .read_storage(offset, data.len() as u32)
            .unwrap();
        assert_eq!(data, expected);
    }
}
