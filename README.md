# parse-tle

[![License:MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

TLE parser in Rust

To install:
```bash
cargo install parse-tle
```

or add to your `cargo.toml`:
```
[dependencies]
parse-tle = "0.1.4"
```

Usage
-----
### API

```rust
let tle: TLE = tle.parse(&str);

```

`TLE` struct is defined as:

```rust
pub struct TLE {
    pub name: String,
    pub catalog_number: String,
    pub international_designator: String,
    pub epoch: Epoch,
    pub epoch_year: i32,
    pub epoch_month: u8,
    pub epoch_day: u8,
    pub epoch_hours: u8,  
    pub epoch_min: u8,
    pub epoch_sec: u8,
    pub mean_motion_1: f64,
    pub mean_motion_2: f64,
    pub radiation_pressure: f64,
    pub inc: f64,
    pub raan: f64,
    pub eccentricity:  f64,
    pub arg_perigee: f64,
    pub mean_anomaly: f64,    
    pub mean_motion: f64,
    pub rev_num: u32
}
```

### CLI

```bash
$ ./target/debug/parse_tle.exe -h
Parse two line element set

Usage: parse_tle.exe [OPTIONS] [TWO_LINE_ELEMENT] [COMMAND]

Commands:
  celestrak  Query celestrak for tles
  help       Print this message or the help of the given subcommand(s)

Arguments:
  [TWO_LINE_ELEMENT]  Two line element directly in cli

Options:
  -v, --verbose                Verbose printing  
  -h, --help                   Print help
  -V, --version                Print version
  -f, --file-path <FILE_PATH>  path to .txt file holding tle information  
```

Running without any arguments defaults to the ISS

```bash
$ ./target/debug/parse_tle.exe

No tle provided, running with demo values!!

ISS (ZARYA)
Catalog #: 25544U
Intl Desig: 98067A
Epoch: 2008/8/20 0:12 25s
Mean Motion: 15.72125391
Mean Motion prime: -0.00002182
Mean Motion prime 2: 0
Radiation Pressure: -1.1606
Inclination: 51.6416
Raan: 247.4627
Eccentricity: 0.0006703
Argument of Perigee: 130.536
Mean Anomaly: 25.0288
Revolution #: 63537
```