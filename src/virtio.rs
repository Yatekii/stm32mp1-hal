use openamp::vring;

#[derive(Debug)]
#[repr(C)]
pub struct DeviceId {
    device: u32,
    vendor: u32,
}

pub type DevResetCb = fn(&mut Device);

#[repr(C)]
pub struct Dispatch {
    get_status: fn(&Device),
    set_status: fn(&mut Device, u32),
    get_features: fn(&Device),
    set_features: fn(&mut Device, u32),
    negotiate_features: fn(&Device, u32),

    /*
     * Read/write a variable amount from the device specific (ie, network)
     * configuration region. This region is encoded in the same endian as
     * the guest.
     */
    read_config: fn(&Device, u32, u32, u32),
    write_config: fn(&Device, u32, u32, u32),
    reset_device: fn(&Device),
    notify: fn(Queue),
}

#[repr(C)]
pub struct Device {
    /// Unique position on the virtio bus
    index: u32,
    /// the device type identification (used to match it with a driver)
    id: DeviceId,
    /// the features supported by both ends
    features: u64,
    /// if it is virtio backend or front end
    role: u32,
    /// user registered device callback
    reset_cb: DevResetCb,
    /// Virtio dispatch table
    func: Dispatch,
    /// TODO: remove pointer to virtio_device private data
    private: u32,
    /// number of vrings
    vrings_num: u32,
//    /// vring info
//    vrings_info: vring::Info,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct Buffer {
    data: [u8; 16],
}

// #[derive(Debug)]
#[repr(C)]
pub struct QueueDescExtra {
    cookie: u32,
    ndescs: u16,
}


// #[derive(Debug)]
#[repr(C)]
pub struct Queue {
    vq_dev: &'static Device,
    vq_name: &'static str,
    vq_queue_index: u16,
    vq_nentries: u16,
    vq_flags: u32,
    callback: fn(&Queue),
    notify: fn(&Queue),
    vq_ring: &'static vring::Vring,
    vq_free_cnt: u16,
    vq_queued_cnt: u16,
    shm_io: u32,

    /*
     * Head of the free chain in the descriptor table. If
     * there are no free descriptors, this will be set to
     * VQ_RING_DESC_CHAIN_END.
     */
    vq_desc_head_idx: u16,

    /*
     * Last consumed descriptor in the used table,
     * trails vq_ring.used->idx.
     */
    vq_used_cons_idx: u16,

    /*
     * Last consumed descriptor in the available table -
     * used by the consumer side.
     */
    vq_available_idx: u16,

    /*
     * Used by the host side during callback. Cookie
     * holds the address of buffer received from other side.
     * Other fields in this structure are not used currently.
     */

    vg_descx: QueueDescExtra,
}

//impl Device {
//    pub fn new(address: u32, size: usize) -> ShmPool {
//        Device {
//            base: address,
//            available: size,
//            size,
//        }
//    }
//}