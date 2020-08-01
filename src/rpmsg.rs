#![allow(dead_code)]


pub const RPMSG_NS_EPT_ADDR: u32 = 0x35;

pub use super::string::String32;
use openamp::vring;
//use alloc::collections::LinkedList;

type EndpointCb= fn(&Endpoint, *const u8, usize, u32, *const u8) -> u32;
type NameServiceUnbindCb = fn(&Endpoint);
type NameServiceBindCb = fn(Device, &str, u32);

pub struct Transport {
    send_channel: vring::GuestVring,
    receive_channel: vring::GuestVring,
}

/// All RemoteProc messages start with this header.
#[derive(Debug)]
#[repr(C)]
pub struct Header {
    pub source: u32,
    pub destination: u32,
    _reserved: u32,
    pub length: u16,
    pub flags: u16,
}

/// Dynamic name service announcement message.
///
/// This message is sent across to publish a new service, or announce about its
/// removal. When the kernel receives these messages, an appropriate rpmsg
/// channel (i.e device) is created/destroyed.
///
/// Must match both Linux kernel definition, and `NAME_SERVICE_ANNOUNCE_LEN`.
#[derive(Debug)]
#[repr(C)]
pub struct NameServiceAnnounce {
    /// name of remote service that is published
    name: String32,
    /// address of remote service that is published
    address: u32,
    /// indicates whether service is created or destroyed
    flags: NameServiceAnnounceFlags,
}

/// RPMsg device operations
struct DeviceOps {
    /// send RPMsg data
    send_offchannel_raw: fn(&Device, u32, u32, *const u8, usize, i32) -> i32,
}


pub struct Device {
//    endpoints: LinkedList<u32>,
    ns_ept: Endpoint,
    bitmap: [u32; 128],
    ns_bind_cb: NameServiceBindCb,
    ops: DeviceOps,
}

pub struct Endpoint {
    /// name of the service supported
    name: &'static str,
    /// pointer to the rpmsg device
    rdev: &'static Device,
    /// local address of the endpoint
    addr: u32,
    /// address of the default remote endpoint binded.
    dest_addr: u32,
    /// user rx callback, return value of this callback is reserved for future use, for now, only allow RPMSG_SUCCESS as return value.
    cb: EndpointCb,
    /// end point service service unbind callback, called when remote ept is destroyed.
    ns_unbind_cb: NameServiceUnbindCb,
//    /// end point node
//    node: LinkedList<u32>,
    priv_data: *const u8,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum NameServiceAnnounceFlags {
    Create = 0,
    Destroy = 1,
}

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Empty,
    Vring(vring::Error),
}

impl From<vring::Error> for Error {
    fn from(e: vring::Error) -> Error {
        match e {
            vring::Error::NoData => Error::Empty,
            _ => Error::Vring(e),
        }
    }
}

pub const MBOX_READY: u32 = 0xFFFFFF00;
pub const MBOX_PENDING_MSG: u32 = 0xFFFFFF01;
pub const MBOX_CRASH: u32 = 0xFFFFFF02;
pub const MBOX_ECHO_REQUEST: u32 = 0xFFFFFF03;
pub const MBOX_ECHO_REPLY: u32 = 0xFFFFFF04;
pub const MBOX_ABORT_REQUEST: u32 = 0xFFFFFF05;
pub const MBOX_FLUSH_CACHE: u32 = 0xFFFFFF06;
pub const MBOX_BOOTINIT_DONE: u32 = 0xFFFFFF07;
pub const MBOX_HIBERNATION: u32 = 0xFFFFFF10;
pub const MBOX_HIBERNATION_FORCE: u32 = 0xFFFFFF11;
pub const MBOX_HIBERNATION_ACK: u32 = 0xFFFFFF12;
pub const MBOX_HIBERNATION_CANCEL: u32 = 0xFFFFFF13;


impl ::core::fmt::Debug for Transport {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        writeln!(fmt, "Send: {:?}", self.send_channel)?;
        writeln!(fmt, "Receive: {:?}", self.receive_channel)?;
        Ok(())
    }
}

pub struct SubSender<'a>(&'a mut vring::GuestVring);

pub trait SendMessage {
    fn send<P>(&mut self, source: u32, destination: u32, payload: &P) -> Result<(), Error>
    where
        P: Sized;
}

impl SendMessage for Transport {
    fn send<P>(&mut self, source: u32, destination: u32, payload: &P) -> Result<(), Error>
    where
        P: Sized,
    {
        let tx_header = Header::new(source, destination, ::core::mem::size_of::<P>());
        self.send_channel.transmit(&tx_header, payload)?;
        Ok(())
    }
}

impl<'a> SendMessage for SubSender<'a> {
    fn send<P>(&mut self, source: u32, destination: u32, payload: &P) -> Result<(), Error>
    where
        P: Sized,
    {
        let tx_header = Header::new(source, destination, ::core::mem::size_of::<P>());
        self.0.transmit(&tx_header, payload)?;
        Ok(())
    }
}

impl Transport {
    pub fn new(send_channel: vring::GuestVring, receive_channel: vring::GuestVring) -> Transport {
        Transport {
            send_channel,
            receive_channel,
        }
    }

    pub fn receive<F>(&mut self, callback: F) -> Result<(), Error>
    where
        F: FnOnce(SubSender, &Header, &[u8]),
    {
        let tx = &mut self.send_channel;
        self.receive_channel.process(move |rx| {
            let buf = rx.get_buffer();
            let (head, tail) = buf.split_at(::core::mem::size_of::<Header>());
            let rx_header: &Header = unsafe { &*(&head[0] as *const _ as *const Header) };
            callback(
                SubSender(tx),
                rx_header,
                &tail[0..rx_header.length as usize],
            );
        })?;
        Ok(())
    }

    pub fn split(self) -> (vring::GuestVring, vring::GuestVring) {
        (self.send_channel, self.receive_channel)
    }
}

impl NameServiceAnnounce {
    pub fn new(
        name: &str,
        address: u32,
        mode: NameServiceAnnounceFlags,
    ) -> NameServiceAnnounce {
        NameServiceAnnounce {
            name: name.into(),
            address,
            flags: mode,
        }
    }
}

impl Header {
    pub fn new(source: u32, destination: u32, length: usize) -> Header {
        assert!(length < 65536);
        Header {
            source,
            destination,
            _reserved: 0,
            length: length as u16,
            flags: 0,
        }
    }
}