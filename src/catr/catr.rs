use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{Arg, ArgAction, Command};

use crate::types::CustomResault;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_line: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> CustomResault<Config> {
    let _matches = Command::new("Cat in Rust")
        .version("0.0.1")
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
                let mut nonblocking_lines_num = 0;
                for (i, line) in file.lines().enumerate() {
                    let line = line?;

                    if _config.number_line {
                        print!("{:>2}\t", i + 1);
                    }

                    if _config.number_nonblank_lines && !line.is_empty() {
                        print!("{:>2}\t", nonblocking_lines_num + 1);
                        nonblocking_lines_num += 1;
                    }

                    println!("{}", line);
                }
            }
        }
        println!("");
    }
    Ok(())
}
