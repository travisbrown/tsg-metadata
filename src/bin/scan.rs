use bzip2::read::MultiBzDecoder;
use clap::Parser;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    tsg_metadata::cli::init_logging(opts.verbose)?;

    let lines = tsg_metadata::archive::zip::extract(File::open(&opts.path)?, &opts.file)?;

    let mut count = 0;
    for line in lines {
        let value = serde_json::from_str(&line).unwrap();
        count += tsg_metadata::extract::find_users(&value).unwrap().len();
    }

    /*for _ in 0..100 {
        let file = File::open(&opts.path)?;

        if opts.path.ends_with(".bz2") {
            for line in BufReader::new(MultiBzDecoder::new(file)).lines() {
                let line = line?;
                count += find_user_info_json(&line).len();
            }
        } else {
            for line in BufReader::new(file).lines() {
                let line = line?;
                count += find_user_info_json(&line).len();
            }
        }
    }*/
    println!("Found: {} user occurrences", count);

    Ok(())
}

fn find_user_info(input: &str) -> Vec<u64> {
    lazy_static::lazy_static! {
        static ref RE: Regex = Regex::new(r#""user"\s*:\{[^\}]+"id_str"\s*:\s*"(\d+)""#).unwrap();
        //static ref RE: Regex = Regex::new(r#""user(?:_mentions)?"\s*:\s*(?:\[)?\s*\{[^\}]+"id_str"\s*:\s*"(\d+)""#).unwrap();
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

fn find_user_id_str(value: &serde_json::Value, acc: &mut Vec<u64>) {
    match value {
        serde_json::Value::Array(values) => {
            for value in values {
                find_user_id_str(value, acc);
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
                find_user_id_str(value, acc);
            }
        }
        _ => (),
    }
}

#[derive(Debug, Parser)]
#[clap(name = "scan", version, author)]
struct Opts {
    /// Level of verbosity
    #[clap(short, long, global = true, action = clap::ArgAction::Count)]
    verbose: u8,
    /// File or directory path
    #[clap(short, long)]
    path: String,
    /// File in archive
    #[clap(short, long)]
    file: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Logging initialization error")]
    LogInit(#[from] log::SetLoggerError),
    #[error("Zip error")]
    Zip(#[from] tsg_metadata::archive::zip::Error),
}
