// Copyright (c) 2018 Teddy Wing
//
// This file is part of DomeKey.
//
// *Purchasing policy notice:* All users of the software are expected to
// purchase a license from Teddy Wing unless they have a good reason not to
// pay. Users who can't purchase a license may apply to receive one for free
// at inquiry@domekey.teddywing.com. Users are free to:
//
// * download, build, and modify the app;
// * share the modified source code;
// * share the purchased or custom-built binaries (with unmodified license
//   and contact info), provided that the purchasing policy is explained to
//   all potential users.
//
// This software is available under a modified version of the Open Community
// Indie Software License:
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose is hereby granted, subject to the following conditions:
//
// * all copies retain the above copyright notice, the above purchasing
//   policy notice and this permission notice unmodified;
//
// * all copies retain the name of the software (DomeKey), the name of the
//   author (Teddy Wing), and contact information (including, but not limited
//   to, inquiry@domekey.teddywing.com, and domekey.teddywing.com URLs)
//   unmodified;
//
// * no fee is charged for distribution of the software;
//
// * the best effort is made to explain the purchasing policy to all users of
//   the software.
//
// THE SOFTWARE IS PROVIDED "AS IS", AND THE AUTHOR AND COPYRIGHT HOLDERS
// DISCLAIM ALL WARRANTIES, EXPRESS OR IMPLIED, WITH REGARD TO THIS SOFTWARE,
// INCLUDING BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE. IN NO EVENT SHALL THE AUTHOR OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY
// DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA, OR PROFITS, WHETHER
// IN AN ACTION OF CONTRACT, NEGLIGENCE, OR OTHER TORTIOUS ACTION, ARISING
// OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

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
