
#[cfg(test)]
mod tests {

    use bt_time_utils::{get_current_time_and_date, get_formatted_date, get_formatted_time_and_date};
    use chrono::NaiveDateTime;

    /*#[test]
    fn test_default_format(){
        assert_eq!(get_current_time_and_date(),("18:52".to_string(),"July 08, 2025".to_string()));
    }*/

    #[test]
    fn test_formatted(){
        assert_eq!(get_formatted_time_and_date("%H:%M","%B %d, %Y"),get_current_time_and_date());
    }

    #[test]
    fn test_get_formatted_date_format() {
        let format = "%Y-%m-%d %H:%M:%S";
        let result = get_formatted_date(format);
        
        // Try to parse the result using the same format
        let parsed = NaiveDateTime::parse_from_str(&result, format);
        assert!(parsed.is_ok(), "Resulting string did not match expected format");
    }

    #[test]
    fn test_get_formatted_date_not_empty() {
        let result = get_formatted_date("%Y");
        assert!(!result.is_empty(), "Resulting date string should not be empty");
    }
}

//convert
#[cfg(test)]
mod local_to_utc_tests {
    use bt_time_utils::parse_local_to_utc;

    #[test]
    fn converts_new_york_to_utc() {
        let input = "2026-04-18T13:41";
        let tz = "America/New_York";

        let result = parse_local_to_utc(input, tz).unwrap();

        // EDT is UTC-4 → 13:41 → 17:41 UTC
        assert_eq!(result.to_string(), "2026-04-18 17:41:00.0");
    }

    #[test]
    fn converts_los_angeles_to_utc() {
        let input = "2026-04-18T13:41";
        let tz = "America/Los_Angeles";

        let result = parse_local_to_utc(input, tz).unwrap();

        // PDT is UTC-7 → 13:41 → 20:41 UTC
        assert_eq!(result.to_string(), "2026-04-18 20:41:00.0");
    }

    #[test]
    fn converts_utc_identity() {
        let input = "2026-04-18T13:41";
        let tz = "UTC";

        let result = parse_local_to_utc(input, tz).unwrap();

        assert_eq!(result.to_string(), "2026-04-18 13:41:00.0");
    }

    #[test]
    fn handles_winter_standard_time() {
        let input = "2026-01-15T12:00";
        let tz = "America/New_York";

        let result = parse_local_to_utc(input, tz).unwrap();

        // EST is UTC-5 → 12:00 → 17:00 UTC
        assert_eq!(result.to_string(), "2026-01-15 17:00:00.0");
    }

    #[test]
    fn invalid_datetime_fails() {
        let input = "not-a-date";
        let tz = "America/New_York";

        let result = parse_local_to_utc(input, tz);

        assert!(result.is_err());
    }

    #[test]
    fn invalid_timezone_fails() {
        let input = "2026-04-18T13:41";
        let tz = "Mars/Phobos";

        let result = parse_local_to_utc(input, tz);

        assert!(result.is_err());
    }

    #[test]
    fn dst_spring_forward_invalid_time() {
        // This time does not exist in NY (skipped during DST jump)
        let input = "2026-03-08T02:30";
        let tz = "America/New_York";

        let result = parse_local_to_utc(input, tz);

        assert!(result.is_err());
    }

    #[test]
    fn dst_fall_back_ambiguous_time() {
        // This time occurs twice
        let input = "2026-11-01T01:30";
        let tz = "America/New_York";

        let result = parse_local_to_utc(input, tz);

        // assume_timezone() should error on ambiguity
        assert!(result.is_err());
    }
}


#[cfg(test)]
mod timezone_convert_tests {
    use bt_time_utils::{format_in_iana_timezone_or_utc, format_in_utcoffset_timezone};
    use time::{macros::format_description, PrimitiveDateTime, UtcOffset};

    #[test]
    fn test_format_est() {
        // 2026-02-23 18:30 UTC → 13:30 EST
        let dt = PrimitiveDateTime::parse(
            "2026-02-23 18:30",
            &format_description!("[year]-[month]-[day] [hour]:[minute]")
        ).unwrap();

        let est = UtcOffset::from_hms(-5, 0, 0).unwrap();

        let formatted = format_in_utcoffset_timezone(dt, est);
        assert_eq!(formatted, "2026-Feb-23 01:30 PM");
    }

    #[test]
    fn test_format_pst() {
        // 2026-02-23 08:15 UTC → 00:15 PST
        let dt = PrimitiveDateTime::parse(
            "2026-02-23 08:15",
            &format_description!("[year]-[month]-[day] [hour]:[minute]")
        ).unwrap();

        let pst = UtcOffset::from_hms(-8, 0, 0).unwrap();

        let formatted = format_in_utcoffset_timezone(dt, pst);
        assert_eq!(formatted, "2026-Feb-23 12:15 AM");
    }

    #[test]
    fn test_midnight_rollover() {
        // 2026-02-23 01:30 UTC → previous day in PST
        let dt = PrimitiveDateTime::parse(
            "2026-02-23 01:30",
            &format_description!("[year]-[month]-[day] [hour]:[minute]")
        ).unwrap();

        let pst = UtcOffset::from_hms(-8, 0, 0).unwrap();

        let formatted = format_in_utcoffset_timezone(dt, pst);
        assert_eq!(formatted, "2026-Feb-22 05:30 PM");
    }

    #[test]
    fn test_new_york_conversion() {
        // 2026-02-23 18:30 UTC → 13:30 EST
        let dt = PrimitiveDateTime::parse(
            "2026-02-23 18:30",
            &format_description!("[year]-[month]-[day] [hour]:[minute]")
        ).unwrap();

        let formatted = format_in_iana_timezone_or_utc(dt, "America/New_York");
        assert_eq!(formatted, "2026-Feb-23 01:30 PM");
    }

    #[test]
    fn test_los_angeles_conversion() {
        // 2026-02-23 18:30 UTC → 10:30 PST
        let dt = PrimitiveDateTime::parse(
            "2026-02-23 18:30",
            &format_description!("[year]-[month]-[day] [hour]:[minute]")
        ).unwrap();

        let formatted = format_in_iana_timezone_or_utc(dt, "America/Los_Angeles");
        assert_eq!(formatted, "2026-Feb-23 10:30 AM");
    }

    #[test]
    fn test_midnight_rollover_iana() {
        // 2026-02-23 01:30 UTC → 17:30 PST (previous day)
        let dt = PrimitiveDateTime::parse(
            "2026-02-23 01:30",
            &format_description!("[year]-[month]-[day] [hour]:[minute]")
        ).unwrap();

        let formatted = format_in_iana_timezone_or_utc(dt, "America/Los_Angeles");
        assert_eq!(formatted, "2026-Feb-22 05:30 PM");
    }

    #[test]
    fn test_invalid_timezone_falls_back_to_utc() {
        let dt = PrimitiveDateTime::parse(
            "2026-02-23 18:30",
            &format_description!("[year]-[month]-[day] [hour]:[minute]")
        ).unwrap();

        let formatted = format_in_iana_timezone_or_utc(dt, "INVALID/TZ");
        assert_eq!(formatted, "2026-Feb-23 06:30 PM"); // UTC fallback
    }

    #[test]
    fn test_dst_transition() {
        // DST starts in US on 2026-03-08 at 02:00 → clocks jump to 03:00
        // 2026-03-08 07:30 UTC → 03:30 EDT (UTC -4 after 2AM)
        let dt = PrimitiveDateTime::parse(
            "2026-03-08 07:30",
            &format_description!("[year]-[month]-[day] [hour]:[minute]")
        ).unwrap();

        let formatted = format_in_iana_timezone_or_utc(dt, "America/New_York");
        assert_eq!(formatted, "2026-Mar-08 03:30 AM");
    }

}