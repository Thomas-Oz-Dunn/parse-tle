/*
Executable for TLE interefacing

*/
use clap::{Args, Parser, Subcommand};
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

    /// Path to file holding tle information
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
    /// 
    /// - CATNR: Catalog Number (1 to 9 digits). Allows return of data for a single catalog number.
    ///
    /// - INTDES: International Designator (yyyy-nnn). Allows return of data for all objects associated with a particular launch.
    ///
    /// - GROUP: Groups of satellites provided on the CelesTrak Current Data page.
    ///
    /// - NAME: Satellite Name. Allows searching for satellites by parts of their name.
    ///
    /// - SPECIAL: Special data sets for the GEO Protected Zone (GPZ) or GPZ Plus.
    ///
    query: String,

    /// Object value
    value: String,
}

fn main() {
    let cli: CLI = CLI::parse();
    let verbose: bool = cli.verbose;
    let tle_string_option: Option<String> = cli.two_line_element;
    let file_path_option: Option<String> = cli.file_path;
    let output_path_option: Option<String> = cli.output_path;
    let command_option: Option<Commands> = cli.command;

    let tles: Vec<TLE>;

    if tle_string_option.is_some() {
        tles = vec![parse(tle_string_option.unwrap().as_str())];
    } else if file_path_option.is_some() {
        tles = tles_from_file(&file_path_option.unwrap().as_str());
    } else if command_option.is_some() {
        let (query, value) = match command_option.unwrap() {
            Commands::Celestrak(celestrak_args) => (celestrak_args.query, celestrak_args.value),
        };
        tles = query_celestrak(&query, &value, verbose);
    } else {
        println!("\nNo tle provided!\n\nUse the '-h' flag for help");
        return;
    }

    let is_write: bool;
    let mut output_path: String = "./".to_string();

    if output_path_option.is_some() {
        output_path = output_path_option.expect("Expected valid output path");
        is_write = true;
    } else {
        is_write = false
    }

    for tle in &tles {
        println!("\n{}", tle);

        if is_write {
            let name: String = tle.name.clone();
            let file_path: String;

            if output_path.contains(".json") {
                file_path = output_path.clone();
            } else {
                file_path = format!("{output_path}/{name}.json");
            }

            write_json(tle, &file_path);

            if verbose {
                println!("Wrote tle for {} in json format to: {}", name, file_path);
            }
        }
    }
}
