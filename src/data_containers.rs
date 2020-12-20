use chrono::NaiveDate;
use std::collections::HashMap;

type DayData = Vec<i32>;
type MonthData = HashMap<i32, DayData>;
type YearData = HashMap<i32, MonthData>;
type SingleStatData = HashMap<i32, YearData>;
pub type MultiStatData = HashMap<String, SingleStatData>;

fn num_days_in_month(y: i32, m: u32) -> u32 {
    if m == 12 {
        NaiveDate::from_ymd(y + 1, 1, 1)
    } else {
        NaiveDate::from_ymd(y, m + 1, 1)
    }.signed_duration_since(NaiveDate::from_ymd(y, m, 1))
    .num_days() as u32
}

fn init_month_data(y: i32, m: u32) -> MonthData {
    (1..=num_days_in_month(y, m))
    .map(|d| {
        (d as i32, vec![])
    }).collect::<HashMap<_, _>>()
}

fn init_year_data(y: i32) -> YearData {
    (1..=12)
    .map(|m| {
        (m as i32, init_month_data(y as i32, m as u32))
    }).collect::<HashMap<_,  _>>()
}

pub fn init_single_stat_data(start_year: i32, end_year: i32) -> SingleStatData{
    (start_year..=end_year)
    .map(|y| {
        (y as i32, init_year_data(y as i32))
    }).collect::<HashMap<_,  _>>()
}