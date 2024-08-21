/*
Parser for TLE
*/
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::{BufWriter, Read, Write};

use hifitime::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TLE {
    pub name: String,
    pub catalog_number: String,
    pub classification: String,
    pub international_designator: String,
    pub epoch: Epoch,
    pub mean_motion_1: f64,
    pub mean_motion_2: f64,
    pub radiation_pressure: f64,
    pub ephemeris_type: u8,
    pub element_set_number: u64,
    pub inc: f64,
    pub raan: f64,
    pub eccentricity: f64,
    pub arg_perigee: f64,
    pub mean_anomaly: f64,
    pub mean_motion: f64,
    pub rev_num: u32,
}

/// From method for `TLE` struct
impl From<&str> for TLE {
    /// From
    fn from(tle_str: &str) -> TLE {
        return parse(tle_str);
    }
}

/// Write TLE struct to JSON formatted file
///
/// Inputs
/// ------
/// tle: `TLE`
///     tle struct
///
/// path_str: `&String`
///     Path to write to
///
pub fn write_json(tle: &TLE, path_str: &String) {
    let file: File = File::create(path_str).unwrap();
    let mut writer: BufWriter<File> = BufWriter::new(file);
    serde_json::to_writer(&mut writer, tle).unwrap();
    writer.flush().unwrap();
}

/// Read TLE struct from JSON formatted file
///
/// Inputs
/// ------
/// json_str: `&str`
///     File containing json data
///
/// Outputs
/// -------
/// tle_values: `TLE`
pub fn read_json(json_path: &str) -> TLE {
    let mut file: File =
        File::open(json_path).expect(format!("{json_path} could not be openned").as_str());

    let mut data: String = String::new();
    file.read_to_string(&mut data)
        .expect(format!("{json_path} could not be read").as_str());

    let tle_values: TLE = serde_json::from_str(&data).expect("JSON was not well-formatted");
    return tle_values;
}

/// Display method for `TLE` struct
impl Display for TLE {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        write!(
            formatter,
            "{}\nCatalog #: {}\nClassification: {}\nIntl Desig: {}\nSet #: {}\nEpoch: {}\nMean Motion: {}\nMean Motion prime: {}\nMean Motion prime 2: {}\nRadiation Pressure: {}\nInclination: {}\nRaan: {}\nEccentricity: {}\nArgument of Perigee: {}\nMean Anomaly: {}\nRevolution #: {}",
            self.name,
            self.catalog_number,
            self.classification,
            self.international_designator,
            self.element_set_number,
            self.epoch,
            self.mean_motion,
            self.mean_motion_1,
            self.mean_motion_2,
            self.radiation_pressure,
            self.inc,
            self.raan,
            self.eccentricity,
            self.arg_perigee,
            self.mean_anomaly,
            self.rev_num
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
/// tle: `TLE`
///     TLE struct
pub fn parse(tle_str: &str) -> TLE {
    let lines: Vec<&str> = tle_str.lines().collect();
    let n_lines: usize = lines.len();

    let (idx_1, idx_2) = match n_lines {
        3 => (1, 2),
        2 => (0, 1),
        _ => panic!("Invalid number of lines"),
    };

    let line_1: String = lines[idx_1].trim().to_string();
    validate_checksum(&line_1);

    let catalog_number: &str = &line_1[2..=6];

    let classification: &str = &line_1[7..=7];

    let intnl_desig: &str = &line_1[9..=16];

    // name
    let name: &str;
    if lines.len() == 3 {
        name = lines[0];
    } else {
        name = intnl_desig
    }

    let epoch_str: &str = &line_1[18..=31];

    let year_endian: i32 = epoch_str[0..=1]
        .to_string()
        .parse::<i32>()
        .expect("Unable to parse year_endian value at epoch_str[0..=1]");

    // epoch_year
    let epoch_year: i32;
    if year_endian < 57 {
        epoch_year = 2000 + year_endian;
    } else {
        epoch_year = 1900 + year_endian;
    }

    let binding: String = epoch_str[2..].to_string();
    let epoch_day_full: Vec<&str> = binding.split_terminator('.').collect();
    let day_of_year: u32 = epoch_day_full[0]
        .to_string()
        .parse::<u32>()
        .expect("Unable to parse day_of_year value at epoch_day_full[0]");

    let month_day: (u8, u8) = calc_month_day(day_of_year, epoch_year as u32);

    let percent_of_day: f64 = (".".to_owned() + epoch_day_full[1])
        .parse::<f64>()
        .expect("Unable to parse percent_of_day value at epoch_day_full[1]");

    let hours_dec: f64 = percent_of_day * 24.0;

    // epoch_hours
    let hours_whole: u8 = hours_dec.div_euclid(24.0).floor() as u8;
    let hours_part: f64 = hours_dec.rem_euclid(24.0);
    let minutes_dec: f64 = hours_part * 60.;

    // epoch_min
    let minutes_whole: u8 = minutes_dec.div_euclid(60.).floor() as u8;
    let minutes_part: f64 = minutes_dec.rem_euclid(60.);
    let seconds_dec: f64 = minutes_part * 60.;

    // epoch_sec
    let seconds_whole: u8 = seconds_dec.div_euclid(60.).floor() as u8;

    // hifitime epoch
    let full_epoch: Epoch = Epoch::from_gregorian_hms(
        epoch_year as i32,
        month_day.0,
        month_day.1,
        hours_whole,
        minutes_whole,
        seconds_whole,
        TimeScale::UTC,
    );

    // mean_motion_1
    let mean_motion_1_sign: f64 = (line_1[33..=33].to_string() + "1")
        .trim()
        .parse::<f64>()
        .expect("Unable to parse mean_motion_1_sign value at line_1[33..=33]");

    let mean_motion_1_base: f64 = line_1[34..=42]
        .to_string()
        .parse::<f64>()
        .expect("Unable to parse mean_motion_1_base value at line_1[34..=42]");
    let mean_motion_1: f64 = mean_motion_1_base * mean_motion_1_sign;

    // mean_motion_2
    let mean_mot_2_sign: f64 = (line_1[44..=44].to_string() + "1")
        .trim()
        .parse::<f64>()
        .unwrap();
    let mean_mot_2_base: f64 = line_1[45..=49].to_string().parse::<f64>().unwrap();
    let mean_mot_2_exp = line_1[50..=51].to_string().parse::<f64>().unwrap();
    let mean_mot_2_pow: f64 = 10_f64.powf(mean_mot_2_exp);
    let mean_motion_2: f64 = (mean_mot_2_sign * mean_mot_2_base) * mean_mot_2_pow;

    // radiation_pressure
    let rad_press_sign: f64 = (line_1[53..=53].to_string() + "1")
        .trim()
        .parse::<f64>()
        .unwrap();
    let rad_press_base: f64 = line_1[54..=58].to_string().parse::<f64>().unwrap();
    let rad_press_exp = line_1[59..=60].to_string().parse::<f64>().unwrap();
    let rad_press_pow: f64 = 10_f64.powf(rad_press_exp);
    let radiation_pressure: f64 = rad_press_sign * rad_press_base * rad_press_pow;

    let ephemeris_type: u8 = line_1[62..=62]
        .to_string()
        .parse::<u8>()
        .expect("Unable to parse ephemeris_type value at line_1[62..=62]");

    let element_set_number: u64 = line_1[64..=67]
        .to_string()
        .trim()
        .parse::<u64>()
        .expect("Unable to parse element_set_number value at line_1[64..=67]");

    let line2: String = lines[idx_2].trim().to_string();
    validate_checksum(&line2);

    // TODO-TD: turn all unwraps into expects
    // --- Angles
    // inc
    let inc: f64 = line2[8..=15].to_string().trim().parse::<f64>().unwrap();

    // raan
    let raan: f64 = line2[17..=24].to_string().trim().parse::<f64>().unwrap();

    // eccentricity
    let eccentricity: f64 = (".".to_owned() + &line2[26..=32]).parse::<f64>().unwrap();

    // arg_perigee
    let arg_perigee: f64 = line2[34..=41].to_string().trim().parse::<f64>().unwrap();

    // mean_anomaly
    let mean_anomaly: f64 = line2[44..=50].to_string().parse::<f64>().unwrap();

    // mean_motion
    let mean_motion: f64 = line2[52..=62].to_string().trim().parse::<f64>().unwrap();

    // rev_num
    let rev_num: u32 = line2[64..=68].to_string().trim().parse::<u32>().unwrap();

    let tle: TLE = TLE {
        name: name.trim().to_string(),
        catalog_number: catalog_number.trim().to_string(),
        classification: classification.trim().to_string(),
        international_designator: intnl_desig.trim().to_string(),
        epoch: full_epoch,
        mean_motion_1: mean_motion_1,
        mean_motion_2: mean_motion_2,
        radiation_pressure: radiation_pressure,
        ephemeris_type: ephemeris_type,
        element_set_number: element_set_number,
        inc: inc,
        raan: raan,
        eccentricity: eccentricity,
        arg_perigee: arg_perigee,
        mean_anomaly: mean_anomaly,
        mean_motion: mean_motion,
        rev_num: rev_num,
    };

    return tle;
}

/// Run checksum on TLE line
///   
/// Inputs
/// ------
/// line: `&String`
///     Line to checksum
pub fn validate_checksum(line: &String){
    let mut checksum: u32 = 0;
    for i_char in line.chars(){
        if i_char == '-'{
            checksum += 1;
        }
        else if i_char != ' ' && i_char.is_numeric(){
            checksum += i_char
                .to_string()
                .parse::<u32>()
                .expect(format!("Unable to parse {} as u32", i_char).as_str());
    
        }
    }
    let tle_checksum: u32 = line[68..=68]
        .to_string()
        .parse::<u32>()
        .expect("Unable to parse checksum value");

    // NOTE: Need to subtract the final due to iteration over entire line
    let mod_10: u32 = (checksum - tle_checksum) % 10;
    
    assert!(
        mod_10 == tle_checksum,  
        "calculated = {}, tle value = {}", 
        mod_10, 
        tle_checksum
    );
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
/// month: `u8`
///     Month (1-12)
///
/// day: `u8`
///     Day of month (1-31)
pub fn calc_month_day(day_of_year: u32, year: u32) -> (u8, u8) {
    assert!(day_of_year < 366, "Day of year must be less than 366");

    let feb_days: u32;
    if check_if_leap_year(year) {
        feb_days = 29;
    } else {
        feb_days = 28;
    }

    let month_lengths: Vec<u32> = vec![31, feb_days, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut month: u8 = 1;
    let mut sum_days: u32 = month_lengths[0];

    while sum_days < day_of_year - month_lengths[month as usize - 1] {
        month += 1;
        sum_days += month_lengths[month as usize - 1];
    }

    let month: u8 = month;
    let day: u32 = day_of_year - sum_days;

    return (month, day as u8);
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
    return is_leap_year;
}

/// Query celestrak.org api for TLE
pub fn query_celestrak(query: &str, value: &str, verbose: bool) -> TLE {
    // TODO-TD: add support for handling multiple results
    // TODO-TD: if query is CATNR, check digit count
    // TODO-TD: if query is GROUP, handle multi result
    let url: String = "https://celestrak.org/NORAD/elements/gp.php?".to_owned() + query + "=" + value;
    let mut response = reqwest::blocking::get(url).unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body)
        .expect("Unable to read request");

    if verbose {
        println!("\n Site Status: {}", response.status());
        println!("\n Site Headers:\n{:#?}", response.headers());
        println!("\n Site Body:\n{}", body);
    }

    return parse(&body.as_str())
}


#[cfg(test)]
mod tle_tests {
    use super::*;

    #[test]
    fn test_calc_month_day() {
        let year: u32 = 2023;
        let day_of_year: u32 = 78;
        let md = calc_month_day(day_of_year, year);

        assert_eq!(md.0, 2);
        assert_eq!(md.1, 19);
    }

    #[test]
    fn test_check_if_leap_year() {
        let test_year: u32 = 2022;
        let is_leap_year: bool = check_if_leap_year(test_year);
        assert_eq!(is_leap_year, false);
    }

    #[test]
    fn test_parser() {
        let sample_tle: &str = "CHANDRAYAAN-3
        1 57320U 23098A   23208.62000000  .00000392  00000+0  00000+0 0  9994
        2 57320  21.3360   6.1160 9054012 182.9630  18.4770  0.46841359   195";

        let chandrayaan_3: TLE = parse(sample_tle);

        assert_eq!(chandrayaan_3.name, "CHANDRAYAAN-3".to_string());

        assert_eq!(chandrayaan_3.inc, 21.3360);
    }
}
