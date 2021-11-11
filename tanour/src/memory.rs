/// defines a pointer to the allocated space in Wasm's linear memory.
pub struct Pointer {
    /// The pointer to the allocated memory
    ptr: *const u8,
    /// The length of allocated memory
    len: u32,
}

impl Pointer {
    /// defiles a pointer from u64
    pub fn from_u64(ptr_64: u64) -> Self {
        let ptr = (ptr_64 & 0xFFFFFFFF) as *const u8;
        let len = (ptr_64 >> 32) as u32;

        Self { ptr, len }
    }

    pub fn offset(&self) -> u32 {
        self.ptr as u32
    }

    pub fn length(&self) -> u32 {
        self.len
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u64() {
        let ptr_64: u64 = 0x0123456789abcdef;
        let ptr = Pointer::from_u64(ptr_64);

        assert_eq!(ptr.length(), 0x01234567);
        assert_eq!(ptr.offset(), 0x89abcdef);
    }
}
