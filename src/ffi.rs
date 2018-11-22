// Copyright (c) 2018 Teddy Wing
//
// This file is part of DomeKey.
//
// *Purchasing policy notice:* All users of the software are expected to
// purchase a license from Teddy Wing unless they have a good reason not to
// pay. Users who can't purchase a license may apply to receive one for free
// at inquiry@domekey.teddywing.com. Users are free to:
//
// * download, build, and modify the app;
// * share the modified source code;
// * share the purchased or custom-built binaries (with unmodified license
//   and contact info), provided that the purchasing policy is explained to
//   all potential users.
//
// This software is available under a modified version of the Open Community
// Indie Software License:
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose is hereby granted, subject to the following conditions:
//
// * all copies retain the above copyright notice, the above purchasing
//   policy notice and this permission notice unmodified;
//
// * all copies retain the name of the software (DomeKey), the name of the
//   author (Teddy Wing), and contact information (including, but not limited
//   to, inquiry@domekey.teddywing.com, and domekey.teddywing.com URLs)
//   unmodified;
//
// * no fee is charged for distribution of the software;
//
// * the best effort is made to explain the purchasing policy to all users of
//   the software.
//
// THE SOFTWARE IS PROVIDED "AS IS", AND THE AUTHOR AND COPYRIGHT HOLDERS
// DISCLAIM ALL WARRANTIES, EXPRESS OR IMPLIED, WITH REGARD TO THIS SOFTWARE,
// INCLUDING BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE. IN NO EVENT SHALL THE AUTHOR OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY
// DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA, OR PROFITS, WHETHER
// IN AN ACTION OF CONTRACT, NEGLIGENCE, OR OTHER TORTIOUS ACTION, ARISING
// OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use std::ffi::{CStr, CString};
use std::fs;
use std::ptr;
use std::slice;

use libc::{c_char, size_t};
use stderrlog;
use xdg;

use {HeadphoneButton, MapGroup};
use config::{self, Config};
use errors::*;
use map::{ModeChange, run_key_action};
use trial;

#[repr(C)]
#[derive(Debug)]
pub struct Trigger {
    pub buttons: *const HeadphoneButton,
    pub length: size_t,
}

#[derive(Default)]
pub struct State {
    pub in_mode: Option<Vec<HeadphoneButton>>,
    pub map_group: Option<MapGroup>,
    mappings_str: String,
}

#[no_mangle]
pub extern "C" fn dome_key_logger_init() {
    stderrlog::new()
        .module(module_path!())
        .color(stderrlog::ColorChoice::Never)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init()
        .unwrap_or_else(|e| dkeprintln!("{}", e));
}

#[no_mangle]
pub extern "C" fn dome_key_state_new() -> *mut State {
    Box::into_raw(Box::new(State::default()))
}

#[no_mangle]
pub extern "C" fn dome_key_state_free(ptr: *mut State) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern "C" fn dome_key_state_load_map_group(ptr: *mut State) {
    match xdg::BaseDirectories::with_prefix("dome-key") {
        Ok(xdg_dirs) => {
            let state = unsafe {
                assert!(!ptr.is_null());
                &mut *ptr
            };

            match xdg_dirs.find_config_file("mappings.dkmap") {
                Some(mapping_file) => {
                    // Store the mapping string contents in `State`. Otherwise
                    // the reference doesn't live long enough.
                    state.mappings_str = match fs::read_to_string(mapping_file)
                        .chain_err(|| "failed to read 'mappings.dkmap'")
                    {
                        Ok(s) => s,
                        Err(e) => {
                            error!("{}", e);

                            String::new()
                        },
                    };

                    state.map_group = match MapGroup::parse(&state.mappings_str) {
                        Ok(mut map_group) => {
                            Some(map_group)
                        },
                        Err(e) => {
                            error!("In 'mappings.dkmap': {}", e);
                            None
                        },
                    };
                },
                None => {
                    state.map_group = Some(MapGroup::default());

                    match xdg_dirs.get_config_home().to_str() {
                        Some(config_home) => {
                            error!(
                                "No mapping file found at '{}{}'. \
                                Using default mappings.",
                                config_home,
                                "mappings.dkmap"
                            )
                        },
                        None => {
                            error!("Config home path contains invalid unicode")
                        }
                    }
                },
            }
        },
        Err(e) => error!("{}", e),
    }
}

#[no_mangle]
pub extern "C" fn dome_key_run_key_action(
    state: *mut State,
    trigger: Trigger,
    on_mode_change: extern "C" fn(mode_change: ModeChange),
) {
    let trigger = unsafe {
        assert!(!trigger.buttons.is_null());

        slice::from_raw_parts(trigger.buttons, trigger.length as usize)
    };

    let mut state = unsafe {
        assert!(!state.is_null());
        &mut *state
    };

    match run_key_action(&mut state, trigger, on_mode_change) {
        Ok(_) => (),
        Err(e) => error!("{}", e),
    };
}

#[no_mangle]
pub extern "C" fn dome_key_parse_args(
    args: *const *const c_char,
    length: size_t,
    config_ptr: *mut Config
) -> *mut Config {
    let args = unsafe {
        assert!(!args.is_null());

        let args = slice::from_raw_parts(args, length as usize);

        args
            .iter()
            .map(|s| {
                assert!(!s.is_null());

                CStr::from_ptr(*s)
                    .to_string_lossy()
                    .into_owned()
            })
            .collect::<Vec<String>>()
    };

    let config = unsafe {
        assert!(!config_ptr.is_null());

        &mut *config_ptr
    };
    config::parse_args(&args, config);

    config_ptr
}

#[no_mangle]
pub extern "C" fn dome_key_config_get() -> *mut Config {
    match config::get_config() {
        Ok(config) => Box::into_raw(Box::new(config)),
        Err(e) => {
            error!("{}", e);

            ptr::null_mut()
        },
    }
}

#[no_mangle]
pub extern "C" fn dome_key_config_free(ptr: *mut Config) {
    if ptr.is_null() { return }
    let config = unsafe { Box::from_raw(ptr) };

    if config.args.license.is_null() { return }
    unsafe { CString::from_raw(config.args.license); }
}

#[no_mangle]
pub extern "C" fn dome_key_do_trial() {
    trial::do_trial();
}
