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

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::result;

use chrono::{DateTime, FixedOffset, Local};
use exitcode;
use magic_crypt::{self, MagicCrypt};
use xdg;

use errors::*;

// Start timestamp on October 1 at 23h
// Trial should be valid until November 1 00h


const DAYS_REMAINING: u8 = 30;
const KEY: &'static str = "
DomeKey enables customisation of headphone buttons. The *dome-key*
program runs in the background listening to button events, executing
user-defined actions in response to these events.
";

/// Entry point to the trial handler. Initialises a trial file or reads a
/// timestamp from the existing one. If a trial is ongoing, print the number of
/// days remaining. If expired, exit the program. Print any errors encountered.
pub fn do_trial() {
    // Try to read trial start from file
    let date = match get_trial_start() {
        Ok(date) => date,
        Err(e) => {
            match e.kind() {
                ErrorKind::Io(e) if e.kind() == io::ErrorKind::NotFound =>
                    // Create the file if it doesn't exist
                    match initialize_trial_start() {
                        Ok(date) => date,
                        Err(e) => {
                            eprintln!("{}", e);
                            ::std::process::exit(exitcode::IOERR);
                        },
                    },
                ErrorKind::Duration(_) => return exit_trial_expired(),
                e => {
                    eprintln!("{}", e);
                    ::std::process::exit(exitcode::SOFTWARE);
                },
            }
        }
    };

    match days_remaining_from_now(date) {
        Ok(remaining) => print_trial_days(remaining),
        Err(e) => {
            match e {
                DurationError::NegativeDuration(_) => exit_trial_expired(),
            }
        },
    }
}

/// Print an "expired" message and exit with `exitcode::NOPERM`.
fn exit_trial_expired() {
    println!("Your trial has expired");

    ::std::process::exit(exitcode::NOPERM)
}

/// Create `~/.local/share/dome-key/.trial` if it doesn't exist and write a
/// timestamp to it.
fn initialize_trial_start() -> Result<DateTime<FixedOffset>> {
    let now = datetime_local_to_fixed_offset(Local::now());
    let encoded_time = encode_datetime(now);

    let xdg_dirs = xdg::BaseDirectories::with_prefix("dome-key")
        .chain_err(|| "failed to get XDG base directories")?;
    let trial_path = xdg_dirs.place_data_file(".trial")
        .chain_err(|| "failed to get trial file path")?;
    let mut trial_file = match OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(trial_path)
    {
        Ok(f) => Ok(f),
        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => return Ok(now),
        Err(e) => Err(e),
    }
        .chain_err(|| "failed to create trial file")?;

    write!(&mut trial_file, "{}", encoded_time)
        .chain_err(|| "failed to write to trial file")?;

    Ok(now)
}

/// Convert a `DateTime<Local>` into a `DateTime<FixedOffset>`.
fn datetime_local_to_fixed_offset(d: DateTime<Local>) -> DateTime<FixedOffset> {
    DateTime::<FixedOffset>::from_utc(d.naive_local(), FixedOffset::east(0))
}

/// Decrypt the time string from the trial file and return it as a `DateTime`.
fn get_trial_start() -> Result<DateTime<FixedOffset>> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("dome-key")
        .chain_err(|| "failed to get XDG base directories")?;
    let trial_path = xdg_dirs.place_data_file(".trial")
        .chain_err(|| "failed to get trial file path")?;
    let mut trial_file = File::open(trial_path)?;
    let mut encoded_time = String::new();
    trial_file.read_to_string(&mut encoded_time)?;

    let trial_start = decode_datetime(&encoded_time)?;

    Ok(trial_start)
}

/// Print `days` remaining in the trial.
fn print_trial_days(days: u8) {
    if days == 1 {
        println!("{} trial day remaining", days);
    } else {
        println!("{} trial days remaining", days);
    }
}

/// Strip times from the input dates and subtract `start` from `now`. Return
/// the resulting value or a `DurationError` if less than 0.
fn days_remaining(
    start: DateTime<FixedOffset>,
    now: DateTime<FixedOffset>,
    days_available: u8,
) -> result::Result<u8, DurationError> {
    let duration = (now.date() - start.date()).num_days() as u8;

    if duration > days_available {
        Err(
            DurationError::NegativeDuration(days_available as i32 - duration as i32)
        )
    } else {
        Ok(days_available - duration)
    }
}

/// Compare `start` with the current local time. Compare the result with
/// `DAYS_REMAINING`.
fn days_remaining_from_now(
    start: DateTime<FixedOffset>
) -> result::Result<u8, DurationError> {
    days_remaining(
        start,
        datetime_local_to_fixed_offset(Local::now()),
        DAYS_REMAINING
    )
}

/// Encrypt a date.
fn encode_datetime(d: DateTime<FixedOffset>) -> String {
    let iv = initialization_vector();

    let mut mc = MagicCrypt::new(KEY, magic_crypt::SecureBit::Bit64, Some(&iv));

    let timestamp = mc.encrypt_str_to_base64(&d.to_rfc3339());

    format!("{}//{}", timestamp, iv)
}

/// Decrypt a date.
fn decode_datetime(
    s: &str
) -> result::Result<DateTime<FixedOffset>, DateCryptError> {
    let encrypted: Vec<_> = s.rsplitn(2, "//").collect();
    let timestamp = encrypted[1];
    let iv = encrypted[0];

    let mut mc = MagicCrypt::new(KEY, magic_crypt::SecureBit::Bit64, Some(&iv));

    let timestamp = mc.decrypt_base64_to_string(&timestamp)?;
    let timestamp = DateTime::parse_from_rfc3339(&timestamp)?;

    Ok(timestamp)
}

/// Create an initialisation vector for use in encryption and decryption.
fn initialization_vector() -> String {
    // Multiplied by 2 for no good reason other than to make the value
    // different from the actual timestamp.
    (Local::now().timestamp_millis() * 2).to_string()
}


#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use super::*;

    #[test]
    fn days_remaining_counts_days_remaining_from_start_date() {
        let remaining = days_remaining(
            datetime_local_to_fixed_offset(
                Local.ymd(2018, 10, 1).and_hms(23, 1, 0)
            ),
            datetime_local_to_fixed_offset(
                Local.ymd(2018, 10, 1).and_hms(23, 30, 0)
            ),
            30,
        );

        assert_eq!(remaining, Ok(30));
    }

    #[test]
    fn days_remaining_with_middle_date() {
        let remaining = days_remaining(
            datetime_local_to_fixed_offset(
                Local.ymd(2018, 10, 1).and_hms(23, 1, 0)
            ),
            datetime_local_to_fixed_offset(
                Local.ymd(2018, 10, 22).and_hms(15, 0, 0)
            ),
            30,
        );

        assert_eq!(remaining, Ok(9));
    }

    #[test]
    fn days_remaining_on_last_day_is_0() {
        let remaining = days_remaining(
            datetime_local_to_fixed_offset(
                Local.ymd(2018, 10, 1).and_hms(23, 1, 0)
            ),
            datetime_local_to_fixed_offset(
                Local.ymd(2018, 10, 31).and_hms(23, 30, 0)
            ),
            30,
        );

        assert_eq!(remaining, Ok(0));
    }

    #[test]
    fn days_remaining_on_day_following_last_day_is_negative_duration_error() {
        let remaining = days_remaining(
            datetime_local_to_fixed_offset(
                Local.ymd(2018, 10, 1).and_hms(23, 1, 0)
            ),
            datetime_local_to_fixed_offset(
                Local.ymd(2018, 11, 1).and_hms(0, 0, 0)
            ),
            30,
        );

        assert_eq!(remaining, Err(DurationError::NegativeDuration(-1)));
    }

    #[test]
    fn days_remaining_after_last_day_is_negative_duration_error() {
        let remaining = days_remaining(
            datetime_local_to_fixed_offset(
                Local.ymd(2018, 10, 1).and_hms(23, 1, 0)
            ),
            datetime_local_to_fixed_offset(
                Local.ymd(2018, 11, 5).and_hms(0, 0, 0)
            ),
            30,
        );

        assert_eq!(remaining, Err(DurationError::NegativeDuration(-5)));
    }
}
