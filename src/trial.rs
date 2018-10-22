use chrono::{DateTime, FixedOffset, Local, TimeZone};
use magic_crypt::{self, MagicCrypt};

use errors::{DateCryptError, DurationError};

// Start timestamp on October 1 at 23h
// Trial should be valid until November 1 00h


const DAYS_REMAINING: u8 = 30;
const KEY: &'static str = "TODO SECRET";

fn days_remaining(
    start: DateTime<Local>,
    now: DateTime<Local>,
    days_available: u8,
) -> Result<u8, DurationError> {
    let duration = (now.date() - start.date()).num_days() as u8;

    if duration > days_available {
        Err(
            DurationError::NegativeDuration(days_available as i32 - duration as i32)
        )
    } else {
        Ok(days_available - duration)
    }
}

fn days_remaining_from_now(start: DateTime<Local>) -> Result<u8, DurationError> {
    days_remaining(start, Local::now(), DAYS_REMAINING)
}

fn encode_datetime(d: DateTime<Local>) -> String {
    let iv = initialization_vector();

    let mut mc = MagicCrypt::new(KEY, magic_crypt::SecureBit::Bit64, Some(&iv));

    let timestamp = mc.encrypt_str_to_base64(&d.to_rfc3339());

    format!("{}//{}", timestamp, iv)
}

fn decode_datetime(s: &str) -> Result<DateTime<FixedOffset>, DateCryptError> {
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
            Local.ymd(2018, 10, 1).and_hms(23, 1, 0),
            Local.ymd(2018, 10, 1).and_hms(23, 30, 0),
            30,
        );

        assert_eq!(remaining, Ok(30));
    }

    #[test]
    fn days_remaining_with_middle_date() {
        let remaining = days_remaining(
            Local.ymd(2018, 10, 1).and_hms(23, 1, 0),
            Local.ymd(2018, 10, 22).and_hms(15, 0, 0),
            30,
        );

        assert_eq!(remaining, Ok(9));
    }

    #[test]
    fn days_remaining_on_last_day_is_0() {
        let remaining = days_remaining(
            Local.ymd(2018, 10, 1).and_hms(23, 1, 0),
            Local.ymd(2018, 10, 31).and_hms(23, 30, 0),
            30,
        );

        assert_eq!(remaining, Ok(0));
    }

    #[test]
    fn days_remaining_on_day_following_last_day_is_negative_duration_error() {
        let remaining = days_remaining(
            Local.ymd(2018, 10, 1).and_hms(23, 1, 0),
            Local.ymd(2018, 11, 1).and_hms(0, 0, 0),
            30,
        );

        assert_eq!(remaining, Err(DurationError::NegativeDuration(-1)));
    }

    #[test]
    fn days_remaining_after_last_day_is_negative_duration_error() {
        let remaining = days_remaining(
            Local.ymd(2018, 10, 1).and_hms(23, 1, 0),
            Local.ymd(2018, 11, 5).and_hms(0, 0, 0),
            30,
        );

        assert_eq!(remaining, Err(DurationError::NegativeDuration(-5)));
    }
}
