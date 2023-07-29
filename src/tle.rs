use chrono::{DateTime, NaiveDateTime, NaiveDate, NaiveTime, Datelike, Timelike, Utc};

/// Parse standard Two Line Element
/// 
/// Inputs
/// ------
/// tle_str : `String` 
///     NORAD Two Line Element Identification String
/// 
/// Outputs
/// -------
/// 
pub fn from_tle(
    tle_str: String
) -> Vec<&str> {
    let lines: Vec<&str> = tle_str.lines().collect();
    
    let name: &str = lines[0];
    let bind1 = lines[1].to_string();
    let line1: Vec<&str> = bind1
        .split_whitespace()
        .collect();

    let epoch_str: &str = line1[3];
    let epoch_year: i32 = epoch_str[..=1]
        .to_string()
        .parse::<i32>()
        .unwrap();

    let year: i32;
    if epoch_year < 57{
        year = 2000 + epoch_year;
    } else {
        year = 1900 + epoch_year;
    }

    let binding = epoch_str[2..]
        .to_string();
    let epoch_day_full: Vec<&str> = binding
        .split_terminator('.')
        .collect();

    let day_of_year: u32 = epoch_day_full[0]
        .to_string()
        .parse::<u32>()
        .unwrap();

    let md: (u32, u32) = xyzt::calc_month_day(day_of_year, year);
    
    let percent_of_day: f64 = 
    (".".to_owned() + epoch_day_full[1])
        .parse::<f64>()
        .unwrap();

    let hours_dec: f64 = percent_of_day * 24.0;
    let hours_whole: u32 = hours_dec.div_euclid(24.0).floor() as u32;
    let hours_part: f64 = hours_dec.rem_euclid(24.0);
    
    let minutes_dec: f64 = hours_part * 60.;
    let minutes_whole: u32 = minutes_dec.div_euclid(60.).floor() as u32;
    let minutes_part: f64 = minutes_dec.rem_euclid(60.);

    let seconds_dec: f64 = minutes_part * 60.;
    let seconds_whole: u32 = seconds_dec.div_euclid(60.).floor() as u32;

    let date: NaiveDate = NaiveDate::from_ymd_opt(year, month, day).unwrap();
    let time: NaiveTime = NaiveTime::from_hms_opt(hour, min, sec).unwrap();

    let dt: NaiveDateTime = NaiveDateTime::new(date, time);
    let date_time: DateTime::<Utc> = DateTime::<Utc>::from_utc(dt, Utc); 

    // let mean_motion_prime: &str = line1[4];
    // let mean_motion_2: &str = line1[5];
    
    let binding: String = lines[2].to_string();
    let line2: Vec<&str> = binding.split_whitespace().collect();
    
    let inc: f64 = line2[2]
        .to_string()
        .parse::<f64>()
        .unwrap();

    let raan: f64 = line2[3]
        .to_string()
        .parse::<f64>()
        .unwrap();

    let ecc: f64 =
        (".".to_owned() + line2[4])
        .parse::<f64>()
        .unwrap();

    let arg_perigee: f64 = line2[5]
        .to_string()
        .parse::<f64>()
        .unwrap();

    let mean_anomaly: f64 = line2[6]
        .to_string()
        .parse::<f64>()
        .unwrap();

    let end_str: &str = line2[line2.len()-1];
    let mean_motion: f64 = end_str[..11]
        .to_string()
        .parse::<f64>()
        .unwrap();


}


/// Convert day of year, year to month, day
/// 
/// Inputs
/// ------
/// day_of_year: `u32`
///     Day of year (1-365)
/// 
/// year: `i32`
///     Year (e.g. 2020)
/// 
/// Outputs
/// -------
/// month: `u32`
///     Month (1-12)
/// 
/// day: `u32`
///     Day of month (1-31)
pub fn calc_month_day(
    day_of_year: u32,
    year: i32
) -> (u32, u32) {
    assert!(day_of_year < 366, "Day of year must be less than 366"); 

    let feb_days: u32;
    if check_if_leap_year(year){feb_days = 29;
    } else {feb_days = 28;}

    let month_lengths: Vec<u32> = vec![
        31, feb_days, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut month: u32 = 1;
    let mut sum_days: u32 = month_lengths[0];

    while sum_days < day_of_year -  month_lengths[month as usize - 1]{
        month += 1;
        sum_days += month_lengths[month as usize - 1];
    }

    let month: u32 = month;
    let day: u32 = day_of_year - sum_days;

    return (month, day);
}


/// Check if the year is a leap year
/// 
/// Inputs
/// ------
/// year: `i32`
///     Gregorian Year of common era.
fn check_if_leap_year(year: i32) -> bool {
    let rule1: bool = year % 4 == 0;
    let rule2: bool = year % 100 != 0;
    let rule3: bool = year % 400 == 0;
    return rule1 && (rule2 || rule3);
}
