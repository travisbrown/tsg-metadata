use cli_helpers::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use tsg_metadata::archive::{read_contents, FileEntry};

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    opts.verbose.init_logging()?;

    let input = Path::new(&opts.input);

    match opts.command {
        Command::One => {
            let entries = read_contents(input)?;
            for FileEntry { path, size, crc32 } in entries {
                println!(
                    "{path},{size},{}",
                    crc32.map(|value| value.to_string()).unwrap_or_default()
                );
            }
        }
        Command::All { output, start } => {
            let output_dir_path = Path::new(&output);
            let mut files = list_files(input)?.into_iter().collect::<Vec<_>>();
            files.sort();

            for (name, path) in files {
                if start.as_ref().filter(|start| start > &&name).is_none() {
                    let output_path = output_dir_path.join(format!("{name}.csv"));
                    let output_file = File::create(output_path)?;
                    let mut writer = BufWriter::new(output_file);

                    let entries = read_contents(path)?;
                    for FileEntry { path, size, crc32 } in entries {
                        writeln!(
                            writer,
                            "{path},{size},{}",
                            crc32.map(|value| value.to_string()).unwrap_or_default()
                        )?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn list_files<P: AsRef<Path>>(base: P) -> Result<HashMap<String, PathBuf>, Error> {
    let mut result = HashMap::new();

    for year_entry in std::fs::read_dir(base)? {
        let year_entry = year_entry?;

        for archive_entry in std::fs::read_dir(year_entry.path())? {
            let archive_entry = archive_entry?;
            let archive_path = archive_entry.path();
            if let Some(file_name) = archive_path
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .map(|file_name| file_name.to_string())
            {
                result
                    .insert(file_name.clone(), archive_path)
                    .map_or(Ok(()), |_| Err(Error::FileNameCollision(file_name)))?;
            } else {
                ::log::warn!("Unexpected archive path: {:?}", archive_path);
            }
        }
    }

    Ok(result)
}

#[derive(Debug, Parser)]
#[clap(name = "contents", version, author)]
struct Opts {
    /// Level of verbosity
    #[clap(flatten)]
    verbose: Verbosity,
    /// File or directory path
    #[clap(short, long)]
    input: String,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    One,
    All {
        /// Output directory path
        #[clap(short, long)]
        output: String,
        /// File name to start with
        #[clap(short, long)]
        start: Option<String>,
    },
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Archive error")]
    Archive(#[from] tsg_metadata::archive::Error),
    #[error("Archive file name collision")]
    FileNameCollision(String),
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("CLI initialization error")]
    Cli(#[from] cli_helpers::Error),
}
