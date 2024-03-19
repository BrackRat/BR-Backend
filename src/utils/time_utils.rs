// use chrono::{DateTime};

// pub fn time_format(timestamp: i64) -> String {
//     let dt = DateTime::from_timestamp(timestamp, 0).map(|dt| dt.with_timezone(&chrono::Local));
//     let formatted_time = dt.expect("Format time failed").format("%Y-%m-%d %H:%M:%S").to_string();
//     formatted_time
// }

pub fn get_local_timestamp() -> i64 {
    chrono::Local::now().timestamp()
}