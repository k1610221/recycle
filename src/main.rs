#![allow(unused)]
use chrono::{*};
mod the_calendar;
use crate::the_calendar::Calendar;

fn main() {

    /* Declare variables */
    let url = "https://www.city.chofu.tokyo.jp/www/contents/1646875289947/simple/text20222.txt";
    let current: DateTime<Local> = Local::now();
    let noon: NaiveTime = NaiveTime::from_hms_opt(12, 0, 0).unwrap();

    /* Determine a target */
    let target: NaiveDate = if current.time() < noon {
        current.date_naive()
    } else {
        current.date_naive().checked_add_days(Days::new(1)).unwrap()
    };

    let (target_month, target_date) = {
        let binding = target.to_string();
        (binding.get(5..7).unwrap().parse().unwrap(), binding.get(8..10).unwrap().parse().unwrap())
    };

    /* Get the txt file from URL */
    let raw_data = reqwest::blocking::get(url).unwrap().text().unwrap(); 

    /* Parse the txt file */
    let calendar = Calendar::new(raw_data);

    /* Process */
    calendar.get(target_month, target_date);

}
