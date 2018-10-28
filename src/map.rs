use std::env;
use std::ffi::OsString;
use std::process::Command;

use {Action, HeadphoneButton, MapAction, MapKind};
use ffi::State;
use sounds;

pub fn run_key_action_for_mode<'a>(
    state: &mut State,
    trigger: &'a [HeadphoneButton],
) {
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

            if mode.is_some() {
                state.in_mode = Some(trigger.to_vec());
            }
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
