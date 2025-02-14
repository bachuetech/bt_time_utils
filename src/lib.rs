use chrono::Local;

///Return Time using default format H:M and Date using default format B (Full Month) d, Y (YYYY)
pub fn get_current_time_and_date() -> (String, String) {
    //Default Format.
    get_formatted_time_and_date("%H:%M", "%B %d, %Y")
}

pub fn get_formatted_time_and_date(time_format: &str, date_format: &str) -> (String, String) {
    // Get the current local date and time
    let now = Local::now();

    // Format the time as HH:mm
    let time = now.format(&time_format).to_string();

    // Format the date as MMMM DD, YYYY
    let date = now.format(&date_format).to_string();

    (time, date)
}
