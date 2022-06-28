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

fn get_previous_days_until_this_weekday(
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
