pub mod tasks;
pub mod users;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::str::FromStr;

/// Метод для получения текущей даты.
/// # Return
/// Возвращает текущую дату в формате NaiveDateTime
fn get_date() -> NaiveDateTime{
    let utc_date_time = chrono::Utc::now().to_string();
    let _naive = match NaiveDateTime::from_str(&utc_date_time) {
        Ok(date_time) => {return date_time},
        Err(_) => {
            return NaiveDateTime::new(
            NaiveDate::from_ymd(2000, 1, 1),
            NaiveTime::from_hms(0, 0, 0)
            )
        },
    };
}