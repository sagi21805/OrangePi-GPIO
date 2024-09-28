#![allow(dead_code)]

use crate::constants::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use crate::utils;

impl GPIO_PIN {
    
    pub fn is_open(&self) -> bool {
        return Path::new(self.as_path()).exists()
    }


    pub fn is_closed(&self) -> bool {
        return !Path::new(self.as_path()).exists()
    }


    pub fn open(&self) {
        if self.is_open() { return; }
        let mut file = File::create(EXPORT_PATH).expect("Can't open export file");
        file.write_all(
            (self.clone() as i32).to_string().as_bytes()
        ).expect(format!("Failed to write to {}", EXPORT_PATH).as_str());
    }


    pub fn close(&self) {
        if self.is_closed() { return; }
        let mut file = File::create(UNEXPORT_PATH).expect("Can't open unexport file");
        file.write_all(
            (self.clone() as i32).to_string().as_bytes()
        ).expect(format!("Failed to write to {}", UNEXPORT_PATH).as_str());
    }


    pub const fn as_path(&self) -> &'static str {
        match self {
            GPIO_PIN::GPIO0_D4 => "/sys/class/gpio/gpio28",
            GPIO_PIN::GPIO0_D5 => "/sys/class/gpio/gpio29",
            GPIO_PIN::GPIO1_A3 => "/sys/class/gpio/gpio35",
            GPIO_PIN::GPIO1_B6 => "/sys/class/gpio/gpio46",
            GPIO_PIN::GPIO1_B7 => "/sys/class/gpio/gpio47", 
            GPIO_PIN::GPIO1_C0 => "/sys/class/gpio/gpio48",
            GPIO_PIN::GPIO1_C1 => "/sys/class/gpio/gpio49",
            GPIO_PIN::GPIO1_C2 => "/sys/class/gpio/gpio50",
            GPIO_PIN::GPIO1_C4 => "/sys/class/gpio/gpio52",
            GPIO_PIN::GPIO1_C6 => "/sys/class/gpio/gpio54",
            GPIO_PIN::GPIO1_D2 => "/sys/class/gpio/gpio58",
            GPIO_PIN::GPIO1_D3 => "/sys/class/gpio/gpio59",
            GPIO_PIN::GPIO2_D4 => "/sys/class/gpio/gpio60",
            GPIO_PIN::GPIO4_A3 => "/sys/class/gpio/gpio131",
            GPIO_PIN::GPIO4_A4 => "/sys/class/gpio/gpio132",
            GPIO_PIN::GPIO4_B2 => "/sys/class/gpio/gpio138",
            GPIO_PIN::GPIO4_B3 => "/sys/class/gpio/gpio139",
        }
    }


    pub fn get_mode(&self) -> Mode {

        if !self.is_open() { panic!("GPIO pin is not opened or it doesn't exist"); }
        let contents = utils::read_sys_file(
            format!("{}/direction", &self.as_path()).as_str()
        ).expect("Can't read file content");
        match contents.as_str() {
            "in\n" => Mode::IN,
            "in" => Mode::IN,
            "out\n" => Mode::OUT,
            "out" => Mode::OUT,
            _ => panic!("Invalid mode")
        }

    }    


    pub fn get_value(&self) -> Value {
        if !self.is_open() { panic!("GPIO pin is not opened or it doesn't exist"); }
        let contents = utils::read_sys_file(
            format!("{}/value", &self.as_path()).as_str()
        ).expect("Can't read file content");
        match contents.as_str() {
            "0" => Value::LOW,
            "1" => Value::HIGH,  
            "0\n" => Value::LOW,
            "1\n" => Value::HIGH,
            _ => panic!("Invalid value")
        }
    }

}


impl Mode {

    pub fn to_string(self) -> String {
        match self {
            Mode::IN => String::from("in"),
            Mode::OUT => String::from("out"),
        }
    }

}


impl Value {

    pub fn to_string(self) -> String {
        match self {
            Value::LOW => String::from("0"),
            Value::HIGH => String::from("1"),
        }
    }

}