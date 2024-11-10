#![no_std]
#![no_main]

mod usart;
use usart::Usart;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn main() -> ! {
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

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

