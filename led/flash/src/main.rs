#![allow(unused)]
#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_rp::{
    flash::{self},
    gpio,
    peripherals::FLASH,
};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

const FLASH_SIZE: usize = 2 * 1024 * 1024;
const SAVE_BASE: u32 = (FLASH_SIZE - 2 * flash::ERASE_SIZE) as u32;
const SECTOR_SIZE: u32 = flash::ERASE_SIZE as u32;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals_config = Default::default();
    // Peripherals.
    let p = embassy_rp::init(peripherals_config);
    let indicator = gpio::Output::new(p.PIN_0, gpio::Level::Low);

    let count = match load_u32(&mut flash) {
        Ok(Some(value)) => {
            defmt::info!("Loaded saved value: {}", value);
        }
        Ok(None) => {
            defmt::info!("No saved value yet");
        }
        Err(e) => {
            defmt::error!("Flash read failed: {:?}", e);
        }
    };

    let value_to_save = 42;

    match save_u32(&mut flash, value_to_save) {
        Ok(()) => defmt::info!("Saved value"),
        Err(e) => defmt::error!("Flash save failed: {:?}", e),
    }

    loop {
        Timer::after_millis(100).await;
    }
}

fn save_u32(
    flash: &mut flash::Flash<'_, FLASH, flash::Async, FLASH_SIZE>,
    value: u32,
) -> Result<(), embassy_rp::flash::Error> {
    let sector_end = SAVE_BASE + SECTOR_SIZE;
    flash.blocking_erase(SAVE_BASE, sector_end)?;
    let mut page = [0xFFu8; 256];
    page[..4].copy_from_slice(&value.to_le_bytes());
    flash.blocking_write(SAVE_BASE, &page)?;

    Ok(())
}

fn load_u32(
    flash: &mut flash::Flash<'_, FLASH, flash::Async, FLASH_SIZE>,
) -> Result<Option<u32>, embassy_rp::flash::Error> {
    let mut bytes = [0u8; 4];
    flash.blocking_read(SAVE_BASE, &mut bytes)?;
    // Erased flash reads as 0xFF.
    if bytes == [0xFF; 4] {
        return Ok(None);
    }
    Ok(Some(u32::from_le_bytes(bytes)))
}
