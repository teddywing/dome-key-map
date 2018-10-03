#![recursion_limit="128"]

extern crate autopilot;
// extern crate cocoa;

#[macro_use]
extern crate combine;
extern crate libc;

#[macro_use]
extern crate log;
extern crate stderrlog;
extern crate xdg;

mod cocoa_bridge;
mod parser;

use parser::{Action, HeadphoneButton, MapAction, MapGroup, MapKind};

pub use cocoa_bridge::*;
