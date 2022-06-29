use std::collections::VecDeque;

use anyhow::Result;
use chrono::{prelude::*, Duration};
fn get_yesterday(date: Date<Local>) -> Result<Date<Local>> {
    let one_day = Duration::days(1);

    Ok(date - Duration::days(1))
}

fn get_nextday(date: Date<Local>) -> Result<Date<Local>> {
    let one_day = Duration::days(1);

    Ok(date + Duration::days(1))
}

pub fn get_this_week(date: Date<Local>) -> Result<Vec<Date<Local>>> {
    let weekday = date.weekday();
    let num_from_monday = date.weekday().num_days_from_monday() as i64;

    let duration_from_monday = Duration::days(num_from_monday);
    let monday = date - duration_from_monday;

    Ok(vec![
        monday,
        monday + Duration::days(1),
        monday + Duration::days(2),
        monday + Duration::days(3),
        monday + Duration::days(4),
        monday + Duration::days(5),
        monday + Duration::days(6),
    ])
}

pub fn get_this_month(date: Date<Local>) -> Result<Vec<Date<Local>>> {
    let first_date_of_this_month = Local.ymd(date.year(), date.month(), 1);

    let mut date_of_this_month: Vec<Date<Local>> = Vec::new();

    get_next_day_until_different_month(first_date_of_this_month, &mut date_of_this_month);

    Ok(date_of_this_month)
}

fn get_next_day_until_different_month(
    date: Date<Local>,
    vec: &mut Vec<Date<Local>>,
) -> Vec<Date<Local>> {
    vec.push(date);
    if date.month() == date.succ().month() {
        get_next_day_until_different_month(date.succ(), vec)
    } else {
        vec.clone()
    }
}

pub fn get_previous_days_until_this_weekday(
    end_weekday: Weekday,
    date: Date<Local>,
    mut vec: VecDeque<Date<Local>>,
) -> Result<VecDeque<Date<Local>>> {
    vec.push_front(date);

    if date.weekday() == end_weekday {
        return Ok(vec);
    }

    get_previous_days_until_this_weekday(end_weekday, get_yesterday(date)?, vec)
}

fn get_days_this_month_until_this_day(
    date: Date<Local>,
    mut vec: VecDeque<Date<Local>>,
) -> Result<VecDeque<Date<Local>>> {
    if date.month() == get_nextday(date)?.month() {
        vec.push_front(date);
        get_days_this_month_until_this_day(get_yesterday(date)?, vec)
    } else {
        Ok(vec)
    }
}

#[test]
fn test_get_yesterday() {
    use super::*;
    let today = Local.ymd(2022, 6, 28);
    let yesterday = get_yesterday(today).unwrap();

    assert_eq!(yesterday.year(), 2022);
    assert_eq!(yesterday.month(), 6);
    assert_eq!(yesterday.day(), 27);

    let today = Local.ymd(2022, 6, 1);
    let yesterday = get_yesterday(today).unwrap();

    assert_eq!(yesterday.year(), 2022);
    assert_eq!(yesterday.month(), 5);
    assert_eq!(yesterday.day(), 31);
}

#[test]
fn test_get_previous_days_until_this_weekday() {
    use super::*;
    let today = Local.ymd(2022, 6, 28);

    let one_week =
        get_previous_days_until_this_weekday(today.weekday().succ(), today, VecDeque::new())
            .unwrap();

    assert_eq!(one_week[0].day(), 22);
    assert_eq!(one_week[0].weekday(), Weekday::Wed);
    assert_eq!(one_week[1].day(), 23);
    assert_eq!(one_week[1].weekday(), Weekday::Thu);
    assert_eq!(one_week[2].day(), 24);
    assert_eq!(one_week[2].weekday(), Weekday::Fri);
    assert_eq!(one_week[3].day(), 25);
    assert_eq!(one_week[3].weekday(), Weekday::Sat);
    assert_eq!(one_week[4].day(), 26);
    assert_eq!(one_week[4].weekday(), Weekday::Sun);
    assert_eq!(one_week[5].day(), 27);
    assert_eq!(one_week[5].weekday(), Weekday::Mon);
    assert_eq!(one_week[6].day(), 28);
    assert_eq!(one_week[6].weekday(), Weekday::Tue);
}

#[test]
fn test_get_days_this_month_until_this_day() {
    use super::*;

    let today = Local.ymd(2022, 6, 3);

    let this_month = get_days_this_month_until_this_day(today, VecDeque::new()).unwrap();
    assert_eq!(this_month.len(), 3);
    assert_eq!((this_month[0].month(), this_month[0].day()), (6, 1));
    assert_eq!((this_month[1].month(), this_month[1].day()), (6, 2));
    assert_eq!((this_month[2].month(), this_month[2].day()), (6, 3));
}

#[test]
fn test_get_this_week() {
    use super::*;

    let today = Local.ymd(2022, 6, 3);

    let this_week = get_this_week(today).unwrap();

    assert_eq!(this_week.len(), 7);
    assert_eq!((this_week[0].month(), this_week[0].day()), (5, 30));
    assert_eq!((this_week[1].month(), this_week[1].day()), (5, 31));
    assert_eq!((this_week[2].month(), this_week[2].day()), (6, 1));
    assert_eq!((this_week[3].month(), this_week[3].day()), (6, 2));
    assert_eq!((this_week[4].month(), this_week[4].day()), (6, 3));
    assert_eq!((this_week[5].month(), this_week[5].day()), (6, 4));
    assert_eq!((this_week[6].month(), this_week[6].day()), (6, 5));
}

#[test]
fn test_get_this_month() {
    use super::*;

    let today = Local.ymd(2022, 6, 3);

    let this_month = get_this_month(today).unwrap();

    assert_eq!(this_month.len(), 30);
}
