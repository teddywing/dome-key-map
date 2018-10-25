#![recursion_limit="128"]

extern crate autopilot;
extern crate chrono;
extern crate cocoa;

#[macro_use]
extern crate combine;
extern crate core_graphics;

#[macro_use]
extern crate error_chain;
extern crate exitcode;
extern crate foreign_types;
extern crate getopts;
extern crate libc;

#[macro_use]
extern crate log;
extern crate magic_crypt;

#[macro_use]
extern crate objc;

#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate serde_derive;
extern crate stderrlog;
extern crate toml;
extern crate xdg;

#[macro_use]
mod prefix_println;

mod autopilot_internal;
mod cocoa_bridge;
mod config;
mod errors;
mod key_code;
mod parser;
mod trial;

use parser::{Action, HeadphoneButton, MapAction, MapGroup, MapKind};

pub use cocoa_bridge::*;
pub use config::{Config, parse_args};
