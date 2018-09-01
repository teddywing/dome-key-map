use std::ffi::{CStr, CString};
use std::ptr;
use std::slice;

// use cocoa::base::nil;
// use cocoa::foundation::{NSArray, NSAutoreleasePool, NSDictionary};
use libc::{c_char, size_t};

use {HeadphoneButton, MapGroup, MapKind};
use parser;

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
pub struct Trigger {
    pub buttons: *const HeadphoneButton,
    pub length: size_t,
}

#[repr(C)]
pub struct KeyActionResult<'a> {
    pub action: Option<CString>,
    pub kind: MapKind,
    pub in_mode: Option<&'a [HeadphoneButton]>,
}

impl<'a> KeyActionResult<'a> {
    fn new(kind: MapKind) -> Self {
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
pub struct CKeyActionResult {
    pub action: *const c_char,
    pub kind: *const MapKind,
}

#[no_mangle]
pub extern "C" fn c_run_key_action(
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
            assert!(!(*mode).buttons.is_null());

            Some(
                slice::from_raw_parts((*mode).buttons, (*mode).length as usize)
            )
        }
    };

    let result = match run_key_action_for_mode(trigger, mode) {
        Some(k) => {
            match k.action {
                Some(a) => {
                    CKeyActionResult {
                        action: a.into_raw(), // memory leak, must be freed from Rust
                        kind: &k.kind,
                    }
                },
                None => {
                    CKeyActionResult {
                        action: ptr::null(),
                        kind: &k.kind,
                    }
                },
            }
        },
        None => {
            CKeyActionResult {
                action: ptr::null(),
                kind: ptr::null(),
            }
        }
    };

    &result as *const CKeyActionResult
}

#[no_mangle]
pub extern "C" fn run_key_action_for_mode<'a>(
    trigger: &'a [HeadphoneButton],
    in_mode: Option<&[HeadphoneButton]>
) -> Option<KeyActionResult<'a>> {
    let sample_maps = "map <up> k
map <down> j
map <play><down> works!
";

    // Figure out how to persist this without re-parsing
    let map_group = MapGroup::parse(sample_maps).unwrap();

    let map = map_group.maps.get(trigger);
    let mode = map_group.modes.get(trigger);

    if let Some(in_mode) = in_mode {
        if let Some(mode) = map_group.modes.get(in_mode) {
            if let Some(map) = mode.get(trigger) {
                return match map.kind {
                    MapKind::Map => {
                        Some(
                            KeyActionResult::new(MapKind::Map)
                                .with_action(&map.action)
                                .in_mode(trigger)
                        )
                    },
                    MapKind::Command => {
                        Some(
                            KeyActionResult::new(MapKind::Command)
                                .in_mode(trigger)
                        )
                    },
                }
            }
        }
    }

    if let Some(map) = map {
        return match map.kind {
            MapKind::Map => {
                // let action_bytes = map.action;
                // let x = action_bytes.as_bytes();
                // let action = CStr::from_bytes_with_nul(x).unwrap();
                let action = CString::new(map.action.clone()).unwrap();

                Some(
                    KeyActionResult::new(MapKind::Map)
                        .with_action(&map.action)
                )
            },
            MapKind::Command => {
                Some(
                    KeyActionResult::new(MapKind::Command)
                )
            },
        }
    }

    if let Some(mode) = mode {
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
