use std::ffi::{CStr, CString};
use std::fs;
use std::mem;
use std::ptr;
use std::slice;

use autopilot::key::type_string;
// use cocoa::base::nil;
// use cocoa::foundation::{NSArray, NSAutoreleasePool, NSDictionary};
use libc::{c_char, size_t};
use xdg;

use {HeadphoneButton, MapGroup, MapKind};

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
    let xdg_dirs = xdg::BaseDirectories::with_prefix("dome-key").unwrap();
    let mapping_file = xdg_dirs.find_config_file("mappings.dkmap")
        .expect(
            &format!(
                "No mapping file found at '{}{}'",
                xdg_dirs.get_config_home()
                    .to_str()
                    .expect("Config home path contains invalid unicode"),
                "mappings.dkmap"
            )
        );

    let state = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };

    let dkmap = fs::read_to_string(mapping_file)
        .expect("Failed to read 'mappings.dkmap'");
    state.map_group = Some(
        MapGroup::parse(&dkmap)
            .expect("Failed to parse 'mappings.dkmap'")
    );
}

#[no_mangle]
pub extern "C" fn c_run_key_action(
    state: *mut State,
    trigger: Trigger,
    mode: *const Trigger,
) -> *const CKeyActionResult {
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

    let result = run_key_action_for_mode(&mut state, trigger, mode);
    let result = match result {
        Some(k) => {
            let action = k.action.map_or_else(
                || ptr::null(),
                |a| a.into_raw(),
            );
            // let in_mode = k.in_mode.map_or_else(
            //     || ptr::null(),
            //     |m| {
            //         let trigger = Trigger {
            //             buttons: m.as_ptr(),
            //             length: m.len(),
            //         };
            //         mem::forget(m);
            //
            //         &trigger
            //     },
            // );
            let trigger;
            let in_mode = if let Some(m) = k.in_mode {
                let boink = Trigger {
                    buttons: m.as_ptr(),
                    length: m.len(),
                };

                trigger = Box::into_raw(Box::new(boink)); // TODO: memory leak
                trigger
            } else {
                ptr::null()
            };
            // mem::forget(k.in_mode);
            // mem::forget(in_mode);
            // println!("IN MODE: {:?}", &in_mode);
            // let in_mode2 = Box::new(k.in_mode);
            // let in_mode_ptr = Box::into_raw(in_mode2);

            let result = CKeyActionResult {
                action: action, // memory leak, must be freed from Rust
                kind: &k.kind,
                in_mode: in_mode,
            };
            println!("CKeyActionResult(161): {:?}", result);
            // mem::forget(result);
            result
        },
        None => {
            CKeyActionResult {
                action: ptr::null(),
                kind: ptr::null(),
                in_mode: ptr::null(),
            }
        }
    };
    // println!("hey result: {:?}", result);
    // mem::forget(result);
    println!("Result 177: {:?}", result);
    let r = Box::new(result);
    let r2 = Box::into_raw(r);
    println!("r2: {:?}", r2);

    // &result as *const CKeyActionResult
    r2 as *const CKeyActionResult
}

#[no_mangle]
pub extern "C" fn run_key_action_for_mode<'a>(
    state: &mut State,
    trigger: &'a [HeadphoneButton],
    in_mode: Option<&[HeadphoneButton]>
) -> Option<KeyActionResult<'a>> {
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

                        return Some(KeyActionResult::new(ActionKind::Mode))
                    }

                    if let Some(map) = mode.get(trigger) {
                        return match map.kind {
                            MapKind::Map => {
                                type_string(&map.action, &[], 0.0, 0.0);

                                Some(
                                    KeyActionResult::new(ActionKind::Map)
                                        .with_action(&map.action)
                                        .in_mode(trigger)
                                )
                            },
                            MapKind::Command => {
                                Some(
                                    KeyActionResult::new(ActionKind::Command)
                                        .in_mode(trigger)
                                )
                            },
                        }
                    }
                }
            }

            // TODO: make sure this doesn't run when in_mode
            if state.in_mode.is_none() {
                if let Some(map) = map {
                    return match map.kind {
                        MapKind::Map => {
                            type_string(&map.action, &[], 0.0, 0.0);

                            Some(
                                KeyActionResult::new(ActionKind::Map)
                                    .with_action(&map.action)
                            )
                        },
                        MapKind::Command => {
                            Some(
                                KeyActionResult::new(ActionKind::Command)
                            )
                        },
                        // MapKind::Mode => {
                            // TODO: Maybe make a new type just for KeyActionResult that
                            // combines regular MapKinds and Mode
                        // },
                    }
                }
            }

            if let Some(mode) = mode {
                state.in_mode = Some(trigger.to_vec());

                return Some(
                    KeyActionResult::new(ActionKind::Mode)
                        .in_mode(trigger)
                )
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

            None
        },
        None => None,
    }
}

// fn run_command(command: Action) -> Result {
// }


mod tests {
    use super::*;

    #[test]
    fn parse_mappings_makes_cocoa_mappings() {
        parse_mappings();
    }
}
