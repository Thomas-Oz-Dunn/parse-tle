/*
Parser for TLE
*/

pub struct TLE {
    name: String,
    catalog_number: u32,
    international_designator: String,
    epoch_year: u32,
    epoch_month: u32,
    epoch_day: u32,
    epoch_hours: u32,  
    epoch_min: u32,
    epoch_sec: u32,
    mean_motion_1: f64,
    mean_motion_2: f64,
    radiation_pressure: f64,
    inc: f64,
    raan: f64,
    eccentricity:  f64,
    arg_perigee: f64,
    mean_anomaly: f64,    
    mean_motion: f64,
    rev_num: f64
}

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
pub fn parse(
    tle_str: String
) -> TLE {
    let lines: Vec<&str> = tle_str.lines().collect();
    // name
    let name: &str = lines[0];
    let bind1: String = lines[1].to_string();
    let line1: Vec<&str> = bind1
        .split_whitespace()
        .collect();
    
    // catalog_number
    let catalog_number: &str = line1[2];
    
    // TODO-TD: international_designator
    let epoch_str: &str = line1[3];

    let year_endian: u32 = epoch_str[..=1]
        .to_string()
        .parse::<u32>()
        .unwrap();

    // epoch_year
    let epoch_year: u32;
    if year_endian < 57{
        epoch_year = 2000 + year_endian;
    } else {
        epoch_year = 1900 + year_endian;
    }

    let binding: String = epoch_str[2..]
        .to_string();
    let epoch_day_full: Vec<&str> = binding
        .split_terminator('.')
        .collect();
    let day_of_year: u32 = epoch_day_full[0]
        .to_string()
        .parse::<u32>()
        .unwrap();

    let md: (u32, u32) = calc_month_day(day_of_year, epoch_year);
    
    // epoch_month
    let epoch_month: u32 = md.0;

    // epoch_day
    let epoch_day: u32 = md.1;

    let percent_of_day: f64 = 
    (".".to_owned() + epoch_day_full[1])
        .parse::<f64>()
        .unwrap();

    let hours_dec: f64 = percent_of_day * 24.0;
    // epoch_hours
    let hours_whole: u32 = hours_dec.div_euclid(24.0).floor() as u32;
    let hours_part: f64 = hours_dec.rem_euclid(24.0);
    let minutes_dec: f64 = hours_part * 60.;

    // epoch_min
    let minutes_whole: u32 = minutes_dec.div_euclid(60.).floor() as u32;
    let minutes_part: f64 = minutes_dec.rem_euclid(60.);
    let seconds_dec: f64 = minutes_part * 60.;

    // epoch_sec
    let seconds_whole: u32 = seconds_dec.div_euclid(60.).floor() as u32;
 
    
    // mean_motion_1
    let mean_motion_1: f64 = line1[4]
        .to_string()
        .parse::<f64>()
        .unwrap();

    // mean_motion_2
    let mean_motion_2: f64 = line1[5]
        .to_string()
        .parse::<f64>()
        .unwrap();

    // TODO-TD: radiation_pressure
    
    let binding: String = lines[2].to_string();
    let line2: Vec<&str> = binding.split_whitespace().collect();
    
    // --- Angles
    // inc
    let inc: f64 = line2[2]
        .to_string()
        .parse::<f64>()
        .unwrap();

    // raan
    let raan: f64 = line2[3]
        .to_string()
        .parse::<f64>()
        .unwrap();

    // eccentricity
    let eccentricity: f64 =
        (".".to_owned() + line2[4])
        .parse::<f64>()
        .unwrap();

    // arg_perigee
    let arg_perigee: f64 = line2[5]
        .to_string()
        .parse::<f64>()
        .unwrap();

    // mean_anomaly
    let mean_anomaly: f64 = line2[6]
        .to_string()
        .parse::<f64>()
        .unwrap();

    let end_str: &str = line2[line2.len()-1];
    
    // mean_motion
    let mean_motion: f64 = end_str[..11]
        .to_string()
        .parse::<f64>()
        .unwrap();

    // rev_num

    return TLE { 
        name: name.to_string(),
        catalog_number: (),
        international_designator: (),
        epoch_year: epoch_year,
        epoch_month: epoch_month,
        epoch_day: epoch_day,
        epoch_hours: hours_whole,
        epoch_min: minutes_whole,
        epoch_sec: seconds_whole,
        mean_motion_1: mean_motion_1,
        mean_motion_2: mean_motion_2,
        radiation_pressure: (),
        inc: inc,
        raan: raan,
        eccentricity:  eccentricity,
        arg_perigee: arg_perigee,
        mean_anomaly: mean_anomaly,
        mean_motion: mean_motion,
        rev_num: ()
    }

}


/// Convert day of year, year to month, day
/// 
/// Inputs
/// ------
/// day_of_year: `u32`
///     Day of year (1-365)
/// 
/// year: `u32`
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
    year: u32
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
/// year: `u32`
///     Gregorian Year of common era.
/// 
/// Outputs
/// -------
/// is_leap_year: `bool`
///     Boolean determining if year is a leap year
fn check_if_leap_year(year: u32) -> bool {
    let rule1: bool = year % 4 == 0;
    let rule2: bool = year % 100 != 0;
    let rule3: bool = year % 400 == 0;
    let is_leap_year: bool = rule1 && (rule2 || rule3);
    return is_leap_year
}


#[cfg(test)]
mod tle_tests {
    use super::*;

    #[test]
    fn test_calc_month_day(){
        let year: u32 = 2023;
        let day_of_year: u32 = 78;
        let md = calc_month_day(day_of_year, year);
        assert_eq!(md.0, 3);
        assert_eq!(md.1, 3);

    }
}
