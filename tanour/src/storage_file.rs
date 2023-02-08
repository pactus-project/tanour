use crate::{
    error::{Error, Result},
    page::Page,
};
use std::{
    collections::{hash_map::Entry, HashMap},
    io::Write,
};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom},
};

const PAGE_SIZE: u32 = 1024 * 1024; // 1 MB
pub struct StorageFile {
    file: File,
    pages: HashMap<u32, Page>,
}

impl StorageFile {
    pub fn create(file_path: &str, file_size_in_mb: u32) -> Result<Self> {
        let file_size = file_size_in_mb * PAGE_SIZE;
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        let zeros = vec![0; file_size as usize];
        file.write_all(&zeros)?;

        Ok(Self {
            file,
            pages: HashMap::new(),
        })
    }

    pub fn read_storage(&mut self, offset: u32, length: u32) -> Result<Vec<u8>> {
        println!("fn: read_storage, offset: {offset}, length: {length}");
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
        log::debug!("fn: read_page, page_no: {page_no}");
        let offset = page_no * PAGE_SIZE;

        let page = match self.pages.entry(page_no) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                log::debug!(
                    "Try to read the storage. offset: {offset}, page_size: {}",
                    PAGE_SIZE
                );

                self.file.seek(SeekFrom::Start(offset as u64))?;
                let mut buf = vec![0; PAGE_SIZE as usize];
                let size = self.file.read(&mut buf)?;
                if size != buf.len() {
                    return Err(Error::IOError(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "unable to read whole buffer",
                    )));
                }

                let page = Page::new(offset, PAGE_SIZE, buf);
                v.insert(page)
            }
        };

        Ok(page)
    }
}

#[cfg(test)]
mod tests {
    use crate::storage_file::StorageFile;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read() {
        let tmpfile = NamedTempFile::new().unwrap();
        let tmpfile_path = tmpfile.path().to_str().unwrap();
        let mut storage_file = StorageFile::create(tmpfile_path, 1).unwrap();

        let data = storage_file.read_storage(3, 12).unwrap();
        assert_eq!(data, vec![0; 12]);
    }

    #[test]
    fn test_write() {
        let tmpfile = NamedTempFile::new().unwrap();
        let tmpfile_path = tmpfile.path().to_str().unwrap();
        let mut storage_file = StorageFile::create(tmpfile_path, 1).unwrap();

        let data = vec![1, 2, 3];
        storage_file.write_storage(3, &data).unwrap();

        let expected = storage_file.read_storage(3, 3).expect("Reading failed");
        assert_eq!(data, expected);
    }
}
