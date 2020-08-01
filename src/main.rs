#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rt;
extern crate volatile_register;
extern crate openamp;

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

mod string;

mod fifo;
use fifo::Fifo;

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

const NUM_ENTRIES: usize = 2;
const SZ_RT_HEADER: usize = core::mem::size_of::<rt::Header>() + (NUM_ENTRIES * 4);

static mut MAILBOX_FIFO: Fifo<PhantomData<u8>> = Fifo {
    storage: [PhantomData; fifo::FIFO_SIZE],
    read: 0,
    write: 0,
};

// ****************************************************************************
//
// Public Functions
//
// ****************************************************************************

fn init_ipcc() {
    let peripherals = unsafe { target::Peripherals::steal() };

    // Initializing IPCC
    // Enable TX and RX interrupts for both channels
    peripherals.IPCC.c2cr.write_with_zero(|w|
        w
        .txfie().set_bit()
        .rxoie().set_bit()
    );

    // Set channels free
    // TODO: modify PAC to add other channels
    peripherals.IPCC.c2scr.write(|w|
        w
        .ch1c().set_bit()
        .ch2c().set_bit()
    );

    // Unmask TX and RX interrupt on both available channels
    // TODO: there should be 6 channels, there is only 2 in the PAC, fix that
    peripherals.IPCC.c2mr.write(|w|
        w
        .ch1om().clear_bit()
        .ch2om().clear_bit()
        .ch1fm().clear_bit()
        .ch2fm().clear_bit()
    );
}


// TODO: add other channels
#[derive(Debug,Copy,Clone,PartialEq)]
enum IpccChannel {
    Channel1,
    Channel2,
}

#[derive(Copy,Clone,PartialEq)]
enum IpccDirection {
    Tx,
    Rx,
}

#[derive(Copy,Clone,PartialEq)]
enum IpccStatus {
    Free,
    Occupied,
}

fn ipcc_unmask_channel(channel: IpccChannel, direction: IpccDirection) {
    let peripherals = unsafe { target::Peripherals::steal() };
    let mask_reg = &peripherals.IPCC.c2mr;

    match direction {
        IpccDirection::Tx => {
            match channel {
                IpccChannel::Channel1 => mask_reg.write(|w| w.ch1fm().clear_bit()),
                IpccChannel::Channel2 => mask_reg.write(|w| w.ch2fm().clear_bit()),
            }
        },
        IpccDirection::Rx => {
            match channel {
                IpccChannel::Channel1 => mask_reg.write(|w| w.ch1om().clear_bit()),
                IpccChannel::Channel2 => mask_reg.write(|w| w.ch2om().clear_bit()),
            }
        }
    }
}

fn ipcc_mask_channel(channel: IpccChannel, direction: IpccDirection) {
    let peripherals = unsafe { target::Peripherals::steal() };
    let mask_reg = &peripherals.IPCC.c2mr;

    match direction {
        IpccDirection::Tx => {
            match channel {
                IpccChannel::Channel1 => mask_reg.write(|w| w.ch1fm().set_bit()),
                IpccChannel::Channel2 => mask_reg.write(|w| w.ch2fm().set_bit()),
            }
        },
        IpccDirection::Rx => {
            match channel {
                IpccChannel::Channel1 => mask_reg.write(|w| w.ch1om().set_bit()),
                IpccChannel::Channel2 => mask_reg.write(|w| w.ch2om().set_bit()),
            }
        }
    }
}

fn ipcc_get_channel_status(channel: IpccChannel, direction: IpccDirection) -> IpccStatus {
    let peripherals = unsafe { target::Peripherals::steal() };
    let ipcc = &peripherals.IPCC;

    let status = match direction {
        IpccDirection::Tx => match channel {
                IpccChannel::Channel1 => ipcc.c2toc1sr.read().ch1f().bit(),
                IpccChannel::Channel2 => ipcc.c2toc1sr.read().ch2f().bit(),
        },
        IpccDirection::Rx => match channel {
                IpccChannel::Channel1 => ipcc.c1toc2sr.read().ch1f().bit(),
                IpccChannel::Channel2 => ipcc.c1toc2sr.read().ch2f().bit(),
        },
    };

    match status {
        true => IpccStatus::Occupied,
        false => IpccStatus::Free,
    }
}

fn ipcc_notify_cpu(channel: IpccChannel, direction: IpccDirection) {
    let peripherals = unsafe { target::Peripherals::steal() };
    let ipcc = &peripherals.IPCC;

    ipcc.c2scr.modify(|_r, w| match direction {
        IpccDirection::Tx => match channel {
            IpccChannel::Channel1 => w.ch1s().set_bit(),
            IpccChannel::Channel2 => w.ch2s().set_bit(),
        },
        IpccDirection::Rx => match channel {
            IpccChannel::Channel1 => w.ch1c().set_bit(),
            IpccChannel::Channel2 => w.ch2c().set_bit(),
        },
    });
}

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

    // This vring is full of available buffers we can use to send
    // data back to the host.
    let ipu_to_host = unsafe {
        vring::GuestVring::new(
            core::ptr::read_volatile(&RESOURCE_TABLE.rpmsg_vring0.da),
            RESOURCE_TABLE.rpmsg_vring0.num,
            RESOURCE_TABLE.rpmsg_vring0.align,
        )
    };

    // This vring containers buffers the host wishes us to look at and do
    // something with.
    let host_to_ipu = unsafe {
        vring::GuestVring::new(
            core::ptr::read_volatile(&RESOURCE_TABLE.rpmsg_vring1.da),
            RESOURCE_TABLE.rpmsg_vring1.num,
            RESOURCE_TABLE.rpmsg_vring1.align,
        )
    };


    // Prepare the rpmsg transport interface
    let mut transport = rpmsg::Transport::new(ipu_to_host, host_to_ipu);
    let res = register_tty_channel(&mut transport);
    writeln!(t, "Registered proto {:?}", res).unwrap();
    writeln!(t, "Transport is now: \n{:#?}", transport).unwrap();

    init_ipcc();

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
fn register_tty_channel(transport: &mut rpmsg::Transport) -> Result<(), rpmsg::Error>
{
    let msg = rpmsg::NameServiceAnnounce::new(
        "rpmsg-tty-channel",
        0x00,
        rpmsg::NameServiceAnnounceFlags::Create,
    );
    let res = transport.send(0x00, 0x35, &msg);

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

    ipcc_notify_cpu(IpccChannel::Channel1, IpccDirection::Tx);
    res
}

#[interrupt]
fn IPCC_RX1() {
    for channel in [
        IpccChannel::Channel1,
        IpccChannel::Channel2,
    ].iter() {

        let rx_status = ipcc_get_channel_status(*channel, IpccDirection::Rx);

        if rx_status == IpccStatus::Occupied {
            // Mask the interrupt
            ipcc_mask_channel(*channel, IpccDirection::Rx);

            // Retrieve the data here
            unsafe { MAILBOX_FIFO.push(PhantomData) };

            // Notify the CPU that the channel is free
            ipcc_notify_cpu(*channel, IpccDirection::Rx);
           // ipcc_set_channel_status(channel, IpccDirection::Rx, IpccStatus::Free);
            ipcc_unmask_channel(*channel, IpccDirection::Rx);
        }
    }
}


#[panic_handler]
#[inline(never)]
pub fn panic(info: &PanicInfo) -> ! {
    let mut t = unsafe { trace::steal_trace() };
    let _ = writeln!(t, "*** SYSTEM PANIC!: {:?}", info);
    loop {}
}


