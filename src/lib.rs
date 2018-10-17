#![recursion_limit="128"]

extern crate autopilot;
extern crate cocoa;

#[macro_use]
extern crate combine;
extern crate core_graphics;
extern crate foreign_types;
extern crate getopts;
extern crate libc;

#[macro_use]
extern crate log;

#[macro_use]
extern crate objc;
extern crate stderrlog;
extern crate xdg;

mod autopilot_internal;
mod cocoa_bridge;
mod config;
mod key_code;
mod parser;

use parser::{Action, HeadphoneButton, MapAction, MapGroup, MapKind};

pub use cocoa_bridge::*;
pub use config::{Config, parse_args};
