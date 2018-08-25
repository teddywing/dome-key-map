extern crate cocoa;

#[macro_use]
extern crate combine;

mod cocoa_bridge;
mod parser;

use parser::{HeadphoneButton, MapGroup, MapKind};

pub use cocoa_bridge::run_key_action;
