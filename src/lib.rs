use std::error::Error;

use clap::{Arg, Command};

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
        .arg(Arg::new(""))
        .get_matches();

    Ok(Config {
        files: vec![],
        number_line: false,
        number_nonblank_lines: false,
    })
}

pub fn run(_config: Config) -> CatrResault<()> {
    println!("Hello World");
    Ok(())
}
