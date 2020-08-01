

struct Descriptor {
    /// Address (guest-physical)
    address: u64,
    /// Length
    lenght: u32,
    /// The flags as indicated above
    flags: u16,
    /// We chain unused descriptors via this, too
    next: u16,
}

struct vring_avail {
    uint16_t flags,
    uint16_t idx;
    uint16_t ring[0];
};

/* uint32_t is used here for ids for padding reasons. */
struct vring_used_elem {
    /* Index of start of used descriptor chain. */
    uint32_t id;
    /* Total length of the descriptor chain which was written to. */
    uint32_t len;
};

struct vring_used {
    uint16_t flags;
    uint16_t idx;
    struct vring_used_elem ring[0];
};

struct vring {
    unsigned int num;

    struct vring_desc *desc;
    struct vring_avail *avail;
    struct vring_used *used;
};