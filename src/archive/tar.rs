use std::io::{Read, Seek};
use std::path::PathBuf;

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
        /*.filter(|result| match result {
            Ok((path, _)) => path.is_file(),
            Err(_) => true,
        })*/
        .collect::<Result<Vec<_>, _>>()?;

    results.sort();

    Ok(results)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
}
