#![allow(unused)]
#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{self},
    pwm::{self, ChannelAPin, SetDutyCycle},
};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals_config = Default::default();
    // Peripherals.
    let p = embassy_rp::init(peripherals_config);
    let input = gpio::Input::new(p.PIN_1, gpio::Pull::Down);
    let mut output = gpio::Output::new(p.PIN_0, gpio::Level::Low);

    loop {
        if input.is_high() {
            output.set_level(gpio::Level::High);
        } else {
            output.set_level(gpio::Level::Low);
        }
    }
}
