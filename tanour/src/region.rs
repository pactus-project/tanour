use std::ptr;

use crate::error::{Error, Result};

/// Refers to allocated heap data in Wasm.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Region {
    pub offset: u32,
    /// The number of bytes available in this region
    pub capacity: u32,
    /// The number of bytes used in this region
    pub length: u32,
}

impl Region {
    pub unsafe fn from_ptr(ptr: i32) -> Result<Self> {
        let r = unsafe {
            match ptr::NonNull::new(ptr as *mut Region) {
                None => Err(Error::InvalidRegion {
                    msg: format!("Invalid pointer: {}", ptr),
                }),
                Some(r) => Ok(r.as_ref()),
            }
        }?;

        r.validate()?;
        Ok(*r)
    }

    /// Performs plausibility checks in the given Region. Regions are always created by the
    /// contract and this can be used to detect problems in the standard library of the contract.
    pub fn validate(&self) -> Result<()> {
        if self.offset == 0 {
            return Err(Error::InvalidRegion {
                msg: format!("offset is zero: {:?}", self),
            });
        }
        if self.length > self.capacity {
            return Err(Error::InvalidRegion {
                msg: format!("Length exceeds capacity: {:?}", self),
            });
        }
        if self.offset + self.capacity > (u32::MAX - self.offset) {
            return Err(Error::InvalidRegion {
                msg: format!("Out of range: {:?}", self),
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_region() {
        // empty
        let region = Region {
            offset: 23,
            capacity: 500,
            length: 0,
        };
        region.validate().unwrap();

        // half full
        let region = Region {
            offset: 23,
            capacity: 500,
            length: 250,
        };
        region.validate().unwrap();

        // full
        let region = Region {
            offset: 23,
            capacity: 500,
            length: 500,
        };
        region.validate().unwrap();

        // at end of linear memory (1)
        let region = Region {
            offset: u32::MAX,
            capacity: 0,
            length: 0,
        };
        region.validate().unwrap();

        // at end of linear memory (2)
        let region = Region {
            offset: 1,
            capacity: u32::MAX - 1,
            length: 0,
        };
        region.validate().unwrap();
    }

    #[test]
    fn invalidate_region() {
        let region = Region {
            offset: 0,
            capacity: 500,
            length: 250,
        };
        assert!(region.validate().is_err());

        let region = Region {
            offset: 23,
            capacity: 500,
            length: 501,
        };
        assert!(region.validate().is_err());

        let region = Region {
            offset: 23,
            capacity: u32::MAX,
            length: 501,
        };
        assert!(region.validate().is_err());

        let region = Region {
            offset: u32::MAX,
            capacity: 1,
            length: 0,
        };
        assert!(region.validate().is_err());
    }
}
