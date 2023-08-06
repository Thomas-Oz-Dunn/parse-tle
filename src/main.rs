use parse_tle::tle::*;

fn main() {
    
    // Parse the command line for: 

    // Direct TLE string

    // flag w/ file path 

    // flag for celestrak mode query site with key desired

    let  tle_string: String = 
    "ISS (ZARYA)
    1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
    2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537".to_string();
    let tle: TLE = tle_string.into();
    print!("{}", tle);

}
