#![no_std]
#![no_main]
#![allow(non_snake_case)]


extern crate panic_halt;
extern crate embedded_hal;
extern crate nrf52840_hal as hal;

use core::marker::PhantomData;

use cortex_m_rt::{entry, exception};
use hal::gpio::{p0, p1};
use hal::target::{Peripherals, twim0};

#[allow(non_snake_case)]
mod pins_redefined;
mod mytwim;

use pins_redefined::Pins;
use mytwim::{i2cPins, Twim};
use embedded_hal::blocking::i2c::Write;
// use cortex_m_semihosting::hprintln;

pub use twim0::frequency::FREQUENCY_A as Frequency;


pub const ADDRESS: u8 = 0x60; 


#[derive(Copy, Clone, Debug)]
pub struct ATECC608A<I2C> {
    pub i2c: PhantomData<I2C>,
    pub dev_addr: u8,
}

impl <I2C, Error> ATECC608A <I2C> where 
    I2C : Write<Error = Error>, 
    {
    pub fn new(_i2c: &I2C) -> Result<Self, Error> {
        let atecc608a = ATECC608A {
            i2c: PhantomData,  
            dev_addr: ADDRESS
        };
        Ok(atecc608a)
    }

    pub fn wake(&self, i2c: &mut I2C) -> Result<(), Error> {
        let bytes = [0; 1];
        i2c.write(self.dev_addr, &bytes)
    }
}
    
#[entry]
fn main () -> ! {
    let p = Peripherals::take().unwrap();
    let pins = Pins::new(p0::Parts::new(p.P0), p1::Parts::new(p.P1));
    
    let scl = pins.p27.into_floating_input().degrade();
    let sda = pins.p26.into_floating_input().degrade();

    let i2c_pins = i2cPins{scl, sda};

    let mut i2c = Twim::new(p.TWIM1, i2c_pins, Frequency::K250);

    let atecc608a = ATECC608A::new(&i2c).unwrap();

    let _response = match atecc608a.wake(&mut i2c) {
        Ok(v)   => v,
        Err(_e) => panic!("Error"),
    };

    loop {
         
    }
}

#[exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}