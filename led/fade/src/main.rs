#![allow(unused)]
#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{Level, Output},
    pwm::{self, ChannelAPin, SetDutyCycle},
};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals_config = Default::default();
    // Peripherals.
    let p = embassy_rp::init(peripherals_config);

    let pwm_config = Default::default();
    let mut external_led = pwm::Pwm::new_output_a(p.PWM_SLICE0, p.PIN_0, pwm_config);
    let mut duty_cycle: i8 = 0;
    let mut fade: i8 = 1;

    loop {
        external_led.set_duty_cycle_percent(duty_cycle as u8);

        Timer::after_millis(100).await;

        duty_cycle += fade;
        if duty_cycle <= 0 || duty_cycle >= 100 {
            fade = -fade;
        }
    }
}
