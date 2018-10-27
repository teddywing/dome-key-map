use std::ffi::{CStr, CString};
use std::fs;
use std::ptr;
use std::slice;

use libc::{c_char, size_t};
use stderrlog;
use xdg;

use {HeadphoneButton, MapGroup};
use config::{self, Config};
use map::run_key_action_for_mode;
use trial;

#[repr(C)]
#[derive(Debug)]
pub struct Trigger {
    pub buttons: *const HeadphoneButton,
    pub length: size_t,
}

#[repr(C)]
pub enum ActionKind {
    Map,
    Command,
    Mode,
}

#[derive(Default)]
pub struct State {
    pub in_mode: Option<Vec<HeadphoneButton>>,
    pub map_group: Option<MapGroup>,
}

#[no_mangle]
pub extern "C" fn dome_key_logger_init() {
    stderrlog::new()
        .module(module_path!())
        .color(stderrlog::ColorChoice::Never)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init()
        .unwrap();
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
            match xdg_dirs.find_config_file("mappings.dkmap") {
                Some(mapping_file) => {
                    let state = unsafe {
                        assert!(!ptr.is_null());
                        &mut *ptr
                    };

                    let dkmap = fs::read_to_string(mapping_file)
                        .expect("Failed to read 'mappings.dkmap'");

                    let mut map_group = MapGroup::parse(&dkmap)
                        .expect("Failed to parse 'mappings.dkmap'");
                    map_group.parse_actions();

                    state.map_group = Some(map_group);
                },
                None => {
                    match xdg_dirs.get_config_home().to_str() {
                        Some(config_home) => {
                            error!(
                                "No mapping file found at '{}{}'",
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
) {
    let trigger = unsafe {
        assert!(!trigger.buttons.is_null());

        slice::from_raw_parts(trigger.buttons, trigger.length as usize)
    };

    let mut state = unsafe {
        assert!(!state.is_null());
        &mut *state
    };

    run_key_action_for_mode(&mut state, trigger);
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
