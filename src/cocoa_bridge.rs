use std::ffi::CString;

use cocoa::base::nil;
use cocoa::foundation::{NSArray, NSAutoreleasePool, NSDictionary};

use {HeadphoneButton, MapGroup, MapKind};

#[repr(C)]
struct renameMeMapGroup {
}

pub extern "C" fn parse_mappings() {
    let sample_maps = "map <up> k
map <down> j";

    let map_group = MapGroup::parse(sample_maps).unwrap();

    unsafe {
        let _pool = NSAutoreleasePool::new(nil);

        let maps = NSDictionary::init(nil).autorelease();
        let modes = NSDictionary::init(nil).autorelease();

        for (trigger, action) in map_group.maps {
            // let t = NSArray::arrayWithObjects(nil, &trigger).autorelease();

            // maps.
        }

        for (trigger, modes) in map_group.modes {
        }
    }
}

// Different method:
// Call Rust function with trigger
// Return keys to press
// or run command (from Rust?)
// Somehow: switch mode inside Rust

#[repr(C)]
pub struct KeyActionResult {
    pub action: Option<CString>,
    pub kind: MapKind,
}

pub extern "C" fn run_key_action(
    trigger: &[HeadphoneButton]
) -> Option<KeyActionResult> {
    let sample_maps = "map <up> k
map <down> j";

    // Figure out how to persist this without re-parsing
    let map_group = MapGroup::parse(sample_maps).unwrap();

    let map = map_group.maps.get(trigger);
    let mode = map_group.modes.get(trigger);

    if let Some(map) = map {
        return match map.kind {
            MapKind::Map => {
                // let action_bytes = map.action;
                // let x = action_bytes.as_bytes();
                // let action = CStr::from_bytes_with_nul(x).unwrap();
                let action = CString::new(map.action.clone()).unwrap();

                Some(KeyActionResult {
                    action: Some(action),
                    kind: MapKind::Map,
                })
            },
            MapKind::Command => {
                Some(KeyActionResult {
                    action: None,
                    kind: MapKind::Command,
                })
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
