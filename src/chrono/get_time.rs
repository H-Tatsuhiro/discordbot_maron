use chrono::Local;

pub fn get_time() -> String {
    let utc_datetime = Local::now();
    let format_datetime = utc_datetime.format("%Y%m%dT%H%M%S+0900").to_string();
    format_datetime
}
