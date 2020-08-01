use stm32mp1_pac as target;

pub fn init_ipcc() {
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


// TODO: add other channels, SVD is missing some fields
#[derive(Debug,Copy,Clone,PartialEq)]
pub enum IpccChannel {
    Channel1,
    Channel2,
}

#[derive(Copy,Clone,PartialEq)]
pub enum IpccDirection {
    Tx,
    Rx,
}

#[derive(Copy,Clone,PartialEq)]
pub enum IpccStatus {
    Free,
    Occupied,
}

pub fn ipcc_unmask_channel(channel: IpccChannel, direction: IpccDirection) {
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

pub fn ipcc_mask_channel(channel: IpccChannel, direction: IpccDirection) {
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

pub fn ipcc_get_channel_status(channel: IpccChannel, direction: IpccDirection) -> IpccStatus {
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

pub fn ipcc_notify_cpu(channel: IpccChannel, direction: IpccDirection) {
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