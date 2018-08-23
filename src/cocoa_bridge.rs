use cocoa::base::nil;
use cocoa::foundation::{NSArray, NSAutoreleasePool, NSDictionary};

use MapGroup;

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
            let t = NSArray::array(nil).autorelease();
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


mod tests {
    use super::*;

    #[test]
    fn parse_mappings_makes_cocoa_mappings() {
        parse_mappings();
    }
}
