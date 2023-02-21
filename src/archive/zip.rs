use std::io::{BufRead, BufReader, Read, Seek};
use std::path::PathBuf;

pub fn list<R: Read + Seek>(reader: R) -> Result<Vec<String>, Error> {
    let archive = zip::ZipArchive::new(reader)?;

    let mut results = archive
        .file_names()
        .filter_map(|name| {
            if name.ends_with('/') {
                None
            } else {
                Some(name.to_string())
            }
        })
        .collect::<Vec<_>>();
    results.sort();

    Ok(results)
}

pub fn extract<R: Read + Seek>(reader: R, name: &str) -> Result<Vec<String>, Error> {
    let mut archive = zip::ZipArchive::new(reader)?;

    let file = BufReader::new(archive.by_name(name)?);
    let decoder = BufReader::new(bzip2::read::MultiBzDecoder::new(file));

    decoder
        .lines()
        .map(|line| line.map_err(Error::from))
        .collect()
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Zip error")]
    Zip(#[from] zip::result::ZipError),
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    #[test]
    fn run() {
        let example_file = File::open("examples/archives/twitter-stream-2021-01-01.zip").unwrap();
        let files = super::list(example_file).unwrap();

        assert_eq!(files.len(), 2);
    }
}
