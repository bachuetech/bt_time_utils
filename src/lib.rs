//pub mod date_time; Not working!

use bt_logger::get_error;
use chrono::Local;
use time::macros::format_description;
use time::{OffsetDateTime, PrimitiveDateTime, UtcOffset};
use time::format_description::well_known::Iso8601;
use time_tz::{OffsetDateTimeExt, PrimitiveDateTimeExt, Tz, timezones};

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

pub fn get_formatted_date(date_format: &str) -> String {
    // Get the current local date and time
    Local::now().format(&date_format).to_string()
}


/// Parses a local date-time string with an IANA timezone and converts it to UTC.
///
/// This function takes a **naive local datetime string** (no offset) and a
/// **timezone name** (e.g., `"America/New_York"`), then:
///
/// 1. Parses the string into a [`PrimitiveDateTime`]
/// 2. Interprets it as local time in the given timezone
/// 3. Converts it to UTC
/// 4. Returns the result as a UTC [`PrimitiveDateTime`]
///
/// # Arguments
///
/// * `datetime_str` - A date-time string in ISO 8601 format (e.g., `"2026-04-18T13:41"`).
///   - Must not include a timezone or offset.
/// * `tz_name` - An IANA timezone identifier (e.g., `"America/New_York"`).
///
/// # Returns
///
/// * `Ok(PrimitiveDateTime)` - The corresponding UTC date-time
/// * `Err(...)` - If parsing fails, timezone is invalid, or the local time is ambiguous/invalid
///
/// # Errors
///
/// This function returns an error if:
///
/// * The input string is not valid ISO 8601
/// * The timezone string is not a valid IANA identifier
/// * The local time does not exist (e.g., during DST spring-forward)
/// * The local time is ambiguous (e.g., during DST fall-back)
///
/// # Example
///
/// ```rust
/// # use time::PrimitiveDateTime;
/// # use bt_time_utils::parse_local_to_utc;
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let utc = parse_local_to_utc("2026-04-18T13:41", "America/New_York")?;
///
/// println!("{utc}");
/// # Ok(())
/// # }
/// ```
///
/// # DST Behavior
///
/// This function uses timezone rules (including daylight saving time).
///
/// For ambiguous times (e.g., `"2026-11-01T01:30"` in `"America/New_York"`),
/// it returns an error rather than guessing.
///
/// # Notes
///
/// * The input is assumed to be **local time in the provided timezone**
/// * The output is a **UTC datetime without offset information**
///
pub fn parse_local_to_utc(
    datetime_str: &str,
    tz_name: &str,
) -> Result<PrimitiveDateTime, Box<dyn std::error::Error>> {
    // Parse naive datetime (no timezone)
    let naive = PrimitiveDateTime::parse(datetime_str, &Iso8601::PARSING)?;

    // Parse timezone string (e.g., "America/New_York")
    let tz: &Tz = timezones::get_by_name(tz_name).ok_or_else(|| get_error!("parse_local_to_utc","Unknown timezone: {}", tz_name))?;
    
    let zoned = match naive.assume_timezone(tz){
        time_tz::OffsetResult::Some(z) => z,
        time_tz::OffsetResult::Ambiguous(z0, z1) => return Err(get_error!("parse_local_to_utc", "Ambigous time {}. May get {} or {}.",datetime_str, z0, z1 ).into()),
        time_tz::OffsetResult::None => return Err(get_error!("parse_local_to_utc", "cannot parse string to PrimitiveDateTime with TimeZone").into()),
    };
    
    // Convert to UTC
    let utc = zoned.to_offset(UtcOffset::UTC);
 
    // Return as PrimitiveDateTime (UTC)
    Ok(PrimitiveDateTime::new(utc.date(), utc.time()))
}

///Format from UTC to to String date_time at UtcOffset
pub fn format_in_utcoffset_timezone(
    utc_dt: PrimitiveDateTime,
    tz_offset: UtcOffset,
) -> String {
    // Attach UTC offset to the naive datetime
    let utc_dt: OffsetDateTime = utc_dt.assume_utc();

    // Convert to the target timezone
    let local_dt = utc_dt.to_offset(tz_offset);

    // Format: YYYY-MMM-DD HH:MM AM/PM
    let format = format_description!("[year]-[month repr:short]-[day] [hour repr:12]:[minute] [period]");

    local_dt.format(format).unwrap()
}

///Format from UTC to String date_time at Iana timezone or default to UTC
pub fn format_in_iana_timezone_or_utc(
    utc_datetime: PrimitiveDateTime,
    tz_name: &str,
) -> String {
    // Convert naive UTC datetime → OffsetDateTime
    let utc_dt  = utc_datetime.assume_utc();

    // Try to load the IANA timezone; fallback to UTC
    let tz = timezones::get_by_name(tz_name)
        .unwrap_or(timezones::db::UTC);

    // Convert UTC → local time
    let local_dt = utc_dt.to_timezone(tz);

    // Format: YYYY-MMM-DD HH:MM AM/PM
    let format = format_description!(
        "[year]-[month repr:short]-[day] [hour repr:12]:[minute] [period]"
    );

    local_dt.format(&format).unwrap()
}