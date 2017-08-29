//! Information about every AVR microcontroller.
//!
//! # Examples
//!
//! ```
//! for mcu in avr_mcu::microcontrollers() {
//!     println!("Device: {}", mcu.device.name);
//! }
//! ```

extern crate xmltree;
#[macro_use]
extern crate lazy_static;

pub use self::model::*;
pub use self::load::{microcontroller, microcontrollers};

mod model;
mod pack;
mod load;

pub mod current;

