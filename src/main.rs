/*
Executable for TLE interefacing

*/
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use clap::{Parser, Subcommand, Args};
use error_chain::error_chain;

use parse_tle::tle::*;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {
    /// Verbose printing
    #[arg(short, long)]
    verbose: bool,

    /// Two line element directly in cli
    two_line_element: Option<String>,
    
    /// path to .txt file holding tle information
    #[arg(short, long)]
    file_path: Option<String>,
    
    /// flag for celestrak mode query with key desired
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Path to write json formatted output
    #[arg(short, long)]
    output_path: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Query celestrak for tles
    Celestrak(CelestrakArgs),
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
    query: String,

    /// Object name
    name: String,

}


fn main() {
    let cli: CLI = CLI::parse();
    let tle_string: Option<String> = cli.two_line_element;
    let verbose: bool = cli.verbose;
    let file_path: Option<String> = cli.file_path;
    let output_path: Option<String> = cli.output_path;
    let command = cli.command;
     
    if tle_string.is_some(){
        let tle: TLE = parse(tle_string.unwrap().as_str());
        println!("{}", tle);
    }
    else if file_path.is_some()  
    {
        
        let contents: String = fs::read_to_string(
            file_path.unwrap().as_str())
        .expect("Should have been able to read the file");

        if verbose {
            println!("File contents:\n{}", contents);
        }
        
        // TODO-TD: handle file with multiple TLEs
        let tle: TLE = parse(&contents.as_str());
        println!("{}", tle);
    }
    else if command.is_some() 
    {
        // TODO-TD: replace elifs with match
        let mut url: String = "https://celestrak.org/NORAD/elements/gp.php?".to_owned();

        let (query, value) = match command.unwrap(){
            Commands::Celestrak(celestrak_args) => (
                celestrak_args.query, celestrak_args.name),
        };

        url.push_str(query.as_str());
        url.push_str("=");
        url.push_str(value.as_str());
        
        if verbose{
            println!("\n Site url: {}", url);
        }
    
        let mut res = reqwest::blocking::get(url).unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
    
        if verbose {
            println!("\n Site Status: {}", res.status());
            println!("\n Site Headers:\n{:#?}", res.headers());
            println!("\n Site Body:\n{}", body);
        }

        let tle: TLE = parse(&body.as_str());
        println!("{}", tle);

    }
    else
    {

        println!("\nNo tle provided, running with demo values!!");
        
        let tle_str: &str = 
        "ISS (ZARYA)
        1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
        2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";
        
        let tle: TLE = parse(tle_str);
        println!("\n{}", tle);
        if output_path.is_some(){
            write_json(tle, output_path.unwrap().as_str());
        }
    }

}
