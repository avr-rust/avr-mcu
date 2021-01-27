//! Utilities for querying information about the microcontroller being targeted.

use load;
use std::env;
use Mcu;

/// Gets information about the current microcontroller.
///
/// Returns `None` if the target architecture is not AVR.
///
/// When targeting AVR, this function will always return `Some(mcu)`.
pub fn mcu() -> Option<Mcu> {
    mcu_name().map(|mcu_name| load::microcontroller(&mcu_name))
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
    target_cpu_fetch::target_cpu().ok().and_then(|o| o)
}

/// Checks if the current cargo target architecture is AVR.
pub fn is_compiling_for_avr() -> bool {
    env::var("CARGO_CFG_TARGET_ARCH") == Ok("avr".to_string())
}

