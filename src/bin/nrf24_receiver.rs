#![no_std]
#![no_main]
use panic_halt as _;

use arduino_hal as hal;

use embedded_hal::blocking::delay::{DelayMs, DelayUs};
use hal::prelude::*;
use hal::spi;
use nrf24_rs::config::{DataPipe, NrfConfig, PALevel};
use nrf24_rs::{Nrf24l01, SPI_MODE};

use arduino_hal::Delay;
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
fn main() -> ! {
    // Take peripherals
    let dp = hal::pac::Peripherals::take().unwrap();

    // Initialize the different pins
    let pins = hal::pins!(dp);
    let ncs = pins.d10.into_output();
    let mosi = pins.d11.into_output();
    let miso = pins.d12.into_pull_up_input();
    let sclk = pins.d13.into_output();

    let mut ce = pins.d5.into_output();
    // Initialize SPI
    let settings = spi::Settings {
        data_order: spi::DataOrder::MostSignificantFirst,
        clock: spi::SerialClockRate::OscfOver4,
        mode: SPI_MODE, // SPI Mode defined in this crate
    };
    let (spi, ncs) = spi::Spi::new(dp.SPI, sclk, mosi, miso, ncs, settings);

    let mut my_delay = MyDelay {};

    // Setup some configuration values
    let config = NrfConfig::default()
        .channel(8)
        .pa_level(PALevel::Min)
        // We will use a payload size the size of our message
        .payload_size(12);

    // Initialize the chip
    let mut nrf_chip = Nrf24l01::new(spi, ce, ncs, &mut my_delay, config).unwrap();
    if !nrf_chip.is_connected().unwrap() {
        panic!("Chip is not connected.");
    }

    // Open reading pipe 0 with address "Node1".
    // The sender will have to open its writing pipe with the same address
    // in order to transmit this message successfully.
    nrf_chip.open_reading_pipe(DataPipe::DP0, b"Node1").unwrap();
    // Set the chip in RX mode
    nrf_chip.start_listening().unwrap();

    // Keep checking if there is any data available to read
    while !nrf_chip.data_available().unwrap() {
        // No data availble, wait 50ms, then check again
        my_delay.delay_ms(50u16);
    }
    // Now there is some data availble to read

    // Initialize empty buffer
    let mut buffer = [0; b"Hello world!".len()];
    nrf_chip.read(&mut buffer).unwrap();

    // assert_eq!(buffer, b"Hello world!"); TODO: CHECK THIS

    loop {}
}
