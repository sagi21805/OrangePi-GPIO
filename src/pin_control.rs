use crate::constants::*;
use crate::utils;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct PinControler {
    gpio: GPIO_PIN,
    pub mode: Mode,
    pub value: Value 
}

impl PinControler {


    pub fn new(gpio: GPIO_PIN) -> Self {
        
        gpio.open();
        let mode = gpio.get_mode();
        let value = gpio.get_value();
        
        PinControler {
            gpio,
            mode,
            value
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

}


impl Drop for PinControler {
    fn drop(&mut self) {
        self.gpio.close();
    }
}