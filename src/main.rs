/*
Executable for TLE interefacing

*/
use std::fs;
use parse_tle::tle::*;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CLI {
    /// Two line element directly in cli
    #[arg(short, long)]
    two_line_element: Option<String>,
    
    
    /// path to .txt file holding tle information
    #[arg(short, long)]
    file_path: Option<String>,
    
    // flag w/ file path to .txt on local machine 
    // flag for celestrak mode query with key desired
    
}

fn main() {
    let cli: CLI = CLI::parse();
    let tle_string: Option<String> = cli.two_line_element;
    let file_path: Option<String> = cli.file_path;
     
    if tle_string.is_some(){
        let tle: TLE = parse(tle_string.unwrap().as_str());
        print!("{}", tle);
    }
    else if file_path.is_some()  
    {
        let contents = fs::read_to_string(file_path.unwrap().as_str()).expect("Should have been able to read the file");

        let tle: TLE = parse(&contents.as_str());
        print!("{}", tle);
    }
    else
    {

        println!("No tle provided, running with demo values!!");
        
        let tle_str: &str = 
        "ISS (ZARYA)
        1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
        2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";
        
        let tle: TLE = parse(tle_str);
        print!("{}", tle);
    }

}
