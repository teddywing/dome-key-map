use std::env;
use std::ffi::{CStr, CString, OsString};
use std::fs;
use std::mem;
use std::process::Command;
use std::ptr;
use std::slice;

use autopilot::key::type_string;
// use cocoa::base::nil;
// use cocoa::foundation::{NSArray, NSAutoreleasePool, NSDictionary};
use libc::{c_char, size_t};
use stderrlog;
use xdg;

use {Action, HeadphoneButton, MapAction, MapGroup, MapKind};
use config::{self, Config};

#[repr(C)]
struct renameMeMapGroup {
}

// pub extern "C" fn parse_mappings() {
//     let sample_maps = "map <up> k
// map <down> j";
//
//     let map_group = MapGroup::parse(sample_maps).unwrap();
//
//     unsafe {
//         let _pool = NSAutoreleasePool::new(nil);
//
//         let maps = NSDictionary::init(nil).autorelease();
//         let modes = NSDictionary::init(nil).autorelease();
//
//         for (trigger, action) in map_group.maps {
//             // let t = NSArray::arrayWithObjects(nil, &trigger).autorelease();
//
//             // maps.
//         }
//
//         for (trigger, modes) in map_group.modes {
//         }
//     }
// }

// Different method:
// Call Rust function with trigger
// Return keys to press
// or run command (from Rust?)
// Somehow: switch mode inside Rust

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

#[repr(C)]
pub struct KeyActionResult<'a> {
    pub action: Option<CString>,
    pub kind: ActionKind,
    pub in_mode: Option<&'a [HeadphoneButton]>,
}

impl<'a> KeyActionResult<'a> {
    fn new(kind: ActionKind) -> Self {
        KeyActionResult {
            action: None,
            kind: kind,
            in_mode: None,
        }
    }

    fn with_action(mut self, action: &str) -> Self {
        let action = CString::new(action.clone()).unwrap();
        self.action = Some(action);
        self
    }

    fn in_mode(mut self, mode: &'a [HeadphoneButton]) -> Self {
        self.in_mode = Some(mode);
        self
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CKeyActionResult {
    pub action: *const c_char,
    pub kind: *const ActionKind,
    pub in_mode: *const Trigger,
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
    let sample_maps = "map <up> k
map <down> j
map <play><down> works!
mode <play><up> {
    map <down> hello
}
";

    // Figure out how to persist this without re-parsing
    // let map_group = MapGroup::parse(sample_maps).unwrap();
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

            // match map_group.get(trigger) {
            //     Some(map_action) => {
            //         Some(KeyActionResult {
            //             action: map_action.action,
            //             kind: MapKind::Map,
            //         })
            //     },
            //     None => {
            //         // TODO: Figure out how to error
            //         None
            //     },
            // }
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

// fn run_command(command: Action) -> Result {
// }

#[no_mangle]
pub extern "C" fn c_parse_args(
    args: *const *const c_char,
    length: size_t,
) -> *const Config {
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

    let config = config::parse_args(&args);

    Box::into_raw(Box::new(config))
}

#[no_mangle]
pub extern "C" fn config_free(ptr: *mut Config) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}


mod tests {
    use super::*;

}
