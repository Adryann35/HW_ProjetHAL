#![no_std]

use core::ptr;

const UBRR0H: *mut u8 = 0xC5 as *mut u8;
const UBRR0L: *mut u8 = 0xC4 as *mut u8;
const UCSR0A: *mut u8 = 0xC0 as *mut u8;
const UCSR0B: *mut u8 = 0xC1 as *mut u8;
const UCSR0C: *mut u8 = 0xC2 as *mut u8;
const UDR0: *mut u8 = 0xC6 as *mut u8;

pub struct Usart;

impl Usart {
    /// Initialize the USART with a given baud rate
    pub fn init(baud: u16) {
        unsafe {
            // Set baud rate (using UBRR registers)
            ptr::write_volatile(UBRR0H, (baud >> 8) as u8);
            ptr::write_volatile(UBRR0L, baud as u8);

            // Set frame format: 8 data bits, 1 stop bit
            ptr::write_volatile(UCSR0C, (1 << 1) | (1 << 2));

            // Enable transmitter and receiver
            ptr::write_volatile(UCSR0B, (1 << 3) | (1 << 4));
        }
    }

    /// Transmit a single byte
    pub fn transmit(data: u8) {
        unsafe {
            // Wait for empty transmit buffer
            while ptr::read_volatile(UCSR0A) & (1 << 5) == 0 {}

            // Put data into buffer, sends the data
            ptr::write_volatile(UDR0, data);
        }
    }

    /// Receive a single byte
    pub fn receive() -> u8 {
        unsafe {
            // Wait for data to be received
            while ptr::read_volatile(UCSR0A) & (1 << 7) == 0 {}

            // Get and return received data from buffer
            ptr::read_volatile(UDR0)
        }
    }
}
