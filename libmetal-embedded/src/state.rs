#![allow(dead_code)]

/// System independent runtime state for libmetal
pub struct CommonState {
    /// Bus
    bus: Bus,
    /// Generic statically defined shared memory segments
    generic_shmem: ShMem,
    /// Generic statically defined device
    generic_device: Device,
}

struct ShMem {}

//impl CommonState {
//    pub fn new() -> CommonState {
//        CommonState {
//            Bus::new
//        }
//    }
//}

struct Device {
    /// Device name
    name: &'static str,
    /// Number of I/O regions in device
    num_regions: usize,
    /// Array of I/O regions in device
    /// TODO: fix this: metal_io_region[METAL_MAX_DEVICE_REGIONS]
    regions: usize,
    /// Number of IRQs per device
    irq_num: isize,
    /// IRQ ID
    irq_info: *const usize,
}

//struct BusOps {
//    bus_close: fn (struct metal_bus *bus) -> void,
//    dev_open: fn (struct metal_bus *bus, const char *dev_name, struct metal_device **device) -> int,
//    dev_close: fn (struct metal_bus *bus, struct metal_device *device) -> void,
//    dev_irq_ack: fn (struct metal_bus *bus, struct metal_device *device, int irq) -> void,
//    dev_dma_map: fn (struct metal_bus *bus, struct metal_device *device, uint32_t dir, struct metal_sg *sg_in, int nents_in, struct metal_sg *sg_out) -> int,
//    dev_dma_unmap: fn (struct metal_bus *bus, struct metal_device *device, uint32_t dir, struct metal_sg *sg, int nents) -> void,
//};

struct Bus {
    name: &'static str,
    devices: &'static [Device],
//    node: [Node],
}

impl Bus {
    pub fn new(name: &'static str, devices: &'static [Device]) -> Bus {
        Bus {
            name,
            devices,
        }
    }

    pub fn close(&self) {

    }
}