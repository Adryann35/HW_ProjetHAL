#![no_std]

use core::ptr;

// Base addresses and offsets for SPI1 on STM32 Cortex-M7 (e.g., STM32F7)
const SPI1_BASE: usize = 0x4001_3000;

const SPI1_CR1: *mut u32 = (SPI1_BASE + 0x00) as *mut u32; // Control Register 1
const SPI1_CR2: *mut u32 = (SPI1_BASE + 0x04) as *mut u32; // Control Register 2
const SPI1_SR: *mut u32 = (SPI1_BASE + 0x08) as *mut u32;  // Status Register
const SPI1_DR: *mut u32 = (SPI1_BASE + 0x0C) as *mut u32;  // Data Register

pub struct SpiCortexM7;

impl SpiCortexM7 {
    /// Initialize SPI in Master Mode
    pub fn init_master() {
        unsafe {
            // Configure SPI in Master Mode with default settings
            ptr::write_volatile(SPI1_CR1, (1 << 6) | // SPE: SPI Enable
                                            (1 << 2) | // MSTR: Master Selection
                                            (3 << 3)); // BR: Baud Rate Control (f_PCLK / 16)

            // Configure SPI1_CR2 (optional: for frame format, data size, etc.)
            ptr::write_volatile(SPI1_CR2, 0);
        }
    }

    /// Transmit a single byte
    pub fn transmit(data: u8) {
        unsafe {
            // Wait until TXE (Transmit Buffer Empty) is set
            while ptr::read_volatile(SPI1_SR) & (1 << 1) == 0 {}

            // Write data to SPI Data Register
            ptr::write_volatile(SPI1_DR as *mut u8, data);
        }
    }

    /// Receive a single byte
    pub fn receive() -> u8 {
        unsafe {
            // Wait until RXNE (Receive Buffer Not Empty) is set
            while ptr::read_volatile(SPI1_SR) & (1 << 0) == 0 {}

            // Read and return data from SPI Data Register
            ptr::read_volatile(SPI1_DR as *const u8)
        }
    }

    /// Transmit and receive a single byte (full-duplex communication)
    pub fn transfer(data: u8) -> u8 {
        unsafe {
            // Write data to SPI Data Register
            Self::transmit(data);

            // Wait until RXNE (Receive Buffer Not Empty) is set
            while ptr::read_volatile(SPI1_SR) & (1 << 0) == 0 {}

            // Read and return data from SPI Data Register
            ptr::read_volatile(SPI1_DR as *const u8)
        }
    }
}
