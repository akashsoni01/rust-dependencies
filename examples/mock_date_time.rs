use chrono::{DateTime, Local, TimeZone};
use std::time::{Duration, UNIX_EPOCH};

trait MockDateTime {
    fn mock() -> DateTime<Local>;
}

impl MockDateTime for DateTime<Local> {
    fn mock() -> DateTime<Local> {
        let duration = Duration::from_secs(0);
        let system_time = UNIX_EPOCH + duration;
        DateTime::<Local>::from(system_time)
    }
}

fn main() {
    let mock_date = DateTime::<Local>::mock();
    println!("Mock date: {}", mock_date);
}
