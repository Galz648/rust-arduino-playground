#![no_std]
#![no_main]

/* work in progress: ultrasonic IO device */
// import println! macro
use arduino_hal::port::mode::PullUp;
use panic_halt as _;
mod stdout;
use stdout::{put_console, subfunction, demo_print_without_ln};
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);

    // println!("Hello from main!");
    subfunction();
    demo_print_without_ln();

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut trig = pins.d5.into_output();
    let mut echo = pins.d6.into_pull_up_input();
    let mut distance: Option<i32> = None;
    loop {
        // arduino_hal::delay_ms(1000);
        // println!("Hello, world!");
        trig.set_high();
        arduino_hal::delay_ms(1000);
        trig.set_low();


    }
}

fn get_distance(echo: &mut arduino_hal::port::Pin<arduino_hal::port::mode::Input<PullUp>>,
    trig: &mut arduino_hal::port::Pin<arduino_hal::port::mode::Output>) -> i32 {
        
    
// Your distance calculation logic here
// println!("echo: {echo}");
// println!("distance: {distance}");

return 0;
}
