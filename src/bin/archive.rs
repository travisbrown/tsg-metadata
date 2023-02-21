use clap::Parser;
use std::fs::File;

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    tsg_metadata::cli::init_logging(opts.verbose)?;

    match opts.command {
        Command::Contents => {
            let file = File::open(opts.path)?;
            let contents = tsg_metadata::archive::tar::list(file)?;

            for (path, size) in contents {
                println!("{:?},{}", path, size);
            }
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(name = "source", version, author)]
struct Opts {
    /// Level of verbosity
    #[clap(short, long, global = true, action = clap::ArgAction::Count)]
    verbose: u8,
    /// File or directory path
    #[clap(short, long)]
    path: String,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    Contents,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("TAR error")]
    Tar(#[from] tsg_metadata::archive::tar::Error),
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Logging initialization error")]
    LogInit(#[from] log::SetLoggerError),
}
