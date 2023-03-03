use memmap::Mmap;
use piz::ZipArchive;
use std::ffi::OsStr;
use std::fs::File;
use std::path::{Path, PathBuf};
use tar::Archive;

pub fn read_contents<P: AsRef<Path>>(input: P) -> Result<Vec<FileEntry>, Error> {
    let extension: Extension = input.as_ref().extension().try_into()?;
    let file = File::open(input)?;

    let mut entries: Vec<FileEntry> = match extension {
        Extension::Zip => {
            let mapping = unsafe { Mmap::map(&file)? };
            let archive = ZipArchive::new(&mapping)?;
            archive
                .entries()
                .iter()
                .filter(|metadata| metadata.is_file())
                .map(|metadata| {
                    FileEntry::new(
                        metadata.path.as_str().to_string(),
                        metadata.size as u32,
                        Some(metadata.crc32),
                    )
                })
                .collect::<Vec<_>>()
        }
        Extension::Tar => {
            let mut archive = Archive::new(file);
            archive
                .entries()?
                .map(|entry| {
                    entry
                        .and_then(|entry| {
                            entry.path().map(|path| (path.to_path_buf(), entry.size()))
                        })
                        .map_err(Error::from)
                })
                .filter(|path| path.as_ref().map_or(true, |(_, size)| *size > 0))
                .map(|path| {
                    path.and_then(|(path, size)| {
                        path.to_str()
                            .ok_or_else(|| Error::InvalidEntryPath(path.to_path_buf()))
                            .map(|path| FileEntry::new(path.to_string(), size as u32, None))
                    })
                })
                .collect::<Result<Vec<_>, _>>()?
        }
    };

    entries.sort();

    Ok(entries)
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Extension {
    Zip,
    Tar,
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct FileEntry {
    pub path: String,
    pub size: u32,
    pub crc32: Option<u32>,
}

impl FileEntry {
    fn new(path: String, size: u32, crc32: Option<u32>) -> Self {
        Self { path, size, crc32 }
    }
}

impl TryFrom<Option<&OsStr>> for Extension {
    type Error = Error;
    fn try_from(value: Option<&OsStr>) -> Result<Self, Self::Error> {
        match value
            .and_then(|value| value.to_str())
            .map(|value| value.to_lowercase())
        {
            Some(value) if value == "zip" => Ok(Self::Zip),
            Some(value) if value == "tar" => Ok(Self::Tar),
            Some(other) => Err(Error::UnknownExtension(other)),
            None => Err(Error::UnknownExtension("".to_string())),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Zip error")]
    Zip(#[from] piz::result::ZipError),
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Unknown extension")]
    UnknownExtension(String),
    #[error("Invalid entry path")]
    InvalidEntryPath(PathBuf),
}
