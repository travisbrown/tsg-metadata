use cli_helpers::prelude::*;
use std::path::Path;

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    opts.verbose.init_logging()?;

    let path = Path::new(&opts.path);
    let records = if path.is_dir() {
        tsg_metadata::source::read_metadata_dir(path)?
    } else {
        tsg_metadata::source::read_metadata(path)?
    };

    match opts.command {
        Command::Urls { https } => {
            for record in records {
                println!(
                    "http{}://archive.org/download/{}/{}",
                    if https { "s" } else { "" },
                    record.item,
                    record.name
                );
            }
        }
        Command::Digests => {
            for record in records {
                println!(
                    "{},{},{},{},{}",
                    record.item,
                    record.name,
                    record.size,
                    hex::encode(record.sha1),
                    hex::encode(record.md5)
                );
            }
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[clap(name = "source", version, author)]
struct Opts {
    /// Level of verbosity
    #[clap(flatten)]
    verbose: Verbosity,
    /// File or directory path
    #[clap(short, long, default_value = "sources/xml/")]
    path: String,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    Urls {
        /// Use https
        #[clap(long)]
        https: bool,
    },
    Digests,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Metadata parsing error")]
    Sources(#[from] tsg_metadata::source::Error),
    #[error("CLI initialization error")]
    Cli(#[from] cli_helpers::Error),
}
