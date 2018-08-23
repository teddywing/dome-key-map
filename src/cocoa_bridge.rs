extern crate cocoa;

use MapGroup;

pub extern "C" fn x() {
    let sample_maps = "map <up> k
map <down> j";

    let map_group = MapGroup::parse(sample_maps);

    unsafe {
    }
}
