use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::result;

use chrono::{DateTime, FixedOffset, Local, TimeZone};
use exitcode;
use magic_crypt::{self, MagicCrypt};
use xdg;

use errors::*;

// Start timestamp on October 1 at 23h
// Trial should be valid until November 1 00h


const DAYS_REMAINING: u8 = 30;
const KEY: &'static str = "TODO SECRET";

/// Entry point to the trial handler.
fn do_trial() {
    // Try to read trial start from file
    let date = match get_trial_start() {
        Ok(date) => date,
        Err(e) => {
            match e.kind() {
                ErrorKind::Io(e) if e.kind() == io::ErrorKind::NotFound =>
                    match initialize_trial_start() {
                        Ok(date) => date,
                        Err(e) => {
                            eprintln!("{}", e);
                            ::std::process::exit(exitcode::IOERR);
                        },
                    },
                DurationError => return trial_expired(),
                e => {
                    eprintln!("{}", e);
                    ::std::process::exit(exitcode::SOFTWARE);
                },
            }
        }
    };
    // If the file doesn't exist, create it
    // Print the number of days remaining
    // If no more days remain, exit the program with error EX_NOPERM

    match days_remaining_from_now(date) {
        Ok(remaining) => print_trial_days(remaining),
        DurationError => trial_expired(),
        Err(e) => (),
    }
}

fn trial_expired() {
    println!("Your trial has expired");

    ::std::process::exit(exitcode::NOPERM)
}

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

fn datetime_local_to_fixed_offset(d: DateTime<Local>) -> DateTime<FixedOffset> {
    let offset = FixedOffset::from_offset(d.offset());
    DateTime::<FixedOffset>::from_utc(d.naive_local(), offset)
}

/// Decrypts the time string from the trial file and returns it as a
/// `DateTime`.
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

fn print_trial_days(days: u8) {
    // TODO: Come up with a label
    println!("{}", days);
}

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

fn days_remaining_from_now(
    start: DateTime<FixedOffset>
) -> result::Result<u8, DurationError> {
    days_remaining(
        start,
        datetime_local_to_fixed_offset(Local::now()),
        DAYS_REMAINING
    )
}

fn encode_datetime(d: DateTime<FixedOffset>) -> String {
    let iv = initialization_vector();

    let mut mc = MagicCrypt::new(KEY, magic_crypt::SecureBit::Bit64, Some(&iv));

    let timestamp = mc.encrypt_str_to_base64(&d.to_rfc3339());

    format!("{}//{}", timestamp, iv)
}

fn decode_datetime(
    s: &str
) -> result::Result<DateTime<FixedOffset>, DateCryptError> {
    let encrypted: Vec<_> = s.rsplitn(2, "//").collect();
    let timestamp = encrypted[0];
    let iv = encrypted[1];

    let mut mc = MagicCrypt::new(KEY, magic_crypt::SecureBit::Bit64, Some(&iv));

    let timestamp = mc.decrypt_base64_to_string(&timestamp)?;
    let timestamp = DateTime::parse_from_rfc3339(&timestamp)?;

    Ok(timestamp)
}

fn initialization_vector() -> String {
    // Multiplied by 2 for no good reason other than to make the value
    // different from the actual timestamp.
    (Local::now().timestamp_millis() * 2).to_string()
}

// initialize_trial_start
// encode_datetime
// check_days_remaining
// days_remaining_from_now


#[cfg(test)]
mod tests {
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
