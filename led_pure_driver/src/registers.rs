use tock_registers::{
    register_structs,
    registers::{ReadOnly, ReadWrite},
};

// bcm2711 datasheet: https://datasheets.raspberrypi.org/bcm2711/bcm2711-peripherals.pdf
// bcm2835 datasheet: https://www.raspberrypi.org/documentation/hardware/raspberrypi/bcm2835/BCM2835-ARM-Peripherals.pdf
register_structs! {
    pub Registers {
        (0x00 => pub gpfsel: [ReadWrite<u32>; 6]),
        (0x18 => gap0: ReadOnly<u32>),
        (0x1c => pub gpset: [ReadWrite<u32>; 2]),
        (0x24 => gap1: ReadOnly<u32>),
        (0x28 => pub gpclr: [ReadWrite<u32>; 2]),
        (0x30 => gap2: ReadOnly<u32>),
        (0x34 => pub gplev: [ReadOnly<u32>; 2]),
        (0x3c => gap3: ReadOnly<u32>),
        (0x40 => pub gpeds: [ReadWrite<u32>; 2]),
        (0x48 => gap4: ReadOnly<u32>),
        (0x4c => pub gpren: [ReadWrite<u32>; 2]),
        (0x54 => gap5: ReadOnly<u32>),
        (0x58 => pub gpfen: [ReadWrite<u32>; 2]),
        (0x60 => gap6: ReadOnly<u32>),
        (0x64 => pub gphen: [ReadWrite<u32>; 2]),
        (0x6c => gap7: ReadOnly<u32>),
        (0x70 => pub gplen: [ReadWrite<u32>; 2]),
        (0x78 => gap8: ReadOnly<u32>),
        (0x7c => pub gparen: [ReadWrite<u32>; 2]),
        (0x84 => gap9: ReadOnly<u32>),
        (0x88 => pub gpafen: [ReadWrite<u32>; 2]),
        (0x90 => gap10: [ReadOnly<u32>; 21]),
        (0xe4 => pub gpio_pup_pdn_cntrl_reg: [ReadWrite<u32>; 4]),
        (0xf4 => @END),
    }
}