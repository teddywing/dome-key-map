use chrono::{DateTime, Local, TimeZone};

use errors::DurationError;

// Start timestamp on October 1 at 23h
// Trial should be valid until November 1 00h


const DAYS_REMAINING: u8 = 30;

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
