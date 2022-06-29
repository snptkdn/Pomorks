use anyhow::Result;
use chrono::{prelude::*, Duration};

pub fn get_this_week(date: Date<Local>) -> Result<Vec<Date<Local>>> {
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
