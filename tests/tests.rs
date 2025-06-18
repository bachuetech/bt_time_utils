
#[cfg(test)]
mod tests {

    use bt_time_utils::{get_current_time_and_date, get_formatted_date, get_formatted_time_and_date};
    use chrono::NaiveDateTime;

    #[test]
    fn test_default_format(){
        assert_eq!(get_current_time_and_date(),("17:38".to_string(),"June 18, 2025".to_string()));
    }

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