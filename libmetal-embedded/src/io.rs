#![allow(dead_code)]

use core::{mem, ptr};
use core::sync::atomic::{Ordering, AtomicPtr, compiler_fence};


enum IoError {
    InvalidVirt,
    InvalidPhys,
    InvalidPageShift,
    BadOffset,
    BadAddress,
}

/// Libmetal I/O region structure
struct IoRegion {
    /// base virtual address
    virt: *const u8,
    /// table of base physical address of each of the pages in the I/O region
    physmap: &'static[*const u8],
    /// size of the I/O region
    pub size: usize,
    /// page shift of I/O region
    page_shift: usize,
    /// page mask of I/O region
    pub page_mask: usize,
    /// memory attribute of the I/O region
    pub mem_flags: usize,
}

impl IoRegion {
    pub fn new(virt: *const u8,
               physmap: &'static[*const u8],
               size: usize,
               page_shift: usize,
               mem_flags: usize) -> Result<IoRegion, IoError> {
        if virt.is_null() {
            return Err(IoError::InvalidVirt)
        }
        for page in physmap {
            if page.is_null() {
                return Err(IoError::InvalidPhys)
            }
        }
        if page_shift > mem::size_of::<usize>() * 8 {
            return Err(IoError::InvalidPageShift)
        }
        // TODO: implement `metal_sys_io_mem_map` -> allow end user to map physical memory by hand
        Ok(IoRegion {
            virt,
            physmap,
            size,
            page_shift,
            page_mask: (1 << page_shift) - 1,
            mem_flags,
        })
    }

    fn get_virt_address(&self, offset: usize) -> Result<*mut u8, IoError> {
        if offset < self.size {
            return Err(IoError::BadOffset)
        }
        Ok(((self.virt as usize) + offset) as *mut u8)
    }

    fn get_offset_from_address(&self, address: *const u8) -> Result<usize, IoError> {
        if address < self.virt || address > (self.virt as usize).wrapping_add(self.size) as *const u8 {
            return Err(IoError::BadAddress)
        }
        let offset = address.wrapping_sub(self.virt as usize) as usize;
        if offset >= self.size {
            return Err(IoError::BadOffset)
        }
        Ok(offset)
    }

    fn get_physical_address(&self, offset: usize) -> Result<*mut u8, IoError> {
        let page = offset >> self.page_shift;
        let base_address = self.physmap[page];
        Ok((base_address as usize + (offset & self.page_mask)) as *mut u8)
    }

    fn get_page_shift(&self) -> usize {
        self.page_shift
    }

    fn set_page_shift(&mut self, page_shift: usize) -> Result<(), IoError> {
        if page_shift > mem::size_of::<usize>() * 8 {
            return Err(IoError::InvalidPageShift)
        }
        self.page_shift = page_shift;
        Ok(())
    }

    pub fn read<T>(&self, offset: usize, order: Ordering) -> *mut T {
        let atomic_ptr = AtomicPtr::new(((self.virt as usize) + offset) as *mut T);
        atomic_ptr.load(order)
    }

    pub fn read_relaxed<T>(&self, offset: usize) -> *mut T {
        self.read::<T>(offset, Ordering::Relaxed)
    }

    pub fn write<T>(&self, data: &mut T, offset: usize, order: Ordering) {
        let atomic_ptr = AtomicPtr::new(((self.virt as usize) + offset) as *mut T);
        atomic_ptr.store(data as *mut T, order)
    }

    pub fn write_relaxed<T>(&self, data: &mut T, offset: usize) {
        self.write::<T>(data, offset, Ordering::Relaxed)
    }

    // TODO: "memset"-like intrinsic is not exposed yet, update when it is
    pub fn fill(&self, value: u8, offset: usize, length: usize, order: Ordering) -> Result<(), IoError> {
        if offset > self.size {
            return Err(IoError::BadOffset);
        }
        let mut ptr = ((self.virt as usize) + offset) as *mut u8;
        for i in 0..length {
            unsafe {
                ptr = ptr.offset(i as isize);
                ptr::write_volatile(ptr, value);
            }
        }
        compiler_fence(order);
        Ok(())
    }

    pub fn fill_sesq_cst(&self, value: u8, offset: usize, length: usize) -> Result<(), IoError> {
        self.fill(value, offset, length, Ordering::SeqCst)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let io_region = IoRegion::new(
            0x1000000 as *const u8,
            [
                0x4000000 as *const u8,
                0x6000000 as *const u8,
            ].as_ref(),
            0x1000,
            0x100,
            0
        )?;
        assert_eq!(io_region.virt, 0x1000000 as *const u8);
    }
}