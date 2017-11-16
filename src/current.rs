//! Utilities for getting information on the current microcontroller.

use Mcu;
use load;
use std::env;

/// Gets information about the current microcontroller, if it is set.
pub fn mcu() -> Option<Mcu> {
    mcu_name().map(|mcu_name| {
        load::microcontroller(&mcu_name)
    })
}

/// Gets the name of the current microcontroller, if it is set.
pub fn mcu_name() -> Option<String> {
    env::var("CARGO_CFG_TARGET_CPU").ok().map(|c| c.to_owned())
}

