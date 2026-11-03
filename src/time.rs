//! Civil-date helpers shared by the dictionary and writing statistics.
//!
//! Stored timestamps are Unix seconds (UTC). Display and statistics use the
//! phone's local timezone: callers pass the UTC offset (seconds east of UTC)
//! that applies **at that timestamp**, which is what makes DST correct. The
//! offset itself comes from [`crate::platform::timezone`] on Android and from
//! the desktop clock in the preview.
//!
//! The conversions are Howard Hinnant's public-domain algorithms, so no date
//! crate is needed for the calendar maths.

pub const MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// Local civil date and time for a Unix timestamp.
pub fn local_date_time(unix_seconds: u64, offset_seconds: i32) -> (i32, u32, u32, u32, u32) {
    let local = unix_seconds as i64 + offset_seconds as i64;
    let days = local.div_euclid(86_400);
    let seconds = local.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = (seconds / 3_600) as u32;
    let minute = ((seconds % 3_600) / 60) as u32;
    (year, month, day, hour, minute)
}

/// Local day number since 1970-01-01. Used for streaks and grouping.
pub fn local_days(unix_seconds: u64, offset_seconds: i32) -> i64 {
    (unix_seconds as i64 + offset_seconds as i64).div_euclid(86_400)
}

/// `12 May 2026` in the local timezone.
pub fn format_date(unix_seconds: u64, offset_seconds: i32) -> String {
    let (year, month, day, _, _) = local_date_time(unix_seconds, offset_seconds);
    format!("{day} {} {year}", MONTHS[(month - 1) as usize])
}

/// `12 May 2026, 14:05` in the local timezone.
pub fn format_date_time(unix_seconds: u64, offset_seconds: i32) -> String {
    let (year, month, day, hour, minute) = local_date_time(unix_seconds, offset_seconds);
    format!(
        "{day} {} {year}, {hour:02}:{minute:02}",
        MONTHS[(month - 1) as usize]
    )
}

/// `May 2026`.
pub fn format_month(year: i32, month: u32) -> String {
    format!("{} {year}", MONTHS[(month - 1) as usize])
}

/// Days since 1970-01-01 to a civil date. Valid for the proleptic Gregorian
/// calendar.
pub fn civil_from_days(days: i64) -> (i32, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    ((if month <= 2 { y + 1 } else { y }) as i32, month, day)
}

/// Civil date to days since 1970-01-01. Used by tests and the date helpers.
pub fn days_from_civil(year: i32, month: u32, day: u32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year } as i64;
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = if month > 2 { month - 3 } else { month + 9 } as i64;
    let doy = (153 * mp + 2) / 5 + day as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Unix seconds for a local wall-clock time at a fixed UTC offset.
    fn local_at(year: i32, month: u32, day: u32, hour: u32, minute: u32, offset: i32) -> u64 {
        (days_from_civil(year, month, day) * 86_400 + hour as i64 * 3_600 + minute as i64 * 60
            - offset as i64) as u64
    }

    #[test]
    fn civil_date_conversion_round_trips() {
        for (year, month, day) in [
            (1970, 1, 1),
            (2024, 2, 29),
            (2025, 12, 31),
            (2026, 1, 1),
            (2026, 5, 12),
            (2100, 3, 1),
        ] {
            assert_eq!(
                civil_from_days(days_from_civil(year, month, day)),
                (year, month, day)
            );
        }
    }

    #[test]
    fn local_time_near_midnight_moves_to_the_next_day() {
        let utc = local_at(2026, 5, 12, 23, 30, 0);
        assert_eq!(format_date(utc, 3_600), "13 May 2026");
        assert_eq!(format_date_time(utc, 3_600), "13 May 2026, 00:30");
    }

    #[test]
    fn local_time_near_midnight_moves_to_the_previous_day() {
        let utc = local_at(2026, 1, 1, 0, 30, 0);
        assert_eq!(format_date(utc, -18_000), "31 Dec 2025");
        assert_eq!(format_date_time(utc, -18_000), "31 Dec 2025, 19:30");
    }

    #[test]
    fn local_time_handles_month_and_year_boundaries() {
        let utc = local_at(2026, 2, 1, 0, 30, 0);
        assert_eq!(format_date(utc, -18_000), "31 Jan 2026");

        let utc = local_at(2026, 1, 1, 0, 30, 0);
        assert_eq!(format_date(utc, -3_600), "31 Dec 2025");
    }

    #[test]
    fn local_days_uses_the_supplied_offset() {
        let utc = local_at(2026, 5, 12, 23, 30, 0);
        assert_eq!(local_days(utc, 3_600), days_from_civil(2026, 5, 13));
        assert_eq!(local_days(utc, 0), days_from_civil(2026, 5, 12));
    }

    #[test]
    fn month_labels_are_stable() {
        assert_eq!(format_month(2026, 5), "May 2026");
        assert_eq!(format_month(2025, 12), "Dec 2025");
    }
}
