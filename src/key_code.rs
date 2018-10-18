use cocoa::appkit::{
    NSEvent,
    NSEventModifierFlags,
    NSEventSubtype::NSScreenChangedEventType,
    NSEventType::NSSystemDefined,
};
use cocoa::base::{id, nil};
use cocoa::foundation::NSPoint;
use core_graphics::event::{
    CGEvent,
    CGEventFlags,
    // CGEventPost,
    CGEventRef,
    CGEventTapLocation,
    CGKeyCode,
    KeyCode,
};

// pub struct KeyCode {
// }
//
// impl KeyCode {
//     pub const VOLUME_UP: CGKeyCode = 0x48;
//     pub const VOLUME_DOWN: CGKeyCode = 0x49;
//     pub const MUTE: CGKeyCode = 0x4A;
// }
//
// impl From<KeyCode> for CGKeyCode {
//     fn from(code: KeyCode) -> CGKeyCode {
//         match code {
//             KeyCode::F1 => event::KeyCode::F1,
//         }
//     }
// }

#[link(name="dome_key_event_source_simulator", kind="static")]
extern "C" {
    pub fn dkess_press_key(key: i16, modifier_flags: CGEventFlags);
}

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

//   kVK_F13                       = 0x69,
//   kVK_F14                       = 0x6B,
//   kVK_F15                       = 0x71,
//   kVK_F16                       = 0x6A,
//   kVK_F17                       = 0x40,
//   kVK_F18                       = 0x4F,
//   kVK_F19                       = 0x50,
//   kVK_F20                       = 0x5A,
//
// enum {
//   kVK_Return                    = 0x24,
//   kVK_Tab                       = 0x30,
//   kVK_Space                     = 0x31,
//   kVK_Delete                    = 0x33,
//   kVK_Escape                    = 0x35,
//   kVK_Command                   = 0x37,
//   kVK_Shift                     = 0x38,
//   kVK_CapsLock                  = 0x39,
//   kVK_Option                    = 0x3A,
//   kVK_Control                   = 0x3B,
//   kVK_RightCommand              = 0x36,
//   kVK_RightShift                = 0x3C,
//   kVK_RightOption               = 0x3D,
//   kVK_RightControl              = 0x3E,
//   kVK_Function                  = 0x3F,
//   kVK_F17                       = 0x40,
//   kVK_VolumeUp                  = 0x48,
//   kVK_VolumeDown                = 0x49,
//   kVK_Mute                      = 0x4A,
//   kVK_F18                       = 0x4F,
//   kVK_F19                       = 0x50,
//   kVK_F20                       = 0x5A,
//   kVK_F5                        = 0x60,
//   kVK_F6                        = 0x61,
//   kVK_F7                        = 0x62,
//   kVK_F3                        = 0x63,
//   kVK_F8                        = 0x64,
//   kVK_F9                        = 0x65,
//   kVK_F11                       = 0x67,
//   kVK_F13                       = 0x69,
//   kVK_F16                       = 0x6A,
//   kVK_F14                       = 0x6B,
//   kVK_F10                       = 0x6D,
//   kVK_F12                       = 0x6F,
//   kVK_F15                       = 0x71,
//   kVK_Help                      = 0x72,
//   kVK_Home                      = 0x73,
//   kVK_PageUp                    = 0x74,
//   kVK_ForwardDelete             = 0x75,
//   kVK_F4                        = 0x76,
//   kVK_End                       = 0x77,
//   kVK_F2                        = 0x78,
//   kVK_PageDown                  = 0x79,
//   kVK_F1                        = 0x7A,
//   kVK_LeftArrow                 = 0x7B,
//   kVK_RightArrow                = 0x7C,
//   kVK_DownArrow                 = 0x7D,
//   kVK_UpArrow                   = 0x7E
// };


// #define NX_NOSPECIALKEY			0xFFFF
// #define NX_KEYTYPE_SOUND_UP		0
// #define NX_KEYTYPE_SOUND_DOWN		1
// #define NX_KEYTYPE_BRIGHTNESS_UP	2
// #define NX_KEYTYPE_BRIGHTNESS_DOWN	3
// #define NX_KEYTYPE_CAPS_LOCK		4
// #define NX_KEYTYPE_HELP			5
// #define NX_POWER_KEY			6
// #define	NX_KEYTYPE_MUTE			7
// #define NX_UP_ARROW_KEY			8
// #define NX_DOWN_ARROW_KEY		9
// #define NX_KEYTYPE_NUM_LOCK		10
//
// #define NX_KEYTYPE_CONTRAST_UP		11
// #define NX_KEYTYPE_CONTRAST_DOWN	12
// #define NX_KEYTYPE_LAUNCH_PANEL		13
// #define NX_KEYTYPE_EJECT		14
// #define NX_KEYTYPE_VIDMIRROR		15
//
// #define NX_KEYTYPE_PLAY			16
// #define NX_KEYTYPE_NEXT			17
// #define NX_KEYTYPE_PREVIOUS		18
// #define NX_KEYTYPE_FAST			19
// #define NX_KEYTYPE_REWIND		20
//
// #define NX_KEYTYPE_ILLUMINATION_UP	21
// #define NX_KEYTYPE_ILLUMINATION_DOWN	22
// #define NX_KEYTYPE_ILLUMINATION_TOGGLE	23


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
