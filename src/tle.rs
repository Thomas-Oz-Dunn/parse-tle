/*
Parser for TLE
*/

use std::convert::From;
use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)] 
pub struct TLE {
    name: String,
    catalog_number: String,
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
    rev_num: u32
}


impl From<&str> for TLE {
    fn from(tle_string: &str) -> TLE {
        return parse(tle_string);
    }
}


impl Display for TLE {

    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result { 
        write!(
            formatter, 
            "{}\nCatalog #: {}\nIntl Desig: {}\nEpoch: {}/{}/{} {}::{}::{}\n
            Mean Motion: {}\nMean Motion`: {}\nMean Motion``: {}\n
            Radiation Pressure: {}\ninclination: {}\nraan: {}\neccentricity: {}\n
            argument of perigee: {}\nmean anomaly: {}\nRevolution #: {}", 
            self.name, self.catalog_number, self.international_designator,
            self.epoch_year, self.epoch_month, self.epoch_day, self.epoch_hours,
            self.epoch_min, self.epoch_sec, self.mean_motion, self.mean_motion_1,
            self.mean_motion_2, self.radiation_pressure, self.inc, self.raan, 
            self.eccentricity, self.arg_perigee, self.mean_anomaly, self.rev_num
        )

    }
}


/// Parse standard Two Line Element
/// 
/// Inputs
/// ------
/// tle_str : `&str` 
///     NORAD Two Line Element Identification String
/// 
/// Outputs
/// -------
/// TLE
///     TLE struct
pub fn parse(
    tle_string: &str
) -> TLE {
    let lines: Vec<&str> = tle_string.lines().collect();
    
    // name
    let name: &str = lines[0];
    let bind1: String = lines[1].trim().to_string();
    
    // catalog_number
    let catalog_number: &str = &bind1[2..=7];
    
    let intnl_desig: &str = &bind1[9..=16];

    let epoch_str: &str = &bind1[18..=31];

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

    let month_day: (u32, u32) = calc_month_day(day_of_year, epoch_year);
    
    let epoch_month: u32 = month_day.0;
    let epoch_day: u32 = month_day.1;

    let percent_of_day: f64 = (".".to_owned() + epoch_day_full[1])
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
    let mean_motion_1_sign: f64 = (
        bind1[33..=33].to_string() +  "1").trim().parse::<f64>().unwrap();
    let mean_motion_1_base: f64 = bind1[34..=42]
        .to_string()
        .parse::<f64>()
        .unwrap();
    let mean_motion_1: f64 = mean_motion_1_base * mean_motion_1_sign;

    // mean_motion_2
    let mean_mot_2_sign: f64 = (
        bind1[44..=44].to_string() +  "1").trim().parse::<f64>().unwrap();
    let mean_mot_2_base: f64 = bind1[45..=49]
        .to_string()
        .parse::<f64>()
        .unwrap();
    let mean_mot_2_pow: f64 = 10_f64.powf((
        bind1[50..=51].to_string()).parse::<f64>().unwrap());
    let mean_motion_2: f64 = (mean_mot_2_sign * mean_mot_2_base) * mean_mot_2_pow;

    // radiation_pressure
    let rad_press_sign: f64 = (
        bind1[53..=53].to_string() +  "1").trim().parse::<f64>().unwrap();
    let rad_press_base: f64 = bind1[54..=58]
        .to_string()
        .parse::<f64>()
        .unwrap();
    let rad_press_pow: f64 = 10_f64.powf((
        bind1[59..=60].to_string()).parse::<f64>().unwrap());
    let radiation_pressure: f64 = rad_press_sign * rad_press_base * rad_press_pow;
    
    let bind2: String = lines[2].to_string();

    // --- Angles
    // inc
    let inc: f64 = bind2[8..=15].to_string().trim().parse::<f64>().unwrap(); 
    
    // raan
    let raan: f64 = bind2[17..=24].to_string().trim().parse::<f64>().unwrap();
    
    // eccentricity
    let eccentricity: f64 = (
        ".".to_owned() + &bind2[26..=32]
    ).parse::<f64>().unwrap();
    
    // arg_perigee
    let arg_perigee: f64 = bind2[34..=41]
        .to_string()
        .parse::<f64>()
        .unwrap();

    // mean_anomaly
    let mean_anomaly: f64 = bind2[44..=50]
        .to_string()
        .parse::<f64>()
        .unwrap();

    // mean_motion
    let mean_motion: f64 = bind2[52..=62]
        .to_string()
        .trim()
        .parse::<f64>()
        .unwrap();

    // rev_num
    let rev_num: u32 = bind2[64..=68]
        .to_string()
        .trim()
        .parse::<u32>()
        .unwrap();

    return TLE { 
        name: name.to_string(),
        catalog_number: catalog_number.to_string(),
        international_designator: intnl_desig.to_string(),
        epoch_year: epoch_year,
        epoch_month: epoch_month,
        epoch_day: epoch_day,
        epoch_hours: hours_whole,
        epoch_min: minutes_whole,
        epoch_sec: seconds_whole,
        mean_motion_1: mean_motion_1,
        mean_motion_2: mean_motion_2,
        radiation_pressure: radiation_pressure,
        inc: inc,
        raan: raan,
        eccentricity:  eccentricity,
        arg_perigee: arg_perigee,
        mean_anomaly: mean_anomaly,
        mean_motion: mean_motion,
        rev_num: rev_num
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
        
        assert_eq!(md.0, 2);
        assert_eq!(md.1, 19);

    }

    #[test]
    fn test_check_if_leap_year(){
        let test_year: u32 = 2022;
        let is_leap_year: bool = check_if_leap_year(test_year);
        assert_eq!(is_leap_year, false);

    }

    #[test]
    fn test_parser(){
        let sample_tle = 
"CHANDRAYAAN-3           
1 57320U 23098A   23208.62000000  .00000392  00000+0  00000+0 0  9994
2 57320  21.3360   6.1160 9054012 182.9630  18.4770  0.46841359   195";

        let chandrayaan_3 = parse(sample_tle);

        assert_eq!(chandrayaan_3.name, "CHANDRAYAAN-3".to_string());

        assert_eq!(chandrayaan_3.epoch_year, 2023);

    }
}
