use super::Archive;
use std::io::{BufRead, BufReader, Read, Seek};
use std::path::PathBuf;

impl<R: Read + Seek> Archive for tar::Archive<R> {
    type Entry = PathBuf;
    type Error = Error;

    fn list_entries(&mut self) -> Result<Vec<Self::Entry>, Self::Error> {
        let mut results = self
            .entries()?
            .map(|entry| {
                let entry = entry?;
                entry
                    .path()
                    .map(|path| path.to_path_buf())
                    .map_err(Error::from)
            })
            .filter(|result| match result {
                Ok(path) => path.is_file(),
                Err(_) => true,
            })
            .collect::<Result<Vec<_>, _>>()?;

        results.sort();

        Ok(results)
    }

    fn extract_bz2(&mut self, target: Self::Entry) -> Result<Option<Vec<String>>, Self::Error> {
        for entry in self.entries_with_seek()? {
            let entry = entry?;
            let path = entry.path()?;

            if target == path {
                let decoder = BufReader::new(bzip2::read::MultiBzDecoder::new(entry));

                return Ok(Some(
                    decoder
                        .lines()
                        .map(|line| line.map_err(Error::from))
                        .collect::<Result<Vec<_>, _>>()?,
                ));
            }
        }

        Ok(None)
    }
}

pub fn list<R: Read + Seek>(reader: R) -> Result<Vec<(PathBuf, u64)>, Error> {
    let mut archive = tar::Archive::new(reader);

    let mut results = archive
        .entries_with_seek()?
        .map(|entry| {
            let entry = entry?;
            entry
                .path()
                .map(|path| (path.to_path_buf(), entry.size()))
                .map_err(Error::from)
        })
        .filter(|result| match result {
            Ok((path, _)) => path.is_file(),
            Err(_) => true,
        })
        .collect::<Result<Vec<_>, _>>()?;

    results.sort();

    Ok(results)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
}
