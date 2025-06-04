mod constants;
mod pin_controller;
mod utils;
mod gpio;
use constants::*;
use pin_controller::*;
use std::thread::sleep;
use std::time::Duration;


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let c = PinControler::new(GPIO_PIN::GPIO1_A3);
        c.set_mode(Mode::OUT);
        for _ in 0..10 {
            c.set_value(Value::HIGH);
            println!("value: {}", c.get_value().to_string());
            sleep(Duration::from_millis(200));
            c.set_value(Value::LOW);
            println!("value: {}", c.get_value().to_string());
            sleep(Duration::from_millis(200));
        }
        assert_eq!(c.get_value().to_string().as_str(), "0");
    }

    #[test]
    fn motor_control() {
        let controller = PinControler::new(GPIO_PIN::GPIO1_A3);
        controller.cycle(Duration::from_micros(2000), Duration::from_millis(10));
        // assert_eq!(1, 2);
    }   
}

