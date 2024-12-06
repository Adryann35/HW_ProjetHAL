#![no_std]

use core::ptr;

// Base addresses and offsets (example for STM32 Cortex-M7 USART1)
const USART1_BASE: usize = 0x4001_3800;

const USART_BRR: usize = USART1_BASE + 0x0C; // Baud Rate Register
const USART_CR1: usize = USART1_BASE + 0x00; // Control Register 1
const USART_ISR: usize = USART1_BASE + 0x1C; // Interrupt and Status Register
const USART_TDR: usize = USART1_BASE + 0x28; // Transmit Data Register
const USART_RDR: usize = USART1_BASE + 0x24; // Receive Data Register

pub struct UsartCortexM7;

impl UsartCortexM7 {
    /// Initialize USART with a given baud rate
    pub fn init(baud: u32, clk: u32) {
        unsafe {
            // Calculate and set baud rate (BRR = clock / baud rate)
            let brr = clk / baud;
            ptr::write_volatile(USART_BRR as *mut u32, brr);

            // Enable USART, transmitter, and receiver in CR1
            let cr1 = (1 << 0) | // UE: USART Enable
                      (1 << 2) | // RE: Receiver Enable
                      (1 << 3);  // TE: Transmitter Enable
            ptr::write_volatile(USART_CR1 as *mut u32, cr1);
        }
    }

    /// Transmit a single byte
    pub fn transmit(data: u8) {
        unsafe {
            // Wait until TXE (Transmit Data Register Empty) is set
            while ptr::read_volatile(USART_ISR as *const u32) & (1 << 7) == 0 {}

            // Write data to Transmit Data Register
            ptr::write_volatile(USART_TDR as *mut u32, data as u32);
        }
    }

    /// Receive a single byte
    pub fn receive() -> u8 {
        unsafe {
            // Wait until RXNE (Read Data Register Not Empty) is set
            while ptr::read_volatile(USART_ISR as *const u32) & (1 << 5) == 0 {}

            // Read and return data from Receive Data Register
            ptr::read_volatile(USART_RDR as *const u32) as u8
        }
    }
}
