#![no_std]
#![no_main]


use ufmt;
// import println! macro
// use arduino_hal::{port::mode::PullUp, hal::port::PD6};
// use arduino_hal::simple_pwm::*;
use panic_halt as _;
// use ufmt::info;
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut x_analog: u16;
    let mut y_analog: u16;
    let mut switch_state:bool;

    // input pins
    let dir_x = pins.d4.into_analog_input(adc)
    let dir_y = pins.a3.into_analog_input(&mut adc);    
    let switch = pins.a5.into_pull_up_input();
    
    loop {  
        x_analog = dir_x.analog_read(&mut adc);
        y_analog = dir_y.analog_read(&mut adc);
        switch_state = switch.is_high();
        ufmt::uwrite!(&mut serial, "switch value: {:#?}\n",(x_analog, y_analog, switch_state)).unwrap(); // find other way to handle than unwrap
        arduino_hal::delay_ms(250);
    }
}


// normalize the values given. (-1, 1)
