#![no_std]
#![no_main]

// extern crate compiler_builtins;

use arduino_hal::Delay;
use embedded_hal::blocking::delay::DelayMs;
// use embedded_hal::blocking::delay::DelayMs;
// use i2cdev::linux::LinuxI2CError;
// use linux_embedded_hal::{Delay, I2cdev};
use mpu6050::*;
// use panic_halt as _;
// #[cfg(not(debug_assertions))]
use panic_halt as _;

use ufmt::uwriteln;

struct MyDelay {}

impl DelayMs<u16> for MyDelay {
    fn delay_ms(&mut self, ms: u16) {
        Delay::new().delay_ms(ms)
    }
}

impl DelayMs<u8> for MyDelay {
    fn delay_ms(&mut self, ms: u8) {
        Delay::new().delay_ms(ms)
    }
}

// use embedded_hal::blocking::delay::{DelayMs, DelayUs};

/// Delay type for `embedded-hal` compatibility.
///
/// This type can be used to pass a generic delay utility to `embedded-hal` drivers.  For direct
/// use in `arduino-hal` code, usage of [`delay_ms`] or [`delay_us`] is preferred.

/// Delay execution for a number of milliseconds.
///
/// Busy-loop for the given time.  This function assumes the default clock speed defined by
/// [`arduino_hal::DefaultClock`][crate::DefaultClock].

/// Delay execution for a number of microseconds.
///
/// Busy-loop for the given time.  This function assumes the default clock speed defined by
/// [`arduino_hal::DefaultClock`][crate::DefaultClock].
// #[cfg(not(doc))]
// #[panic_handler]
// fn panic(info: &core::panic::PanicInfo) -> ! {
//     // disable interrupts - firmware has panicked so no ISRs should continue running
//     avr_device::interrupt::disable();

//     // get the peripherals so we can access serial and the LED.
//     //
//     // SAFETY: Because main() already has references to the peripherals this is an unsafe
//     // operation - but because no other code can run after the panic handler was called,
//     // we know it is okay.
//     let dp = unsafe { arduino_hal::Peripherals::steal() };
//     let pins = arduino_hal::pins!(dp);
//     let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

//     // Print out panic location
//     ufmt::uwriteln!(&mut serial, "Firmware panic!\r").void_unwrap();
//     if let Some(loc) = info.location() {
//         ufmt::uwriteln!(
//             &mut serial,
//             "  At {}:{}:{}\r",
//             loc.file(),
//             loc.line(),
//             loc.column(),
//         )
//         .void_unwrap();
//     }

//     // Blink LED rapidly
//     let mut led = pins.d13.into_output();
//     loop {
//         led.toggle();
//         arduino_hal::delay_ms(100);
//     }
// }
#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let mut buffer = [0u8; 8];

    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );
    // let i2c = I2cdev::new("/dev/i2c-1").map_err(Mpu6050Error::I2c)?;
    // let mut delay = Delay;
    let mut mpu = Mpu6050::new(i2c);

    mpu.get_accel_hpf();
    // mpu.get_gyro();
    // let mut delay = myDelay::new((1000);
    // mpu.init(&mut delay);
    let mut led = pins.d13.into_output();
    // let result = mpu.get_gyro();
    loop {
        led.toggle();
    }
}

//     // // get gyro data, scaled with sensitivity
// let gyro = mpu.get_gyro();
//     // uwriteln!(&mut serial, "gyro: {:?}", gyro);

//     // // get accelerometer data, scaled with sensitivity
//     let acc = mpu.get_acc();
//     // uwriteln!(&mut serial, "acc: {:?}", acc);
// }
