/*!
 * This is an example of how one can define a println!() macro which can be called anywhere to
 * write data to the console.
 *
 * Keep in mind that this will enter a critical section while printing so no interrupts can be
 * served in the meantime.
 */
#![no_std]
#![no_main]

mod stdout;
use panic_halt as _;
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

    loop {}
}
