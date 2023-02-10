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
        let data_offset =
            code_offset + code_length + (PAGE_SIZE - (code_offset + code_length % PAGE_SIZE));

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
        if length == 0 {
            return Ok(Vec::new());
        }
        let first_page = offset / PAGE_SIZE;
        let last_page = (offset + length - 1) / PAGE_SIZE;
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
        if length == 0 {
            return Ok(());
        }
        let first_page = offset / PAGE_SIZE;
        let last_page = (offset + length - 1) / PAGE_SIZE;
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

                self.file
                    .seek(SeekFrom::Start((self.header.data_offset + offset) as u64))?;
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
    use crate::{address_from_hex, random_address};
    use quickcheck_macros::quickcheck;
    use tempfile::NamedTempFile;

    #[test]
    fn test_header_size() {
        assert_eq!(128, std::mem::size_of::<Header>());
    }

    #[test]
    fn test_header() {
        let owner = address_from_hex("0102030405060708090a0b0c0d0e0f101112131415");
        let created_at = 0x12345678;
        let valid_until = 0x87654321;
        let tmpfile = NamedTempFile::new().unwrap();
        let tmpfile_path = tmpfile.path().to_str().unwrap();
        let code = vec![1, 2, 3, 4];
        let mut storage_file =
            StorageFile::create(tmpfile_path, 2, owner, created_at, valid_until, &code).unwrap();

        let mut buffer = [0u8; std::mem::size_of::<Header>()];
        storage_file.file.rewind().unwrap();
        storage_file.file.read_exact(&mut buffer).unwrap();

        assert_eq!(
            buffer,
            [
                0x74, 0x73, // bom
                1,    // version
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                21, // owner
                0x78, 0x56, 0x34, 0x12, // created_at
                0x21, 0x43, 0x65, 0x87, // valid_until
                128, 0, 0, 0, // code_offset
                4, 0, 0, 0, // code_length
                0, 0, 16, 0, // data_offset
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0 // reserved
            ]
        );
    }

    #[test]
    fn test_bad_offset() {
        let owner = random_address();
        let created_at = 1;
        let valid_until = 1000;
        let tmpfile = NamedTempFile::new().unwrap();
        let tmpfile_path = tmpfile.path().to_str().unwrap();
        let code = vec![1, 2, 3, 4];
        let mut storage_file =
            StorageFile::create(tmpfile_path, 2, owner, created_at, valid_until, &code).unwrap();

        storage_file.write_storage(0, &[1]).unwrap();
        assert_eq!(storage_file.read_storage(0, 1).unwrap(), vec![1]);

        storage_file.write_storage(PAGE_SIZE - 1, &[1]).unwrap();
        assert_eq!(
            storage_file.read_storage(PAGE_SIZE - 1, 1).unwrap(),
            vec![1]
        );

        assert!(storage_file.write_storage(PAGE_SIZE, &[1]).is_err());
        assert!(storage_file.read_storage(PAGE_SIZE, 1).is_err());
    }

    #[quickcheck]
    fn test_empty_data() {
        do_prop_test_read_write(0, vec![], vec![]);
    }

    #[quickcheck]
    fn prop_test_read_write(mut offset: u32, mut length: u32, mut data: Vec<u8>, code: Vec<u8>) {
        offset %= PAGE_SIZE;
        length %= PAGE_SIZE;
        data.truncate(length as usize);

        do_prop_test_read_write(offset, data, code);
    }

    fn do_prop_test_read_write(offset: u32, data: Vec<u8>, code: Vec<u8>) {
        let size_in_mb = ((offset + code.len() as u32 + data.len() as u32) / PAGE_SIZE) + 2;
        let owner = random_address();
        let created_at = 1;
        let valid_until = 1000;
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
