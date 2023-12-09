/*
Executable for TLE interefacing

*/
use std::fs;
use std::io::Read;
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
    
    /// Path to .txt file holding tle information
    #[arg(short, long)]
    file_path: Option<String>,
    
    /// Flag for celestrak mode query with key desired
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
    let verbose: bool = cli.verbose;
    let tle_string_option: Option<String> = cli.two_line_element;
    let file_path_option: Option<String> = cli.file_path;
    let output_path_option: Option<String> = cli.output_path;
    let command_option: Option<Commands> = cli.command;

    let default_str: &str = 
    "ISS (ZARYA)
    1 25544U 98067A   08264.51782528 -.00002182  00000-0 -11606-4 0  2927
    2 25544  51.6416 247.4627 0006703 130.5360 325.0288 15.72125391563537";
    let mut tles: Vec<TLE> = vec![parse(default_str)];
    
    if tle_string_option.is_some(){
        tles[0] = parse(tle_string_option.unwrap().as_str());
    }
    else if file_path_option.is_some()  
    {
        let file: String = file_path_option.unwrap();

        if file.contains(".json") {
            tles[0] = read_json(&file.as_str());

        } else {

            let contents: String = fs::read_to_string(file.as_str())
            .expect("Should have been able to read the file");
    
            if verbose {
                println!("File contents:\n{}", contents);
            }
    
            let lines: Vec<&str> = contents.lines().collect();
            let n_lines: usize = lines.len();
            if n_lines < 3 {

                tles[0] = parse(&contents.as_str());

            } else {
                let n_tles: usize = n_lines / 3;
                let file_str: &str = &contents.as_str();
                let tle_lines: Vec<&str> = file_str.lines().collect();
    
                for i_tle in 0..n_tles{
    
                    let line1: &str = tle_lines[3*i_tle];
                    let line2: &str = tle_lines[3*i_tle + 1];
                    let line3: &str = tle_lines[3*i_tle + 2];
    
                    let together: String = format!("{line1}\n{line2}\n{line3}\n");
                    tles[i_tle] = parse(together.as_str());
                }
            };
        }

    }
    else if command_option.is_some() 
    {
        // TODO-TD: replace elifs with match
        let mut url: String = "https://celestrak.org/NORAD/elements/gp.php?".to_owned();

        let (query, value) = match command_option.unwrap(){
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

        tles[0] = parse(&body.as_str());
    }
    else
    {
        println!("\nNo tle provided, running with demo values!!");
    }

    let is_write: bool;
    let mut output_path: String = "./".to_string();

    if output_path_option.is_some(){
        output_path = output_path_option.unwrap();
        is_write = true;
    } else {
        is_write = false
    }

    for tle in tles{
        if verbose {
            println!("\n{}", tle);
        }

        if is_write{
            let name: String = tle.name.clone();               
            let file_path: String = format!("{output_path}\\{name}.json");
            write_json(tle, &file_path);
            
            if verbose {
                println!("Wrote tle for {} in json format to: {}", name, file_path);
            }
        }
    }

}
