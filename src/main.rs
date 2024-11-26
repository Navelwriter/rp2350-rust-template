//! # GPIO 'Blinky' Example
//!
//! This application demonstrates how to control a GPIO pin on the rp235x.
//!
//! It may need to be adapted to your particular board layout and/or pin assignment.

#![no_std]
#![no_main]

use defmt::*;
// Alias for our HAL crate
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::{Duration, Timer};
use gpio::{Level, Output};
use {defmt_rtt as _, panic_probe as _};
use embassy_rp::block::ImageDef;
// Create a logging macro
macro_rules! log { //Treat as: fn log!(Level, "Message")
    ($level:ident, $($arg:tt)*) => {
        {
            #[cfg(feature = "debug-logging")]
            defmt::$level!($($arg)*);
        }
    };
}

#[cortex_m_rt::pre_init]
unsafe fn before_main() {
    // Soft-reset doesn't clear spinlocks. Clear the one used by critical-section
    // before we hit main to avoid deadlocks when using a debugger
    embassy_rp::pac::SIO.spinlock(31).write_value(1);
}

/// Tell the Boot ROM about our application
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

/// The function configures the rp235x peripherals, then toggles a GPIO pin in
/// an infinite loop. If there is an LED connected to that pin, it will blink.
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    log!(info, "Start!");
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    loop {
        log!(info, "led on!");
        led.set_high();
        Timer::after_secs(1).await;

        log!(info, "led off!");
        led.set_low();
        Timer::after_secs(1).await;
    }
}