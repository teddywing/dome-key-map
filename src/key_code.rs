use core_graphics::event::CGEventFlags;

#[link(name="dome_key_event_source_simulator", kind="static")]
extern "C" {
    pub fn dkess_press_key(key: i16, modifier_flags: CGEventFlags);
}

pub type NXKey = i16;

// /System/Library/Frameworks/IOKit.framework/Versions/A/Headers/hidsystem/ev_keymap.h
pub const NX_KEYTYPE_SOUND_UP: NXKey = 0;
pub const NX_KEYTYPE_SOUND_DOWN: NXKey = 1;
pub const NX_KEYTYPE_BRIGHTNESS_UP: NXKey = 2;
pub const NX_KEYTYPE_BRIGHTNESS_DOWN: NXKey = 3;
pub const NX_KEYTYPE_HELP: NXKey = 5;
pub const NX_POWER_KEY: NXKey = 6;
pub const NX_KEYTYPE_MUTE: NXKey = 7;
pub const NX_KEYTYPE_NUM_LOCK: NXKey = 10;

pub const NX_KEYTYPE_CONTRAST_UP: NXKey = 11;
pub const NX_KEYTYPE_CONTRAST_DOWN: NXKey = 12;
pub const NX_KEYTYPE_EJECT: NXKey = 14;
pub const NX_KEYTYPE_VIDMIRROR: NXKey = 15;

pub const NX_KEYTYPE_PLAY: NXKey = 16;
pub const NX_KEYTYPE_NEXT: NXKey = 17;
pub const NX_KEYTYPE_PREVIOUS: NXKey = 18;
pub const NX_KEYTYPE_FAST: NXKey = 19;
pub const NX_KEYTYPE_REWIND: NXKey = 20;

pub const NX_KEYTYPE_ILLUMINATION_UP: NXKey = 21;
pub const NX_KEYTYPE_ILLUMINATION_DOWN: NXKey = 22;
pub const NX_KEYTYPE_ILLUMINATION_TOGGLE: NXKey = 23;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn dkess_press_key_presses_play_key() {
        use core_graphics::event::CGEventFlags;

        unsafe {
            const NX_KEYTYPE_PLAY: i16 = 16;
            dkess_press_key(NX_KEYTYPE_PLAY, CGEventFlags::CGEventFlagNull);
        }
    }
}
