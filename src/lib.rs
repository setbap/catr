use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{Arg, ArgAction, Command};

type CatrResault<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_line: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> CatrResault<Config> {
    let _matches = Command::new("Clap in Rust")
        .version("1.0.2")
        .author("Sina (Setbap)")
        .about("Simple Cat in Rust")
        .name("catr")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input File(s)")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("Number of Lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .help("Number non-block Lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number"),
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
        number_line: _matches.get_flag("number"),
        number_nonblank_lines: _matches.get_flag("number_nonblank"),
    })
}

fn open_file(filename: &str) -> CatrResault<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(_config: Config) -> CatrResault<()> {
    for filename in _config.files {
        match open_file(&filename) {
            Ok(_) => println!("Opended : {}", filename),
            Err(err) => eprintln!("Error in Opening {} : {}", filename, err),
        }
    }
    Ok(())
}
