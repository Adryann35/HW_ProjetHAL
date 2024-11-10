// src/gpio.rs

#![no_std]

use core::ptr;

const DDRD: *mut u8 = 0x2A as *mut u8;
const PORTD: *mut u8 = 0x2B as *mut u8;
const PIND: *const u8 = 0x29 as *const u8;

pub enum PinMode {
    Input,
    Output,
}

pub struct Pin<const PIN: u8>;

impl<const PIN: u8> Pin<PIN> {
    /// Configures the pin as input or output
    pub fn configure(&self, mode: PinMode) {
        unsafe {
            match mode {
                PinMode::Input => {
                    // Clear the bit in DDRD to configure as input
                    ptr::write_volatile(DDRD, ptr::read_volatile(DDRD) & !(1 << PIN));
                }
                PinMode::Output => {
                    // Set the bit in DDRD to configure as output
                    ptr::write_volatile(DDRD, ptr::read_volatile(DDRD) | (1 << PIN));
                }
            }
        }
    }

    /// Writes a value to the pin (only works if configured as output)
    pub fn write(&self, value: bool) {
        unsafe {
            if value {
                // Set the bit in PORTD to drive the pin high
                ptr::write_volatile(PORTD, ptr::read_volatile(PORTD) | (1 << PIN));
            } else {
                // Clear the bit in PORTD to drive the pin low
                ptr::write_volatile(PORTD, ptr::read_volatile(PORTD) & !(1 << PIN));
            }
        }
    }

    /// Reads the value of the pin (only works if configured as input)
    pub fn read(&self) -> bool {
        unsafe {
            // Read the bit in PIND to get the pin value
            (ptr::read_volatile(PIND) & (1 << PIN)) != 0
        }
    }
}


