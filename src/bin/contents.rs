use cli_helpers::prelude::*;
use memmap::Mmap;
use piz::ZipArchive;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use tsg_metadata::archive::{list_entries, ArchiveEntries, Extension, FileEntry};

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    opts.verbose.init_logging()?;

    match opts.command {
        Command::One => {
            let entries = list_entries(opts.input)?;
            for FileEntry { path, size, crc32 } in entries {
                println!(
                    "{path},{size},{}",
                    crc32.map(|value| value.to_string()).unwrap_or_default()
                );
            }
        }
        Command::All { output, start } => {
            let output_dir_path = Path::new(&output);
            let mut files = list_files(opts.input)?.into_iter().collect::<Vec<_>>();
            files.sort();

            for (name, path) in files {
                if start.as_ref().filter(|start| start > &&name).is_none() {
                    let output_path = output_dir_path.join(format!("{name}.csv"));
                    let output_file = File::create(output_path)?;
                    let mut writer = BufWriter::new(output_file);

                    let entries = list_entries(path)?;
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
        Command::Dump => {
            let extension: Extension = opts.input.as_path().try_into()?;
            let file = File::open(&opts.input)?;

            match extension {
                Extension::Zip => {
                    let mapping = unsafe { Mmap::map(&file)? };
                    let archive = ZipArchive::new(&mapping)?;
                    for result in ArchiveEntries::from(&archive) {
                        match result {
                            Ok((_, value)) => {
                                println!("{}", value);
                            }
                            Err(error) => {
                                ::log::error!("{:?}", error);
                            }
                        }
                    }
                }
                Extension::Tar => {
                    let mut archive = tar::Archive::new(file);
                    for result in (ArchiveEntries::Tar {
                        entries: archive.entries()?,
                    }) {
                        match result {
                            Ok((_, value)) => {
                                println!("{}", value);
                            }
                            Err(error) => {
                                ::log::error!("{:?}", error);
                            }
                        }
                    }
                }
                Extension::TarGz => {
                    let stream = flate2::read::GzDecoder::new(file);
                    let mut archive = tar::Archive::new(stream);
                    for result in (ArchiveEntries::TarGz {
                        entries: archive.entries()?,
                    }) {
                        match result {
                            Ok((_, value)) => {
                                println!("{}", value);
                            }
                            Err(error) => {
                                ::log::error!("{:?}", error);
                            }
                        }
                    }
                }
            };
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
    input: PathBuf,
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    One,
    All {
        /// Output directory path
        #[clap(short, long)]
        output: PathBuf,
        /// File name to start with
        #[clap(short, long)]
        start: Option<String>,
    },
    Dump,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Archive error")]
    Archive(#[from] tsg_metadata::archive::Error),
    #[error("Zip error")]
    Zip(#[from] piz::result::ZipError),
    #[error("Archive file name collision")]
    FileNameCollision(String),
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("CLI initialization error")]
    Cli(#[from] cli_helpers::Error),
}
