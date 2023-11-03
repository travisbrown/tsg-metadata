use bzip2::read::MultiBzDecoder;
use flate2::read::GzDecoder;
use memmap::Mmap;
use piz::read::FileMetadata;
use piz::ZipArchive;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use tar::Archive;

pub enum ArchiveEntries<'a> {
    Zip {
        archive: &'a ZipArchive<'a>,
        entries: Vec<&'a FileMetadata<'a>>,
    },
    Tar {
        entries: tar::Entries<'a, File>,
    },
    TarGz {
        entries: tar::Entries<'a, GzDecoder<File>>,
    },
}

impl<'a> From<&'a ZipArchive<'a>> for ArchiveEntries<'a> {
    fn from(value: &'a ZipArchive<'a>) -> Self {
        let mut entries = value
            .entries()
            .iter()
            .filter(|metadata| metadata.is_file())
            .collect::<Vec<_>>();

        entries.reverse();

        Self::Zip {
            archive: value,
            entries,
        }
    }
}

impl<'a> Iterator for ArchiveEntries<'a> {
    type Item = Result<(FileEntry, String), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Zip { archive, entries } => entries.pop().map(|value| {
                let file_entry: FileEntry = value.into();
                let mut reader = archive.read(value)?;

                let mut buffer = String::new();

                if file_entry.is_bzip2() {
                    MultiBzDecoder::new(reader)
                        .read_to_string(&mut buffer)
                        .map_err(|error| Error::DecodingIo(file_entry.clone(), error))?;
                } else {
                    reader
                        .read_to_string(&mut buffer)
                        .map_err(|error| Error::DecodingIo(file_entry.clone(), error))?;
                }

                Ok((file_entry, buffer))
            }),
            Self::Tar { entries } => match entries.next() {
                Some(Ok(entry)) => match read_tar_entry(entry) {
                    Ok(Some((file_entry, value))) => Some(Ok((file_entry, value))),
                    Ok(None) => self.next(),
                    Err(error) => Some(Err(error)),
                },
                Some(Err(error)) => Some(Err(Error::from(error))),
                None => None,
            },
            Self::TarGz { entries } => match entries.next() {
                Some(Ok(entry)) => match read_tar_entry(entry) {
                    Ok(Some((file_entry, value))) => Some(Ok((file_entry, value))),
                    Ok(None) => self.next(),
                    Err(error) => Some(Err(error)),
                },
                Some(Err(error)) => Some(Err(Error::from(error))),
                None => None,
            },
        }
    }
}

fn read_tar_entry<R: Read>(mut entry: tar::Entry<R>) -> Result<Option<(FileEntry, String)>, Error> {
    let path = entry.path()?.to_path_buf();

    let path_str = path
        .to_str()
        .ok_or_else(|| Error::InvalidEntryPath(path.clone()))?;
    let size = entry.size();

    if size > 0 {
        let file_entry = FileEntry::new(path_str.to_string(), size as u32, None);

        let mut buffer = String::new();

        if file_entry.is_bzip2() {
            MultiBzDecoder::new(entry)
                .read_to_string(&mut buffer)
                .map_err(|error| Error::DecodingIo(file_entry.clone(), error))?;
        } else {
            entry
                .read_to_string(&mut buffer)
                .map_err(|error| Error::DecodingIo(file_entry.clone(), error))?;
        }

        Ok(Some((file_entry, buffer)))
    } else {
        let mut buffer = Vec::new();
        entry.read_to_end(&mut buffer)?;
        Ok(None)
    }
}

pub fn list_entries<P: AsRef<Path>>(input: P) -> Result<Vec<FileEntry>, Error> {
    let extension: Extension = input.as_ref().try_into()?;
    let file = File::open(input)?;

    let mut entries: Vec<FileEntry> = match extension {
        Extension::Zip => {
            let mapping = unsafe { Mmap::map(&file)? };
            let archive = ZipArchive::new(&mapping)?;
            archive
                .entries()
                .iter()
                .filter(|metadata| metadata.is_file())
                .map(FileEntry::from)
                .collect::<Vec<_>>()
        }
        Extension::Tar => {
            let mut archive = Archive::new(file);
            list_tar_entries(&mut archive)?
        }
        Extension::TarGz => {
            let stream = GzDecoder::new(file);
            let mut archive = Archive::new(stream);
            list_tar_entries(&mut archive)?
        }
    };

    entries.sort();

    Ok(entries)
}

fn list_tar_entries<R: Read>(archive: &mut Archive<R>) -> Result<Vec<FileEntry>, Error> {
    archive
        .entries()?
        .map(|entry| {
            entry
                .and_then(|entry| entry.path().map(|path| (path.to_path_buf(), entry.size())))
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
        .collect()
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Extension {
    Zip,
    Tar,
    TarGz,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FileEntry {
    pub path: String,
    pub size: u32,
    pub crc32: Option<u32>,
}

impl FileEntry {
    fn new(path: String, size: u32, crc32: Option<u32>) -> Self {
        Self { path, size, crc32 }
    }

    pub fn is_bzip2(&self) -> bool {
        self.path.to_lowercase().ends_with(".bz2")
    }
}

impl<'a> From<&FileMetadata<'a>> for FileEntry {
    fn from(value: &FileMetadata) -> Self {
        Self::new(
            value.path.as_str().to_string(),
            value.size as u32,
            Some(value.crc32),
        )
    }
}

impl TryFrom<&Path> for Extension {
    type Error = Error;
    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let file_str = value
            .file_name()
            .and_then(|value| value.to_str())
            .map(|value| value.to_lowercase())
            .ok_or_else(|| Error::InvalidEntryPath(value.to_path_buf()))?;

        let parts = file_str.split('.').collect::<Vec<_>>();
        println!("{:?}", parts);

        match parts.last() {
            Some(value) if *value == "zip" => Ok(Self::Zip),
            Some(value) if *value == "tar" => Ok(Self::Tar),
            Some(value)
                if *value == "gz"
                    && parts
                        .get(parts.len() - 2)
                        .filter(|value| **value == "tar")
                        .is_some() =>
            {
                Ok(Self::TarGz)
            }
            Some(value) => Err(Error::UnknownExtension(value.to_string())),
            None => Err(Error::InvalidEntryPath(value.to_path_buf())),
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
    #[error("Decoding I/O error")]
    DecodingIo(FileEntry, std::io::Error),
}
