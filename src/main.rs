// src/main.rs

mod gpio;
use gpio::{Pin, PinMode};

fn main() {
    // Create an instance of Pin 2 (corresponding to Arduino digital pin 2)
    let pin2 = Pin::<2>::new();

    // Configure pin2 as input and read its value
    pin2.configure(PinMode::Input);
    let pin_value = pin2.read();
    println!("Pin 2 is {}", if pin_value { "HIGH" } else { "LOW" });

    // Configure pin2 as output and set it to HIGH
    pin2.configure(PinMode::Output);
    pin2.write(true);
    println!("Set Pin 2 to HIGH");

    // Set pin2 to LOW
    pin2.write(false);
    println!("Set Pin 2 to LOW");
}
