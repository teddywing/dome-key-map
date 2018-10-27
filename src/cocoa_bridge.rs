use std::env;
use std::ffi::{CStr, CString, OsString};
use std::fs;
use std::process::Command;
use std::ptr;
use std::slice;

use libc::{c_char, size_t};
use stderrlog;
use xdg;

use {Action, HeadphoneButton, MapAction, MapGroup, MapKind};
use config::{self, Config};
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
    in_mode: Option<Vec<HeadphoneButton>>,
    map_group: Option<MapGroup>,
}

#[no_mangle]
pub extern "C" fn logger_init() {
    stderrlog::new()
        .module(module_path!())
        .color(stderrlog::ColorChoice::Never)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init()
        .unwrap();
}

#[no_mangle]
pub extern "C" fn state_new() -> *mut State {
    Box::into_raw(Box::new(State::default()))
}

#[no_mangle]
pub extern "C" fn state_free(ptr: *mut State) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}

#[no_mangle]
pub extern "C" fn state_load_map_group(ptr: *mut State) {
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
pub extern "C" fn c_run_key_action(
    state: *mut State,
    trigger: Trigger,
    mode: *const Trigger,
) {
    let trigger = unsafe {
        assert!(!trigger.buttons.is_null());

        slice::from_raw_parts(trigger.buttons, trigger.length as usize)
    };

    let mode = unsafe {
        if mode.is_null() {
            None
        } else {
            println!("In mode(110): {:?}", *mode);
            assert!(!(*mode).buttons.is_null());

            Some(
                slice::from_raw_parts((*mode).buttons, (*mode).length as usize)
            )
        }
    };
    println!("Mode after unsafe (118): {:?}", mode);

    let mut state = unsafe {
        assert!(!state.is_null());
        &mut *state
    };

    run_key_action_for_mode(&mut state, trigger, mode);
}

#[no_mangle]
pub extern "C" fn run_key_action_for_mode<'a>(
    state: &mut State,
    trigger: &'a [HeadphoneButton],
    in_mode: Option<&[HeadphoneButton]>
) {
    match state.map_group {
        Some(ref map_group) => {
            let map = map_group.maps.get(trigger);
            let mode = map_group.modes.get(trigger);

            if let Some(in_mode) = state.in_mode.clone() {
                if let Some(mode) = map_group.modes.get(&in_mode) {
                    // Deactivate mode by pressing current mode trigger
                    if &in_mode[..] == trigger {
                        state.in_mode = None;

                        return;
                    }

                    if let Some(map) = mode.get(trigger) {
                        run_action(&map);
                    }
                }
            }

            // TODO: make sure this doesn't run when in_mode
            if state.in_mode.is_none() {
                if let Some(map) = map {
                    run_action(&map);
                }
            }

            if let Some(mode) = mode {
                state.in_mode = Some(trigger.to_vec());
            }
        },
        None => (),
    }
}

fn run_action(map_action: &MapAction) {
    match map_action.kind {
        MapKind::Map => {
            if let Action::Map(action) = &map_action.action {
                for key in action {
                    key.tap()
                }
            }
        },
        MapKind::Command => {
            if let Action::String(action) = &map_action.action {
                let shell = match env::var_os("SHELL") {
                    Some(s) => s,
                    None => OsString::from("/bin/sh"),
                };

                match Command::new(shell)
                    .arg("-c")
                    .arg(action)
                    .spawn() {
                    Ok(_) => (),
                    Err(e) => error!(
                        "Command failed to start: `{}'",
                        e
                    ),
                }
            }
        },
    }
}

#[no_mangle]
pub extern "C" fn c_parse_args(
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
    let config = config::parse_args(&args, config);

    config_ptr
}

#[no_mangle]
pub extern "C" fn config_get() -> *mut Config {
    match config::get_config() {
        Ok(config) => Box::into_raw(Box::new(config)),
        Err(e) => {
            error!("{}", e);

            ptr::null_mut()
        },
    }
}

#[no_mangle]
pub extern "C" fn config_free(ptr: *mut Config) {
    if ptr.is_null() { return }
    let config = unsafe { Box::from_raw(ptr) };

    if config.args.license.is_null() { return }
    unsafe { CString::from_raw(config.args.license); }
}

#[no_mangle]
pub extern "C" fn do_trial() {
    trial::do_trial();
}
