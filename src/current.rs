//! Utilities for querying information about the microcontroller being targeted.

use Mcu;
use load;
use std::env;

/// Gets information about the current microcontroller.
///
/// Returns `None` if the target architecture is not AVR.
///
/// When targeting AVR, this function will always return `Some(mcu)`.
pub fn mcu() -> Option<Mcu> {
    mcu_name().map(|mcu_name| {
        load::microcontroller(&mcu_name)
    })
}

/// Gets the name of the microcontroller being targeted.
///
/// Returns `None` if the target architecture is not AVR.
///
/// When targeting AVR, this function will always return `Some(mcu_name)`.
///
/// # Example results
///
/// * `Some("atmega328")`
/// * `Some("attiny85")`
/// * `None`
pub fn mcu_name() -> Option<String> {
    if is_targeting_avr() {
        // The avr-rust compiler fork has a special configuration option
        // that exposes the target microcontroller name in lowercase.
        match env::var("CARGO_CFG_TARGET_CPU") {
            Ok(target_cpu) => Some(target_cpu),
            Err(..) => panic!("cannot retrieve mcu name, use a version of the avr-rust compiler that exposes the 'target_cpu' configuration option"),
        }
    } else {
        None
    }
}

fn is_targeting_avr() -> bool {
    env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "avr"
}

