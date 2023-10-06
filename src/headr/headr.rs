use std::{
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

use clap::{value_parser, Arg, ArgAction, Command};

use crate::types::CustomResault;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> CustomResault<Config> {
    let _matches = Command::new("Head in Rust")
        .version("1.0.2")
        .author("Sina (Setbap)")
        .about("Simple Head in Rust")
        .name("headr")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input File(s)")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_parser(value_parser!(usize))
                .default_value("10")
                .help("Number of Lines")
                .conflicts_with("bytes"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_parser(value_parser!(usize))
                .help("count of bytes")
                .conflicts_with("lines"),
        )
        .get_matches();

    let _v: Vec<String> = _matches
        .get_many::<String>("files")
        .unwrap()
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    Ok(Config {
        files: _v,
        lines: *_matches.get_one("lines").unwrap(),
        bytes: _matches.get_one("bytes").map(|f| *f),
    })
}

fn open_file(filename: &str) -> CustomResault<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(_config: Config) -> CustomResault<()> {
    for filename in _config.files {
        match open_file(&filename) {
            Err(err) => eprintln!("Error in Opening {} : {}", filename, err),
            Ok(file) => {
                println!("\n----------- {} ----------", filename);
                let lines = _config.lines;
                if let Some(byte) = _config.bytes {
                    let bytes: Result<Vec<_>, _> = file.bytes().take(byte).collect();
                    print!("{}", String::from_utf8_lossy(&bytes?));
                } else {
                    for line in file.lines().take(lines) {
                        println!("{}", line?);
                    }
                }
            }
        }
        println!("");
    }
    Ok(())
}
