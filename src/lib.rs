//! Information about every AVR microcontroller.
//!
//! # Device representation
//!
//! The API consists of a set of types that represent information about each
//! microcontroller. The top-level type is [`Mcu`](struct.Mcu.html), modelling
//! a single microcontroller.
//!
//! # Retrieving microcontroller information
//!
//! It is possible to look up information for a specific MCU, or all of them at once.
//!
//! ## Getting information for the current target
//!
//! In a lot of cases, we only care about the target microcontroller.
//!
//! ```nodoc
//! let mcu = avr_mcu::current::mcu().unwrap();
//! println!("Device: {}", mcu.device.name);
//! ```
//!
//! # Behind-the-hood
//!
//! This crate embeds a set of "packfiles" released by Atmel. These are XML
//! specifications containing all of the information exposed by this crate.
//!
//! You can see a list of all packfiles [here](https://github.com/avr-rust/avr-mcu/tree/master/packs).
//!
//! A build script takes these packfiles and persists them as data structures in Rust.
//!
//! # Examples
//!
//! ```nodoc
//! for mcu in avr_mcu::microcontrollers() {
//!     println!("Device: {}", mcu.device.name);
//! }
//! ```

#![feature(conservative_impl_trait)]

extern crate xmltree;
#[macro_use]
extern crate lazy_static;

pub use self::model::*;
pub use self::load::{microcontroller, microcontroller_names, microcontrollers};

mod model;
mod pack;
mod load;
mod extra_info;

pub mod current;

