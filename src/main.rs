use parse_tle::tle::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    /// Two line element directly in cli
    #[arg(short, long)]
    two_line_element: Option<String>,
    
    // Direct TLE string
    // flag w/ file path to .txt on local machine 
    // flag for celestrak mode query with key desired
    
}

fn main() {
    let cli = CLI::parse();
    let tle_string = &cli.two_line_element;
    // if none use
    
    let  tle_string: &str = 
    "ISS (ZARYA)
    1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
    2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";
    
    
    let tle: TLE = parse(tle_string);
    print!("{}", tle);

}
