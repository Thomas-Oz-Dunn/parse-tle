# parse-tle

[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

TLE parser in Rust

## Install

Command Line:

```bash
cargo install parse-tle
```

or add to your `cargo.toml`:

```.yml
[dependencies]
parse-tle = "0.1.3"
```

## Usage

API

```rust

use parse_tle::tle::*;

let tle: TLE = parse(tle_string);
println!("\n{}", tle);
```

CLI

To build:

```bash
cargo build --release
```

To run:

```bash
./target/release/parse_tle.exe -h
```

Which generates the help page:

```bash
Parse two line element set

Usage: parse_tle [OPTIONS] [TWO_LINE_ELEMENT] [COMMAND]

Commands:
  celestrak  Query celestrak for tles
  help       Print this message or the help of the given subcommand(s)

Arguments:
  [TWO_LINE_ELEMENT]  Two line element directly in cli

Options:
  -v, --verbose                    Verbose printing
  -f, --file-path <FILE_PATH>      Path to file holding tle information
  -o, --output-path <OUTPUT_PATH>  Path to write json formatted output
  -h, --help                       Print help
  -V, --version                    Print version
```

To run an example of reading from file:

```bash
./target/release/parse_tle.exe -f ./examples/starlink_1007_tle.txt

STARLINK-1007
Catalog #: 44713U
Intl Desig: 19074A
Epoch: 2023-11-05T00:04:07 UTC
Mean Motion: 15.06420179
Mean Motion prime: 0.00013448
Mean Motion prime 2: 0
Radiation Pressure: 91.946
Inclination: 53.0548
Raan: 292.5095
Eccentricity: 0.0001422
Argument of Perigee: 105.9473
Mean Anomaly: 54.1673
Revolution #: 24322
```

Query celestrak for the ISS:
```bash
./target/release/parse_tle celestrak CATNR 25544
```