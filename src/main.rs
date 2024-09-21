mod constants;
mod pin_control;
mod utils;
use constants::*;
use pin_control::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let controller = PinControler::new(GPIO_PIN::GPIO1_C2);
    controller.set_mode(Mode::OUT);
    controller.set_value(Value::LOW);

    loop {

        sleep(Duration::from_millis(30));
        controller.set_value(Value::HIGH);
        sleep(Duration::from_millis(30));
        controller.set_value(Value::LOW);

    }

}
