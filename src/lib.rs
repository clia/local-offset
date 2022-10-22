//! Get current local timezone offset simplely.

pub mod error;

use time::UtcOffset;
use tz::TimeZone;

/// Get current local timezone offset
pub fn current_local_offset() -> Result<UtcOffset, error::LocalOffsetError> {
    // Get local time zone (UNIX only)
    let time_zone_local = TimeZone::local()?;

    // Get the current local time type
    let current_local_time_type = time_zone_local.find_current_local_time_type()?;

    let diff_secs = current_local_time_type.ut_offset();

    let offset_sec = UtcOffset::from_hms(
        (diff_secs / 3_600) as _,
        ((diff_secs / 60) % 60) as _,
        (diff_secs % 60) as _,
    )?
    // .expect("Can not get local offset!")
    .whole_seconds();

    UtcOffset::from_whole_seconds(offset_sec).map_err(|e| e.into())
    //.expect("Can not from whole seconds!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = current_local_offset();
        println!("{:?}", result);
    }
}
