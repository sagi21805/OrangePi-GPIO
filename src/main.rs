mod constants;
mod pin_control;
use std::pin::Pin;

use pin_control::*;

use constants::*;
fn main() {
    let controller = PinControler::new(GPIO_PIN::GPIO0_D4);
    println!("mode: {}, value: {}", controller.get_mode().to_string(), controller.get_value().to_string())
}
