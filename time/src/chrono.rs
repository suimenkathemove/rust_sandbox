use chrono::{Datelike, Days, NaiveDate, Utc};
use chrono_tz::Asia::Tokyo;

pub fn main() {
    let current_date = Utc::now().with_timezone(&Tokyo).date_naive();
    let from_date = NaiveDate::from_ymd_opt(current_date.year() - 1, current_date.month(), 1)
        .expect("Invalid date");
    let to_date = current_date
        .with_day(1)
        .expect("Invalid date")
        .checked_sub_days(Days::new(1))
        .expect("Invalid date");

    println!("from_date: {:?}", from_date);
    println!("to_date: {:?}", to_date);
}
