#![no_std]

use core::ops::Deref;

use registers::Registers;
use tock_registers::interfaces::{Readable, Writeable};

pub mod registers;

#[cfg(feature = "osl")]
use osl::log_info;

#[cfg(feature = "osl")]
#[macro_use]
extern crate osl;
#[cfg(feature = "osl")]
const __LOG_PREFIX: &[u8] = b"[Led Driver]\0";

pub struct Led<T: Deref<Target = Registers>> {
    regs: T,
    state: bool
}

unsafe impl<T: Deref<Target = Registers>> Send for Led<T> {}
unsafe impl<T: Deref<Target = Registers>> Sync for Led<T> {}

impl<T: Deref<Target = Registers>> Led<T> {
    pub fn new(regs: T) -> Self {
        // gpio init
        // set gpio 16 as output
        regs.gpfsel[1].set(regs.gpfsel[1].get() | (1 << 18));
        // state off
        regs.gpclr[0].set(1 << 16);
        Self {
            regs,
            state: false
        }
    }

    pub fn toggle(&mut self) {
        if self.state {
            self.off();
        } else {
            self.on();
        }
        #[cfg(feature = "osl")]
        log_info!("Hello");
    }

    pub fn off(&mut self) {
        self.state = false;
        self.regs.gpclr[0].set(1 << 16);
        #[cfg(feature = "osl")]
        log_info!("Led state: {}", self.state);
    }

    pub fn on(&mut self) {
        self.state = true;
        self.regs.gpset[0].set(1 << 16);
        #[cfg(feature = "osl")]
        log_info!("Led state: {}", self.state);
    }
}
