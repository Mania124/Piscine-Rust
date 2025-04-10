use chrono::{Datelike, NaiveDate, Weekday as wd};

pub fn middle_day(year: i32) -> Option<wd> {
    // Check if it's a leap year (even number of days -> 366)
    let is_leap = NaiveDate::from_ymd_opt(year, 2, 29).is_some();

    if is_leap {
        None
    } else {
        // Non-leap year has 365 days → middle day is the 183rd (index 182)
        let middle = NaiveDate::from_ymd_opt(year, 7, 2)?; // July 2nd is the 183rd day
        Some(middle.weekday())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = middle_day(1022).unwrap();
        // assert_eq!(result, true);
        assert_eq!(wd::Tue, middle_day(2019).unwrap());
    }
}
