//! # Resource Tables
//!
//! Copyright (c) 2018, Cambridge Consultants Ltd.
//! See the top-level README.md for licence details.
//!
//! This is the code that is generic to all resource tables. Your specific
//! resource table for your application should be defined elsewhere.

// ****************************************************************************
//
// Imports
//
// ****************************************************************************

pub use super::string::String32;

// ****************************************************************************
//
// Sub-modules
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Macros
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Types / Traits
//
// ****************************************************************************

/// The types of entry you can have in a Resource Table.
#[repr(u32)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum ResourceType {
    /// Get the host to allocate you some memory
    CARVEOUT = 0,
    /// Map some MMIO registers in
    DEVMEM = 1,
    /// Point at a buffer where you can write strings
    TRACE = 2,
    /// Map a VirtIO device
    VDEV = 3,
}
/// All resource tables start with this header, followed by
/// the offset array.
#[repr(C)]
#[derive(Debug)]
pub struct Header {
    pub ver: u32,
    pub num: usize,
    pub reserved: [u32; 2],
}

/// This is the structure for `ResourceType::TRACE`.
#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub struct Trace {
    pub rtype: ResourceType,
    pub da: usize,
    pub len: usize,
    pub reserved: u32,
    pub name: String32,
}

/// This is the structure for `ResourceType::VDEV`. It must be followed by the
/// appropriate number of `VdevVring` structures.
#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub struct Vdev {
    pub rtype: ResourceType,
    pub id: u32,
    pub notifyid: u32,
    pub dfeatures: u32,
    pub gfeatures: u32,
    pub config_len: u32,
    pub status: u8,
    pub num_of_vrings: u8,
    pub reserved: [u8; 2],
}

/// The individual vrings follow on from their `Vdev`.
#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub struct VdevVring {
    pub da: usize,
    pub align: usize,
    pub num: usize,
    pub notifyid: u32,
    pub reserved: u32,
}

// ****************************************************************************
//
// Public Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Types / Traits
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Data
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// Private Functions
//
// ****************************************************************************

// None

// ****************************************************************************
//
// End Of File
//
// ****************************************************************************
