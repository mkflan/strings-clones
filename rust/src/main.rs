#![allow(dead_code, unused)]
#![deny(rust_2018_idioms)]

mod cli;
mod config;

use config::ConfigBuilder;
use std::env;

static HELP: &str = "Usage: rstrings [option(s)] [file(s)]
Function: Display printable strings in the given files
Options:
    -d --data                    Scan only the data sections in the file (default: scan whole file)
    -f --print-file-name         Print the name of the file before each string
    -n <number> --bytes=<number> Locate & print any sequence of atleast <number> displayable characters (default: 4)
    -t --radix={o,d,x}           Print the location of the string in base 8, 10, or 16
    -w --whitespace              Include all whitespace as valid string characters
    -e --encoding={s,S,b,l,B,L}  Select character size and endianness: s = 7-bit, S = 8-bit, {b,l} = 16-bit, {B,L} = 32-bit
    --unicode={default|show|invalid|hex|escape|highlight}
    -U {d|s|i|x|e|h}             Specify how to treat UTF-8 encoded unicode chracters
    -s --separator=<string>      String used to separate strings in output
    -h --help                    Display this information
";

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() <= 1 {
        println!("{HELP}");
        std::process::exit(0);
    }

    let config = cli::parse_cli_args(args);
}
