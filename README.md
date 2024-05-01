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

Example

```rust
use parse_tle::tle::*;
let sample_tle: &str = 
"CHANDRAYAAN-3      
1 57320U 23098A   23208.62000000  .00000392  00000+0  00000+0 0  9994
2 57320  21.3360   6.1160 9054012 182.9630  18.4770  0.46841359   195";

let chandrayaan_3: TLE = parse(sample_tle);
let String file_path = format!("{chandrayaan_3.name}_tle.json")
write_json(chandrayaan_3, &file_path);
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

Usage: parse_tle.exe [OPTIONS] [TWO_LINE_ELEMENT] [COMMAND]

Commands:
  celestrak  Query celestrak for tles
  help       Print this message or the help of the given subcommand(s)

Arguments:
  [TWO_LINE_ELEMENT]  Two line element directly in cli

Options:
  -v, --verbose                    Verbose printing
  -f, --file-path <FILE_PATH>      path to .txt file holding tle information
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
