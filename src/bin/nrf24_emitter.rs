#![no_std]
#![no_main]
use panic_halt as _;

use arduino_hal as hal;

use core::default::Default;
use hal::spi;
use nrf24_rs::config::{NrfConfig, PALevel};
use nrf24_rs::{Nrf24l01, SPI_MODE};

use arduino_hal::Delay;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

/*

TODO:
* Make a dynamic message size


*/
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

impl DelayUs<u16> for MyDelay {
    fn delay_us(&mut self, us: u16) {
        Delay::new().delay_us(us)
    }
}

impl DelayUs<u8> for MyDelay {
    fn delay_us(&mut self, us: u8) {
        Delay::new().delay_us(us)
    }
}

#[arduino_hal::entry]
fn entry() -> ! {
    // Take peripherals
    let dp = hal::pac::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let ncs = pins.d10.into_output();
    let mosi = pins.d11.into_output();
    let miso = pins.d12.into_pull_up_input();
    let sclk = pins.d13.into_output();

    // Initialize the different pins
    let mut ce_pin = pins.d5.into_output();
    // let (mut spi, _) = arduino_hal::Spi::new(
    //     dp.SPI,
    //     pins.d13.into_output(),
    //     pins.d11.into_output(),
    //     pins.d12.into_pull_up_input(),
    //     pins.d10.into_output(),
    //     spi::Settings::default(),
    // );

    // Initialize SPI
    let settings = spi::Settings {
        data_order: spi::DataOrder::MostSignificantFirst,
        clock: spi::SerialClockRate::OscfOver4,
        mode: SPI_MODE, // SPI Mode defined in this crate
    };
    let (spi, ncs) = spi::Spi::new(dp.SPI, sclk, mosi, miso, ncs, settings);

    // let mut delay = hal::delay::Delay::<hal::clock::MHz16>::new();

    let message = b"Hello world!"; // The message we will be sending
    let mut my_delay = MyDelay {};
    // Setup some configuration values
    let config = NrfConfig::default()
        .channel(8)
        .pa_level(PALevel::Min)
        // We will use a payload size the size of our message
        .payload_size(12); // TODO: change to dynamic message size

    // Initialize the chip

    let mut nrf_chip = Nrf24l01::new(spi, ce_pin, ncs, &mut my_delay, config).unwrap();
    if !nrf_chip.is_connected().unwrap() {
        panic!("Chip is not connected.");
    }

    // // Open a writing pipe on address "Node1".
    // // The listener will have to open a reading pipe with the same address
    // // in order to recieve this message.
    nrf_chip.open_writing_pipe(b"Node1").unwrap();
    // /`/ Keep trying to send the message
    let _ = nrf_chip.write(&mut my_delay, message);
    while let Err(_e) = nrf_chip.write(&mut my_delay, message) {
        // Something went wrong while writing, try again in 50ms
        my_delay.delay_ms(50u16);
    }
    //     // Something went wrong while writing, try again in 50ms
    //     // delay.delay_ms(50u16);
    arduino_hal::delay_ms(50u16);
    // }

    // Message should now successfully have been sent!
    loop {}
}
