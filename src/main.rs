#![no_main]
#![no_std]
#![feature(core_intrinsics, ptr_offset_from, const_raw_ptr_deref, const_ptr_offset_from)]


#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate volatile_register;
extern crate openamp;
#[macro_use]
extern crate memoffset;

use core::fmt::Write;
use core::panic::PanicInfo;
use core::marker::PhantomData;

use cortex_m_rt::entry;
use stm32mp1_pac as target;
use target::interrupt;

mod trace;
mod resource_table;
use resource_table as rt;

mod rpmsg;
use rpmsg::SendMessage;

mod virtio;
mod rpmsg_virtio;

use openamp::vring;

mod ipcc;

mod string;

mod fifo;
use fifo::Fifo;


// Setting up the resource table
/// We have a VDev and a Trace entry; the two VRings are children of the VDev
const NUM_ENTRIES: usize = 2;
#[repr(C)]
#[derive(Debug)]
/// Definition of the resource table structure
struct ResourceTable {
    base: rt::Header,
    offsets: [usize; NUM_ENTRIES],
    rpmsg_vdev: rt::Vdev,
    rpmsg_vring0: rt::VdevVring,
    rpmsg_vring1: rt::VdevVring,
    trace: rt::Trace,
}

#[link_section = ".resource_table"]
#[no_mangle]
static RESOURCE_TABLE: ResourceTable = ResourceTable {
    base: rt::Header {
        ver: 1,
        num: NUM_ENTRIES,
        reserved: [0, 0],
    },

    offsets: [
        offset_of!(ResourceTable, rpmsg_vdev),
        offset_of!(ResourceTable, trace)
    ],

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
    /// Set the address to an invalid value, will be rewritten by Linux
    rpmsg_vring0: rt::VdevVring {
        da: 0xffffffff,
        align: 16,
        num: 16,
        notifyid: 0,
        reserved: 0,
    },

    /// vring1 is for Linux-to-rproc comms
    /// Set the address to an invalid value, will be rewritten by Linux
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
        da: 0x1002C000,
        len: 0x4000,
        reserved: 0,
        name: rt::String32 {
            buffer: *b"trace_cm4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
        },
    },
};


#[entry]
fn main() -> ! {
    // Retrieve the trace singleton
    let mut t = unsafe { trace::steal_trace() };

    // Retrieve the peripherals
    let mut core_peripherals = unsafe { target::CorePeripherals::steal() };
    let peripherals = unsafe { target::Peripherals::steal() };

    // Enabling clocks for the HSEM and IPCC
    peripherals.RCC
        .rcc_mc_ahb3ensetr
        .write(|w|
            w
                .hsemen().set_bit()
                .ipccen().set_bit()
        );
    // Enable clock for the GPIO
    peripherals.RCC
        .rcc_mc_ahb4ensetr
        .write(|w| w.gpioden().set_bit());

    // Enable the LED7 on the devboard
    let led_bank = peripherals.GPIOH;
    let led_pin = 7;
    let offset = 2 * led_pin;
    led_bank
        .gpiox_pupdr
        .modify(|r, w| unsafe { w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)) } );
    led_bank
        .gpiox_otyper
        .modify(|r, w| unsafe { w.bits(r.bits() & !(0b1 << led_pin)) } );
    led_bank
        .gpiox_moder
        .modify(|r, w| unsafe { w.bits((r.bits() & !(0b11 << offset)) | (0b01 << offset)) } );
    led_bank
        .gpiox_bsrr
        .write(|w| unsafe { w.bits(0 << led_pin) } );

    // Turn on the LED7
    led_bank
        .gpiox_bsrr
        .write(|w| unsafe { w.bits(1 << led_pin) } );

    // Spin until status is OK
    const BUFS_PRIMED: u8 = vring::VIRTIO_CONFIG_S_ACKNOWLEDGE
        | vring::VIRTIO_CONFIG_S_DRIVER
        | vring::VIRTIO_CONFIG_S_DRIVER_OK;
    let status_ptr = &RESOURCE_TABLE.rpmsg_vdev.status as *const u8;
    loop {
        // Volatile read as we're in a loop
        let status = unsafe { core::ptr::read_volatile(status_ptr) };
        writeln!(t, "Buffer status is {}", status).unwrap();
        if status == BUFS_PRIMED {
            break;
        } else {
            for _ in 0..10_000 {
                cortex_m::asm::nop();
            }
        }
    }

    // Configure vrings
    // This vring is full of available buffers we can use to send data back to the host.
    let ipu_to_host = unsafe {
        vring::GuestVring::new(
            core::ptr::read_volatile(&RESOURCE_TABLE.rpmsg_vring0.da),
            RESOURCE_TABLE.rpmsg_vring0.num,
            RESOURCE_TABLE.rpmsg_vring0.align,
        )
    };

    // This vring containers buffers the host wishes us to look at and do something with.
    let host_to_ipu = unsafe {
        vring::GuestVring::new(
            core::ptr::read_volatile(&RESOURCE_TABLE.rpmsg_vring1.da),
            RESOURCE_TABLE.rpmsg_vring1.num,
            RESOURCE_TABLE.rpmsg_vring1.align,
        )
    };


    // Prepare the rpmsg transport interface
    let mut transport = rpmsg::Transport::new(ipu_to_host, host_to_ipu);
    register_tty_channel(&mut transport, 0).unwrap();
    register_tty_channel(&mut transport, 1).unwrap();
    register_tty_channel(&mut transport, 2).unwrap();
    register_tty_channel(&mut transport, 3).unwrap();
    register_tty_channel(&mut transport, 4).unwrap();
    register_tty_channel(&mut transport, 5).unwrap();
    writeln!(t, "Transport is now: \n{:#?}", transport).unwrap();

    ipcc::init_ipcc();

    // Enable interrupts for IPCC RX
    core_peripherals.NVIC.enable(target::Interrupt::IPCC_RX1);

    loop {
        let popped = cortex_m::interrupt::free(|_cs| unsafe { MAILBOX_FIFO.pop() });
        if popped.is_some() {
            let _ = transport.receive(| _tx, header, payload| {
                // Skip the framing message
                if header.length == 2 && payload == ['\r' as u8, '\n' as u8] {
                    return
                }
                // Print the received message
                writeln!(t, "Got: {:?}, {}", header, core::str::from_utf8(payload).unwrap()).unwrap();
            });
        } else {
            // Wait for stuff to happen...
            cortex_m::asm::wfe();
        }
    }
}

/// Register an rpmsg protocol
fn register_tty_channel(transport: &mut rpmsg::Transport, channel: u32) -> Result<(), rpmsg::Error>
{
    let msg = rpmsg::NameServiceAnnounce::new(
        "rpmsg-tty-channel",
        channel,
        rpmsg::NameServiceAnnounceFlags::Create,
    );
    let res = transport.send(channel, rpmsg::RPMSG_NS_EPT_ADDR, &msg);

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

    ipcc::ipcc_notify_cpu(ipcc::IpccChannel::Channel1, ipcc::IpccDirection::Tx);
    res
}


static mut MAILBOX_FIFO: Fifo<PhantomData<u8>> = Fifo {
    storage: [PhantomData; fifo::FIFO_SIZE],
    read: 0,
    write: 0,
};

#[interrupt]
fn IPCC_RX1() {
    for channel in [
        ipcc::IpccChannel::Channel1,
        ipcc::IpccChannel::Channel2,
    ].iter() {

        let rx_status = ipcc::ipcc_get_channel_status(*channel, ipcc::IpccDirection::Rx);

        if rx_status == ipcc::IpccStatus::Occupied {
            // Mask the interrupt
            ipcc::ipcc_mask_channel(*channel, ipcc::IpccDirection::Rx);

            // Retrieve the data here
            unsafe { MAILBOX_FIFO.push(PhantomData) };

            // Notify the CPU that the channel is free
            ipcc::ipcc_notify_cpu(*channel, ipcc::IpccDirection::Rx);
           // ipcc_set_channel_status(channel, IpccDirection::Rx, IpccStatus::Free);
            ipcc::ipcc_unmask_channel(*channel, ipcc::IpccDirection::Rx);
        }
    }
}

// Setting up the panic handler
#[panic_handler]
#[inline(never)]
pub fn panic(info: &PanicInfo) -> ! {
    let mut t = unsafe { trace::steal_trace() };
    writeln!(t, "*** SYSTEM PANIC!: {:?}", info).unwrap();
    loop {}
}

// Setting up the reset vector
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