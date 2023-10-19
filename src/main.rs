#![no_std]
#![no_main]

/* work in progress: ultrasonic IO device */
// import println! macro
// use arduino_hal::{port::mode::PullUp, hal::port::PD6};
use panic_halt as _;
// use ufmt::info;
mod stdout;
use stdout::{put_console, subfunction, demo_print_without_ln};
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);
    subfunction();
    let mut trig = pins.d5.into_output();
    let mut echo = pins.d6.into_pull_up_input();
    let mut distance: Option<i32> = None;
    let mut led = pins.d13.into_output();
    loop {
        // arduino_hal::delay_ms(1000);
        // println!("Hello, world!");
        trig.set_high();
        // get_input(&mut echo);
        arduino_hal::delay_ms(10);
        trig.set_low();
        echo.is_high();
        arduino_hal::delay_ms(10);
        echo.is_low();
        if echo.is_high() {
            // builtin LED is turnt on
            led.set_high();

        } else {
            // builtin LED is turnt off
            led.set_low();
        }
    }
}
// // trig: &mut arduino_hal::port::Pin<arduino_hal::port::mode::Output>
// fn get_input(echo: &mut arduino_hal::port::Pin<arduino_hal::port::mode::Input<PullUp>>) -> i32 {
        
//     // get the
// // Your distance calculation logic here
// // println!("echo: {echo}");
// // println!("distance: {distance}");

// return 0;
// }
