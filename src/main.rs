/*
[CORRECTION USART] (Don't hesitate to remove this comment)
Where is your second target?


[CORRECTION SPI] (Don't hesitate to remove this comment)
You didn't implement the reception part of the SPI feature for your ATMEGA328P target.
You should implement the peripheral/slave mode as well (not only the controler/master mode) for your CORTEX M7 target.
*/
#![no_std]
#![no_main]

mod gpio;                                  
use gpio::{Pin, PinMode};   // GPIO module

mod usart;
use usart::Usart;   // USART module

mod spi;
use spi::Spi;   // SPI module

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() -> ! {
    // Example: GPIO
    gpio_example();

    // Example: USART
    usart_example();

    // Example: SPI
    spi_example();

    // Infinite loop to keep the microcontroller running
    loop {}
}

/// GPIO Example: Configuring a pin as output, writing to it, and toggling.
fn gpio_example() {
    // Create an instance of Pin 2 (corresponding to Arduino digital pin 2)
    let pin2 = Pin::<2>;

    // Configure pin2 as input and read its value
    pin2.configure(PinMode::Input);
    let pin_value = pin2.read();
    // (In actual embedded use, you would likely use an LED or serial to check the output)

    // Configure pin2 as output and set it to HIGH
    pin2.configure(PinMode::Output);
    pin2.write(true);

    // Set pin2 to LOW
    pin2.write(false);
}

/// USART Example: Transmitting and receiving data.
fn usart_example() {
    // Initialize USART with a common baud rate (9600 in this case)
    Usart::init(103); // 9600 baud rate for 16MHz clock

    // Transmit a byte
    Usart::transmit(b'H');
    Usart::transmit(b'e');
    Usart::transmit(b'l');
    Usart::transmit(b'l');
    Usart::transmit(b'o');

    // Receive a byte and transmit it back (echo functionality)
    let received = Usart::receive();
    Usart::transmit(received);
}

/// SPI Example: Transmitting and receiving data using SPI.
fn spi_example() {
    // Initialize SPI as Master
    Spi::init_master();

    // Transmit a byte and receive the response
    let received = Spi::transfer(0xA5);

    // Transmit another byte
    Spi::transfer(0x5A);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

