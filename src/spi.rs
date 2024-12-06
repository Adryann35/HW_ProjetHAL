#![no_std]

use core::ptr;

const SPCR: *mut u8 = 0x4C as *mut u8; // SPI Control Register
const SPSR: *mut u8 = 0x4D as *mut u8; // SPI Status Register
const SPDR: *mut u8 = 0x4E as *mut u8; // SPI Data Register
const DDRB: *mut u8 = 0x24 as *mut u8; // Data Direction Register B
const PORTB: *mut u8 = 0x25 as *mut u8; // Port B Register

pub struct Spi;

impl Spi {
    /// Initialize SPI as Master
    pub fn init_master() {
        unsafe {
            // Set MOSI (PB3) and SCK (PB5) as outputs, MISO (PB4) as input
            ptr::write_volatile(DDRB, ptr::read_volatile(DDRB) | (1 << 3) | (1 << 5));
            ptr::write_volatile(DDRB, ptr::read_volatile(DDRB) & !(1 << 4));

            // Enable SPI, Set as Master, and set clock rate (fosc/16)
            ptr::write_volatile(SPCR, (1 << 6) | (1 << 4) | (1 << 0));
        }
    }

    /// Initialize SPI as Slave
    pub fn init_slave() {
        unsafe {
            // Set MISO (PB4) as output, MOSI (PB3) and SCK (PB5) as inputs
            ptr::write_volatile(DDRB, ptr::read_volatile(DDRB) | (1 << 4));
            ptr::write_volatile(DDRB, ptr::read_volatile(DDRB) & !((1 << 3) | (1 << 5)));

            // Enable SPI, Set as Slave
            ptr::write_volatile(SPCR, (1 << 6));
        }
    }

    /// Transmit and receive data (full-duplex)
    pub fn transfer(data: u8) -> u8 {
        unsafe {
            // Load data into SPI Data Register
            ptr::write_volatile(SPDR, data);

            // Wait until transmission is complete (SPIF set in SPSR)
            while ptr::read_volatile(SPSR) & (1 << 7) == 0 {}

            // Return received data from SPI Data Register
            ptr::read_volatile(SPDR)
        }
    }
}
