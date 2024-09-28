#![allow(dead_code)]
#![allow(non_camel_case_types)] 

pub const EXPORT_PATH: &'static str = "/sys/class/gpio/export";
pub const UNEXPORT_PATH: &'static str = "/sys/class/gpio/unexport";


#[derive(Clone)]
pub enum GPIO_PIN {
    GPIO0_D4 = 28,
    GPIO0_D5 = 29,
    GPIO1_A3 = 35,
    GPIO1_B6 = 46,
    GPIO1_B7 = 47, 
    GPIO1_C0 = 48,
    GPIO1_C1 = 49,
    GPIO1_C2 = 50,
    GPIO1_C4 = 52,
    GPIO1_C6 = 54,
    GPIO1_D2 = 58,
    GPIO1_D3 = 59,
    GPIO2_D4 = 60,
    GPIO4_A3 = 131,
    GPIO4_A4 = 132,
    GPIO4_B2 = 138,
    GPIO4_B3 = 139,
}

pub enum Mode {
    IN,
    OUT
}

pub enum Value {
    LOW,
    HIGH,
}