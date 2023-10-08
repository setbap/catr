// this code is worst code that you ever seen and i wrote
// no matter how much is bad,i just wana write rust code and solve some errors that compiler show
// me. in future my ability in writing Rust code will improve :)
use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader},
};

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

#[derive(Debug)]
pub struct Output {
    line: Option<i32>,
    word: Option<i32>,
    byte: Option<i32>,
    char: Option<i32>,
}

impl Output {
    fn new() -> Self {
        Output {
            line: Some(0),
            word: Some(0),
            byte: Some(0),
            char: Some(0),
        }
    }

    fn new_base_none() -> Self {
        Output {
            line: None,
            word: None,
            byte: None,
            char: None,
        }
    }

    fn increase(&mut self, line: i32, word: i32, byte: i32, char: i32) {
        self.line = Some(self.line.unwrap() + line);
        self.word = Some(self.word.unwrap() + word);
        self.byte = Some(self.byte.unwrap() + byte);
        self.char = Some(self.char.unwrap() + char);
    }

    fn fill_according_config(base: Output, config: &Config) -> Self {
        let mut new_output = Output::new_base_none();

        if config.lines {
            new_output.line = Some(base.line.unwrap());
        }
        if config.words {
            new_output.word = Some(base.word.unwrap());
        }
        if config.bytes {
            new_output.byte = Some(base.byte.unwrap());
        }
        if config.chars {
            new_output.char = Some(base.char.unwrap());
        }
        new_output
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        for ele in [self.line, self.word, self.byte, self.char] {
            if let Some(el) = ele {
                res += &format!("{:>6}\t", el);
            }
        }
        write!(f, "{}", res)
    }
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

fn open_file(filename: &str) -> CustomResault<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> CustomResault<()> {
    let filenames = config.files;
    let new_config = Config {
        files: Vec::new(),
        lines: config.lines,
        words: config.words,
        bytes: config.bytes,
        chars: config.chars,
    };
    for filename in filenames {
        match open_file(&filename) {
            Err(e) => eprint!("{}", e),
            Ok(result) => count(&new_config, result),
        }
        println!("\t {}", filename);
    }
    Ok(())
}

fn count(config: &Config, result: Box<dyn BufRead>) {
    let mut output_container = Output::new();
    result.lines().for_each(|line| match line {
        Err(e) => eprintln!("{}", e),
        Ok(line) => output_container.increase(
            1,
            line.split_whitespace().count() as i32,
            // not always true :) + performance and memory problem
            line.chars().count() as i32,
            line.bytes().count() as i32,
        ),
    });

    print!(
        "{}",
        Output::fill_according_config(output_container, config)
    );
}
