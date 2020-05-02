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
    // TARGET environment variable should contain the value that was specified
    // by --target X(C)argo option. Normally it's the name of .json file
    // containing custom target specification, e. g. atmega88pa.json
    // So in order to work, the name of *.json file should be the same
    // as the name of your MCU
    let target = env::var("TARGET").expect(
        "cannot retrieve mcu name, please, pass --target \
         flag to Cargo, e. g. \"--target atmega88pa\"",
    );
    let is_avr = env::var("CARGO_CFG_TARGET_ARCH").unwrap() == "avr";

    match is_avr {
        true => Some(target),
        false => None,
    }
}
