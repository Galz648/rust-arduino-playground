#![no_std]
#![no_main]

use ufmt;
// import println! macro
// use arduino_hal::{port::mode::PullUp, hal::port::PD6};
use arduino_hal::simple_pwm::*;
use panic_halt as _;
// use ufmt::info;
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);


    // output
    let mut d3 = pins.d3.into_output();
    let mut d4 = pins.d4.into_output();
    // input
    // let a1 = pins.a1.into_analog_input(&mut adc);

    // let mut led = pins.d13.into_output();
    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);

    // Digital pin 5 is connected to a LED and a resistor in series
    let mut pwm_led = pins.d5.into_output().into_pwm(&timer0);
    pwm_led.enable();
    loop {
        d3.set_low();
        d4.set_high();
        
        for x in (0..=255).chain((0..=254).rev()) {
            pwm_led.set_duty(x);
            ufmt::uwriteln!(&mut serial, "x: {}", &x).unwrap();
            arduino_hal::delay_ms(10);
        }
        // analog write
        // let val = a1.read();
    }
}

