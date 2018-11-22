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

use std::env;
use std::ffi::OsString;
use std::process::Command;

use {Action, HeadphoneButton, MapAction, MapKind};
use errors::*;
use ffi::State;

#[repr(C)]
pub enum ModeChange {
    Activated,
    Deactivated,
}

pub fn run_key_action<'a>(
    state: &mut State,
    trigger: &'a [HeadphoneButton],
    on_mode_change: extern "C" fn(mode_change: ModeChange),
) -> Result<()> {
    match state.map_group {
        Some(ref map_group) => {
            let map = map_group.maps.get(trigger);
            let mode = map_group.modes.get(trigger);

            if let Some(in_mode) = state.in_mode.clone() {
                if let Some(mode) = map_group.modes.get(&in_mode) {
                    // Deactivate mode by pressing current mode trigger
                    if &in_mode[..] == trigger {
                        state.in_mode = None;

                        on_mode_change(ModeChange::Deactivated);

                        return Ok(());
                    }

                    if let Some(map) = mode.get(trigger) {
                        run_action(&map)?;
                    }
                }
            }

            if state.in_mode.is_none() {
                if let Some(map) = map {
                    run_action(&map)?;
                }
            }

            if mode.is_some() {
                state.in_mode = Some(trigger.to_vec());

                on_mode_change(ModeChange::Activated);
            }
        },
        None => (),
    };

    Ok(())
}

fn run_action(map_action: &MapAction) -> Result<()> {
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

                return Command::new(shell)
                    .arg("-c")
                    .arg(action)
                    .spawn()
                    .map(|_| ())
                    .chain_err(|| "command failed to start");
            }
        },
    };

    Ok(())
}
