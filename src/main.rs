#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate volatile_register;
extern crate vring;

use core::fmt::Write;
use core::panic::PanicInfo;
use resource_table as rt;

use rpmsg::SendMessage;

pub use stm32mp1_pac as target;
pub use target::interrupt;
use cortex_m_rt::{entry, exception};

#[macro_use]
mod resource_table;
mod rpmsg;
mod string;
mod trace;
mod version;

#[repr(C)]
#[derive(Debug)]
pub struct ResourceTable {
    base: rt::Header,
    offsets: [usize; NUM_ENTRIES],
    rpmsg_vdev: rt::Vdev,
    rpmsg_vring0: rt::VdevVring,
    rpmsg_vring1: rt::VdevVring,
    trace: rt::Trace,
}

#[link_section = ".resource_table"]
#[no_mangle]
pub static RESOURCE_TABLE: ResourceTable = ResourceTable {
    base: rt::Header {
        ver: 1,
        num: NUM_ENTRIES,
        reserved: [0, 0],
    },
    // We don't have an offsetof macro so we have to calculate these by hand
    offsets: [SZ_RT_HEADER, SZ_RT_HEADER + 68],

    rpmsg_vdev: rt::Vdev {
        rtype: rt::ResourceType::VDEV,
        id: vring::VIRTIO_ID_RPMSG,
        notifyid: 0,
        dfeatures: 1,
        gfeatures: 0,
        config_len: 0,
        status: 0,
        num_of_vrings: 2,
        reserved: [0, 0],
    },

    /// vring0 is for rproc-to-Linux comms
    rpmsg_vring0: rt::VdevVring {
        da: 0xffffffff,
        align: 16,
        num: 16,
        notifyid: 0,
        reserved: 0,
    },

    /// vring1 is for Linux-to-rproc comms
    rpmsg_vring1: rt::VdevVring {
        da: 0xffffffff,
        align: 16,
        num: 16,
        notifyid: 1,
        reserved: 0,
    },

    trace: rt::Trace {
        rtype: rt::ResourceType::TRACE,
        /// We must ensure that the tracebuffer structure is linked at this
        /// address. We do this by placing it at the start of the ".tracebuffer"
        /// section. Ideally we'd just take the address of our buffer
        /// but that's now allowed in a static variable definition.
//        da: 0x10056800,
        da: 0x1002C000,
        len: 0x4000,
        reserved: 0,
        name: rt::String32 {
            buffer: *b"trace_cm4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        },
    },
};

#[doc(hidden)]
#[link_section = ".vector_table.reset_vector_stm32mp1"]
#[no_mangle]
pub static __RESET_VECTOR_STM32MP1: unsafe extern "C" fn() -> ! = ResetSTM32MP1;

extern "C" {
    fn Reset() -> !;
}

#[allow(non_snake_case)]
pub unsafe extern "C" fn ResetSTM32MP1() -> ! {
    Reset();
}

//struct BufferWriter<'a> {
//    buf: &'a mut [u8],
//    offset: usize,
//}

//struct Fifo<T>
//where
//    T: Copy,
//{
//    storage: [T; 64],
//    write: u8,
//    read: u8,
//}

const NUM_ENTRIES: usize = 2;
const SZ_RT_HEADER: usize = core::mem::size_of::<rt::Header>() + (NUM_ENTRIES * 4);

//static mut MAILBOX_FIFO: Fifo<u32> = Fifo {
//    storage: [0; 64],
//    read: 0,
//    write: 0,
//};

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

fn init_ipcc() {
    let mut core_peripherals = unsafe { target::CorePeripherals::steal() };
    let peripherals = unsafe { target::Peripherals::steal() };

    // Initializing IPCC
    // Enable TX and RX interrupts for both channels
    peripherals.IPCC
        .c2cr
        .write_with_zero(|w|
            w
                .txfie().set_bit()
                .rxoie().set_bit()
        );

    // Set channels free
    // TODO: modify PAC to add other channels
    peripherals.IPCC
        .c2scr
        .write(|w|
            w
                .ch1c().set_bit()
                .ch2c().set_bit()
        );

    // Unmask TX and RX interrupt on both available channels
    // TODO: there should be 6 channels, there is only 2 in the PAC, fix that
    peripherals.IPCC
        .c2mr
        .write(|w|
            w
                .ch1om().clear_bit()
                .ch2om().clear_bit()
                .ch1fm().clear_bit()
                .ch2fm().clear_bit()
        );

    // Enable interrupts for IPCC RX
    core_peripherals.NVIC.enable(target::Interrupt::IPCC_RX1);
}


// TODO: add other channels
enum IpccChannel {
    Channel1,
    Channel2,
}

enum IpccDirection {
    Tx,
    Rx,
}

enum IpccStatus {
    Free,
    Occupied,
}

fn ipcc_get_channel_status(channel: IpccChannel, direction: IpccDirection) -> IpccStatus {
    let peripherals = unsafe { target::Peripherals::steal() };
    let ipcc = &peripherals.IPCC;

    let register = match direction {
        IpccDirection::Tx => &ipcc.c2toc1sr,
        IpccDirection::Rx => &ipcc.c1toc2sr,
    };

    let status = match channel {
        IpccChannel::Channel1 => register.read().ch1f().bit(),
        IpccChannel::Channel2 => register.read().ch2f().bit(),
    };

    match status {
        true => IpccStatus::Occupied,
        false => IpccStatus::Free,
    }
}

fn ipcc_notify_cpu(channel: IpccChannel, direction: IpccDirection) {
    let peripherals = unsafe { target::Peripherals::steal() };
    let ipcc = &peripherals.IPCC;

    ipcc.c2scr.modify(|r, w|
        match direction {
            IpccDirection::Tx => {
                match channel {
                    IpccChannel::Channel1 => w.ch1s().set_bit(),
                    IpccChannel::Channel2 => w.ch2s().set_bit(),
                }
            },
            IpccDirection::Rx => {
                match channel {
                    IpccChannel::Channel1 => w.ch1c().set_bit(),
                    IpccChannel::Channel2 => w.ch2c().set_bit(),
                }
            },
        }
    );
}

#[entry]
fn main() -> ! {
    // Retrieve the trace singleton
    let mut t = unsafe { trace::steal_trace() };

    // Retrieve the peripherals
    let mut core_peripherals = unsafe { target::CorePeripherals::steal() };
    let peripherals = unsafe { target::Peripherals::steal() };


    peripherals.RCC
        .rcc_mc_ahb3ensetr
        .write(|w|
            w
                // Enable clock for the HSEM block
                .hsemen().set_bit()
                // Enable clock for the IPCC
                .ipccen().set_bit()
        );
    // Enable clock for the GPIO
    peripherals.RCC
        .rcc_mc_ahb4ensetr
        .write(|w| w.gpioden().set_bit());

    // TODO: put in other function
    // Setup the LED7 on the devboard
    // TODO: rename `i` to `LED7` or something
    let i = 7;
    let offset = 2 * i;
    peripherals.GPIOH
        .gpiox_pupdr
        .modify(|r, w| unsafe { w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)) } );
    peripherals.GPIOH
        .gpiox_otyper
        .modify(|r, w| unsafe { w.bits(r.bits() & !(0b1 << i)) } );
    peripherals.GPIOH
        .gpiox_moder
        .modify(|r, w| unsafe { w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)) } );
    peripherals.GPIOH
        .gpiox_bsrr
        .write(|w| unsafe { w.bits(0 << i) } );

    writeln!(t, "Setup complete. Booting {:?}", version::version()).unwrap();

    // Turn on the LED7
    peripherals.GPIOH
        .gpiox_bsrr
        .write(|w| unsafe { w.bits(1 << i) } );

    // Configure vrings

    // This vring is full of available buffers we can use to send
    // data back to the host.
    let ipu_to_host = unsafe {
        vring::GuestVring::new(
            0x10040000,
            RESOURCE_TABLE.rpmsg_vring0.num,
            RESOURCE_TABLE.rpmsg_vring0.align,
        )
    };

    // This vring containers buffers the host wishes us to look at and do
    // something with.
    let host_to_ipu = unsafe {
        vring::GuestVring::new(
            0x10041000,
            RESOURCE_TABLE.rpmsg_vring1.num,
            RESOURCE_TABLE.rpmsg_vring1.align,
        )
    };

    // Spin until status is OK
    const BUFS_PRIMED: u8 = vring::VIRTIO_CONFIG_S_ACKNOWLEDGE
        | vring::VIRTIO_CONFIG_S_DRIVER
        | vring::VIRTIO_CONFIG_S_DRIVER_OK;
    let status_ptr = &RESOURCE_TABLE.rpmsg_vdev.status as *const u8;
    loop {
        // Volatile read as we're in a loop
        let status = unsafe { ::core::ptr::read_volatile(status_ptr) };
        writeln!(t, "Buffer status is {}", status).unwrap();
        if status == BUFS_PRIMED {
            break;
        } else {
            for _ in 0..10_000 {
                cortex_m::asm::nop();
            }
        }
    }

    // Prepare the rpmsg transport interface
    let mut transport = rpmsg::Transport::new(ipu_to_host, host_to_ipu);
    let res = register_proto(&mut transport);
    writeln!(t, "Registered proto {:?}", res).unwrap();
    writeln!(t, "Transport is now: \n{:#?}", transport).unwrap();

    init_ipcc();

    loop {}

//        let mut loops: u32 = 0;
//        loop {
//            loops = loops.wrapping_add(1);
//            let popped = cortex_m::interrupt::free(|_cs| unsafe { MAILBOX_FIFO.pop() });
//            if let Some(msg) = popped {
//                if (loops % 1000) == 0 {
//                    writeln!(t, "Rx message {}", loops).unwrap();
//                }
//                match msg {
//                    rpmsg::MBOX_READY => {
//                        writeln!(t, "{}: Ready received.", loops).unwrap();
//                    }
//                    rpmsg::MBOX_ECHO_REQUEST => {
//                        writeln!(t, "{}: Echo request received, sending reply.", loops).unwrap();
//                        chip.send_message(rpmsg::MBOX_ECHO_REPLY, TX_MAILBOX);
//                    }
//                    1 => {
//                        let res_rx = transport.receive(|mut tx, _header, _payload| {
//                            // writeln!(t, "Got: {:?}, {:x?}", _header, _payload).unwrap();
//                            let mut buffer = [0u8; 64];
//                            {
//                                let mut writer = BufferWriter::new(&mut buffer);
//                                write!(writer, "Response to message {}", loops).unwrap();
//                            }
//                            tx.send(REMOTE_ID, HOST_ID, &buffer)
//                                .expect("Failed to send");
//                            chip.send_message(0, TX_MAILBOX);
//                        });
//                        match res_rx {
//                            Ok(()) => {
//                                // writeln!(t, "{}: Message processed", loops).unwrap();
//                            }
//                            Err(rpmsg::Error::Empty) => {
//                                writeln!(t, "{}: Queue empty??", loops).unwrap();
//                            }
//                            Err(e) => {
//                                writeln!(t, "{}: Transport error: {:?}", loops, e).unwrap();
//                            }
//                        }
//                    }
//                    0 => {
//                        // Ignore - letting us know about space on the to-host ring
//                        // writeln!(t, "{}: Ignoring space indication.", loops).unwrap();
//                    }
//                    m => {
//                        writeln!(t, "{}: Unexpected message ID 0x{:08x}.", loops, m).unwrap();
//                    }
//                }
//            } else {
//                // Wait for stuff to happen...
//                cortex_m::asm::wfe();
//            }
//        }
}

/// Register an rpmsg protocol
fn register_proto(transport: &mut rpmsg::Transport) -> Result<(), rpmsg::Error>
{
    let msg = rpmsg::NameServiceAnnounce::new(
        "rpmsg-tty-channel",
        "rpmsg-tty-channel",
        0x10040000,
        rpmsg::NameServiceAnnounceFlags::Create,
    );
    let res = transport.send(1, 0, &msg);

    let peripherals = unsafe { target::Peripherals::steal() };
    peripherals.IPCC
        .c2cr
        .write(|w| w.txfie().set_bit());

    peripherals.IPCC
        .c2mr
        .write(|w| w.ch1fm().set_bit());

    peripherals.IPCC
        .c2scr
        .write(|w| w.ch1s().set_bit());

    // TODO: the PAC is not generating proper field for this register
//    peripherals.IPCC
//        .c2toc1sr
//        .write(|w| unsafe { w.bits(0) } );


//    IPCC_C1CR.TXFIE
//    IPCC_C1MR.CHnFM
//    IPCC_C1SCR.CHnS
//    IPCC_C1TOC2SR.CHnF

    res
}

//fn send_IPCC_msg()
//{
//    unsafe {
//        &(*target::IPCC::ptr())
//            .
//            .write(|w| w.bits(1 << 8))
//    };
//}

//// Convert the addresses in the vring to addresses we can actually read
//fn address_map(physical_address: u64) -> u64 {
//    RESOURCE_TABLE.pa_to_da(physical_address as usize).unwrap() as u64
//}

// // define the hard fault handler
// cortex_m_rt::exception!(HardFault, hard_fault);

// /// Our default hard fault handler
// #[cortex_m_rt::exception]
// fn hard_fault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
//     panic!("HardFault at {:#?}", ef);
// }

// // define the default exception handler
// cortex_m_rt::exception!(*, default_handler);

///// Our default interrupt handler
//fn default_handler(irqn: i16) {
//    panic!("Unhandled exception (IRQn = {})", irqn);
//}

#[interrupt]
fn IPCC_RX1() {
    let t = trace::get_trace().unwrap();
    writeln!(t, "TEST !!!").unwrap();
}

//    unsafe {
//        // We have to do the read in the interrupt otherwise we'll bounce straight back in to this ISR
//        let mailbox =
//            stm32mp1::get_mailbox(RX_MAILBOX.id, &RESOURCE_TABLE).expect("Bad resource_table in IRQ");
//        if let Some(id) = mailbox.get_message(RX_MAILBOX.slot) {
//            MAILBOX_FIFO.push(id);
//            // Clear the interrupt flag
//            mailbox.clear_interrupts(RX_MAILBOX.user);
//            // Set event to wake from wfe, in case this occurs just after the FIFO
//            // check but just before we enter wfe.
//            cortex_m::asm::sev();
//        }
//    };


#[panic_handler]
#[inline(never)]
pub fn panic(info: &PanicInfo) -> ! {
    let mut t = unsafe { trace::steal_trace() };
    let _ = writeln!(t, "*** SYSTEM PANIC!: {:?}", info);
    loop {}
}

//impl<'a> BufferWriter<'a> {
//    fn new(buf: &'a mut [u8]) -> Self {
//        BufferWriter {
//            buf: buf,
//            offset: 0,
//        }
//    }
//}
//
///// From https://stackoverflow.com/questions/39488327/how-to-write-an-integer-as-a-string-to-a-byte-array-with-no-std
//impl<'a> ::core::fmt::Write for BufferWriter<'a> {
//    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
//        let bytes = s.as_bytes();
//        let buffer_len = self.buf.len();
//        let space = &mut self.buf[self.offset..];
//        let to_fill = &mut space[..bytes.len()];
//        to_fill.copy_from_slice(bytes);
//
//        self.offset += bytes.len().min(buffer_len);
//
//        Ok(())
//    }
//}

//impl<T> Fifo<T>
//where
//    T: Copy,
//{
//    pub fn push(&mut self, data: T) {
//        let write_idx = self.write as usize % self.storage.len();
//        self.storage[write_idx] = data;
//        self.write = self.write.wrapping_add(1);
//    }
//
//    pub fn pop(&mut self) -> Option<T> {
//        if self.read == self.write {
//            None
//        } else {
//            let read_idx = self.read as usize % self.storage.len();
//            let data = self.storage[read_idx];
//            self.read = self.read.wrapping_add(1);
//            Some(data)
//        }
//    }
//}