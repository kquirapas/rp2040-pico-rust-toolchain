#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = Default::default();
    // Peripherals.
    let p = embassy_rp::init(config);
    info!("--- Pico 1 External LED Script Active ---");

    let mut external_led = Output::new(p.PIN_15, Level::Low);

    loop {
        external_led.set_high();
        Timer::after_millis(500).await;

        // external_led.set_low();
        Timer::after_millis(500).await;
    }
}
