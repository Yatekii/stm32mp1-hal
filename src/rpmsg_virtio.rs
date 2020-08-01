#![allow(dead_code)]

use crate::virtio;
use crate::rpmsg;

#[derive(Debug)]
#[repr(C)]
pub struct ShmPool {
    /// base address of the memory pool
    base: u32,
    /// available memory size
    available: usize,
    /// total pool size
    size: usize,
}

impl ShmPool {
    pub fn new(address: u32, size: usize) -> ShmPool {
        ShmPool {
            base: address,
            available: size,
            size,
        }
    }
}

#[repr(C)]
pub struct Device {
    rdev: rpmsg::Device,
    vdev: &'static virtio::Device,
    rvq: &'static virtio::Queue,
    svq: &'static virtio::Queue,
    shpool: &'static ShmPool,
}

//impl Device {
//    fn get_role(&self) {
//        self.vdev.role
//    }
//}