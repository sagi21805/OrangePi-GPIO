use crate::constants::*;
use std::{fs::File, io::{Read, Write}};
use std::io::BufReader;
use std::io::prelude::*;
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

    pub fn is_open(&self) -> bool { self.gpio.is_open() }

    pub fn open(&self) { self.gpio.open(); }

    pub fn close(&self) { self.gpio.close(); }

    pub fn get_mode(&self) -> Mode { self.gpio.get_mode() }    

    pub fn get_value(&self) -> Value { self.gpio.get_value() }

}


impl Drop for PinControler {
    fn drop(&mut self) {
        self.close();
    }
}