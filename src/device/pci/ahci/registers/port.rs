// SPDX-License-Identifier: GPL-3.0-or-later

use {crate::mem::Accessor, bitfield::bitfield};

pub struct Registers {
    ssts: Accessor<PortxSerialAtaStatus>,
}

bitfield! {
    pub struct PortxSerialAtaStatus(u32);
    pub device_detection, _: 3, 0;
}
