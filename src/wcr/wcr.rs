use clap::{Arg, ArgAction, Command};

use crate::types::CustomResault;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn run(_config: Config) -> CustomResault<()> {
    dbg!(_config);
    Ok(())
}

pub fn get_args() -> CustomResault<Config> {
    let _matched = Command::new("wcr")
        .version("0.0.1")
        .author("Setbap")
        .about("Simple w[ord]c[ount][ in ]r[ust]")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("list of file[s]")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .help("show number of lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .help("show number of words")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help("show number of bytes")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .help("show number of chars")
                .default_value("false")
                .conflicts_with("bytes")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let mut lines = _matched.get_flag("lines");
    let mut words = _matched.get_flag("words");
    let mut bytes = _matched.get_flag("bytes");
    let chars = _matched.get_flag("chars");

    if [lines, words, bytes, chars].iter().all(|v| !v) {
        println!("debug");
        lines = true;
        words = true;
        bytes = true;
    }

    let files: Vec<String> = _matched
        .get_many::<String>("files")
        .unwrap()
        .map(|f| f.to_owned())
        .collect();

    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}
