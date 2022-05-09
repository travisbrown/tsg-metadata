use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    let _ = tsg_metadata::cli::init_logging(opts.verbose)?;

    let file = File::open(opts.path)?;

    for line in BufReader::new(file).lines() {
        let line = line?;
        println!("{}", line);

        let mut hit = line.find("\"user\":")
    }

    Ok(())
}

fn find_user_info(input: &str) -> Option<(u64, &str, &str)> {

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
