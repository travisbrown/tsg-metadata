use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    let _ = tsg_metadata::cli::init_logging(opts.verbose)?;

    let file = File::open(opts.path)?;
    let mut count = 0;

    for line in BufReader::new(file).lines() {
        let line = line?;
        //println!("{}", line);
        //println!("{:?}", find_user_info(&line));
        count += find_user_info_json(&line).len();

        //let mut hit = line.find("\"user\":")
    }

    println!("Found: {} user occurrences", count);

    Ok(())
}

fn find_user_info(input: &str) -> Vec<u64> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r#""user"\s*:\{[^\}]+"id_str"\s*:\s*"(\d+)""#).unwrap();
    }

    RE.captures_iter(input)
        .map(|capture| capture[1].parse::<u64>().unwrap())
        .collect()
}

fn find_user_info_json(input: &str) -> Vec<u64> {
    /*lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r#""user"\s*:\{[^\}]+"id_str"\s*:\s*"(\d+)""#).unwrap();
    }

    RE.captures_iter(input).map(|capture| {
        capture[1].parse::<u64>().unwrap()
    }).collect()*/

    let value = serde_json::from_str(input).unwrap();
    let mut result = Vec::with_capacity(1);

    find_user_id_str(&value, &mut result);

    result
}

fn find_user_id_str(value: &serde_json::Value, acc: &mut Vec<u64>) -> () {
    match value {
        serde_json::Value::Array(values) => {
            for value in values {
                find_user_id_str(&value, acc);
            }
        }
        serde_json::Value::Object(map) => {
            if let Some(user) = map.get("user") {
                acc.push(
                    user.get("id_str")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .parse()
                        .unwrap(),
                );
            }
            for value in map.values() {
                find_user_id_str(&value, acc);
            }
        }
        _ => (),
    }
}

#[derive(Debug, Parser)]
#[clap(name = "scan", version, author)]
struct Opts {
    /// Level of verbosity
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    /// File or directory path
    #[clap(short, long)]
    path: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Logging initialization error")]
    LogInit(#[from] log::SetLoggerError),
}
