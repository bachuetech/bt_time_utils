use bt_time_utils::{get_current_time_and_date, get_formatted_time_and_date};

#[test]
fn test_default_format(){
    assert_eq!(get_current_time_and_date(),("13:00".to_string(),"February 14, 2025".to_string()));
}

#[test]
fn test_formatted(){
    assert_eq!(get_formatted_time_and_date("%H:%M","%B %d, %Y"),get_current_time_and_date());
}