// src/gpio.rs

use avr_device::atmega328p::Peripherals;
use core::marker::PhantomData;

pub enum PinMode {
    Input,
    Output,
}

pub struct Pin<const PIN: u8> {
    _pin: PhantomData<u8>,
}

impl<const PIN: u8> Pin<PIN> {
    /// Creates a new instance of the pin.
    pub fn new() -> Self {
        Self {
            _pin: PhantomData,
        }
    }

    /// Configures the pin as input or output
    pub fn configure(&self, mode: PinMode) {
        let peripherals = unsafe { Peripherals::steal() };
        let portd = &peripherals.PORTD;
        
        match mode {
            PinMode::Input => {
                // Clear the bit in DDRD to configure as input
                portd.ddrd.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << PIN)) });
            }
            PinMode::Output => {
                // Set the bit in DDRD to configure as output
                portd.ddrd.modify(|r, w| unsafe { w.bits(r.bits() | (1 << PIN)) });
            }
        }
    }

    /// Writes a value to the pin (only works if configured as output)
    pub fn write(&self, value: bool) {
        let peripherals = unsafe { Peripherals::steal() };
        let portd = &peripherals.PORTD;

        if value {
            // Set the bit in PORTD to drive the pin high
            portd.portd.modify(|r, w| unsafe { w.bits(r.bits() | (1 << PIN)) });
        } else {
            // Clear the bit in PORTD to drive the pin low
            portd.portd.modify(|r, w| unsafe { w.bits(r.bits() & !(1 << PIN)) });
        }
    }

    /// Reads the value of the pin (only works if configured as input)
    pub fn read(&self) -> bool {
        let peripherals = unsafe { Peripherals::steal() };
        let portd = &peripherals.PORTD;

        // Read the bit in PIND to get the pin value
        (portd.pind.read().bits() & (1 << PIN)) != 0
    }
}
