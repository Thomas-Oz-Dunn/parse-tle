/*
Executable for TLE interefacing

*/
use std::fs;
use clap::{Parser, Subcommand, Args};
use error_chain::error_chain;
use std::io::Read;

use parse_tle::tle::*;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

// TODO-TD: test positional vs keyword arguments 

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {
    /// Two line element directly in cli
    two_line_element: Option<String>,
    
    
    /// path to .txt file holding tle information
    file_path: Option<String>,
    
    /// flag for celestrak mode query with key desired
    #[command(subcommand)]
    command: Option<Commands>,
    
}

#[derive(Subcommand)]
enum Commands {
    /// Query celestrak for tles
    CelesTrak(CelestrakArgs),
}

#[derive(Args)]
struct CelestrakArgs {



    /// Query type
    /// ---
    /// CATNR: Catalog Number (1 to 9 digits). Allows return of data for a single catalog number.
    /// 
    /// INTDES: International Designator (yyyy-nnn). Allows return of data for all objects associated with a particular launch.
    /// 
    /// GROUP: Groups of satellites provided on the CelesTrak Current Data page.
    /// 
    /// NAME: Satellite Name. Allows searching for satellites by parts of their name.
    /// 
    /// SPECIAL: Special data sets for the GEO Protected Zone (GPZ) or GPZ Plus.
    /// 
    #[arg(short, long)]
    query: String,

    /// Object name
    #[arg(short, long)]
    name: String,

}


fn main() {
    let cli: CLI = CLI::parse();
    let tle_string: Option<String> = cli.two_line_element;
    let file_path: Option<String> = cli.file_path;
    let command = cli.command;
     
    if tle_string.is_some(){
        let tle: TLE = parse(tle_string.unwrap().as_str());
        println!("{}", tle);
    }
    else if file_path.is_some()  
    {
        let contents = fs::read_to_string(
            file_path.unwrap().as_str())
        .expect("Should have been able to read the file");

        println!("File contents:\n{}", contents);
        
        // TODO-TD: handle file with multiple TLEs

        let tle: TLE = parse(&contents.as_str());
        println!("{}", tle);
    }
    else if command.is_some() 
    {

        // https://celestrak.org/NORAD/elements/gp.php?{QUERY}=VALUE
        let mut res = reqwest::blocking::get("https://celestrak.org/").unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        
        println!("Status: {}", res.status());
        println!("Headers:\n{:#?}", res.headers());
        println!("Body:\n{}", body);
    }
    else
    {

        println!("No tle provided, running with demo values!!");
        
        let tle_str: &str = 
        "ISS (ZARYA)
        1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
        2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";
        
        let tle: TLE = parse(tle_str);
        println!("{}", tle);
    }

}
