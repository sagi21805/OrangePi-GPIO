use crate::constants::*;
use std::fs::File;
use std::io::prelude::*;

pub struct PinControler {
    gpio: GPIO_PIN,
}

impl PinControler {


    pub fn new(gpio: GPIO_PIN) -> Self {
        
        gpio.open();        
        PinControler {
            gpio,
        }

    }


    pub fn get_mode(&self) -> Mode { self.gpio.get_mode() }    


    pub fn get_value(&self) -> Value { self.gpio.get_value() }


    pub fn set_mode(&self, mode: Mode) {

        let path = format!("{}/direction", &self.gpio.as_path());
        let mut file = File::create(&path).expect("Can't open export file");
        file.write_all(
            mode.to_string().as_bytes()
        ).expect(format!("Failed to write to {}", path).as_str());
    
    }    


    pub fn set_value(&self, value: Value) {

        let path = format!("{}/value", &self.gpio.as_path());
        let mut file = File::create(&path).expect("Can't open export file");
        file.write_all(
            value.to_string().as_bytes()
        ).expect(format!("Failed to write to {}", path).as_str());
    }

    
    pub fn blink(&self, duration: std::time::Duration) {
        loop {
            self.set_value(Value::HIGH);
            std::thread::sleep(duration);
            self.set_value(Value::LOW);
            std::thread::sleep(duration);
        }
    }

}


impl Drop for PinControler {
    fn drop(&mut self) {
        self.set_value(Value::LOW);
        self.gpio.close();
    }
}