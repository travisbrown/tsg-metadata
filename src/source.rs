use std::collections::HashMap;
use std::convert::TryInto;
use std::path::{Path, PathBuf};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Record {
    pub item: String,
    pub name: String,
    pub size: i64,
    pub crc32: u32,
    pub md5: [u8; 16],
    pub sha1: [u8; 20],
}

pub fn read_metadata_dir<P: AsRef<Path>>(path: P) -> Result<Vec<Record>, Error> {
    let mut results = vec![];

    for entry in std::fs::read_dir(path)? {
        results.extend(read_metadata(entry?.path())?);
    }

    results.sort();

    Ok(results)
}

pub fn read_metadata<P: AsRef<Path>>(path: P) -> Result<Vec<Record>, Error> {
    let contents = std::fs::read_to_string(&path)?;
    let doc = roxmltree::Document::parse(&contents)?;

    let item =
        extract_item_name(&path).ok_or_else(|| Error::InvalidPath(path.as_ref().to_path_buf()))?;

    let mut records: Vec<Record> = doc
        .root_element()
        .children()
        .filter_map(|node| {
            if node.has_tag_name("file") {
                node.attribute("name").map(|name| (name, node))
            } else {
                None
            }
        })
        .filter_map(|(name, node)| {
            let fields: HashMap<&str, &str> = node
                .children()
                .filter(|child| child.is_element())
                .filter_map(|child| child.text().map(|text| (child.tag_name().name(), text)))
                .collect();

            let format = fields.get("format");

            if format == Some(&"ZIP") || format == Some(&"TAR") {
                let size = fields
                    .get("size")
                    .and_then(|value| value.parse::<i64>().ok())?;
                let crc32 = fields
                    .get("crc32")
                    .and_then(|value| u32::from_str_radix(value, 16).ok())?;
                let md5 = fields
                    .get("md5")
                    .and_then(|value| hex::decode(value).ok())
                    .and_then(|value| value.try_into().ok())?;
                let sha1 = fields
                    .get("sha1")
                    .and_then(|value| hex::decode(value).ok())
                    .and_then(|value| value.try_into().ok())?;

                Some(Record {
                    item: item.to_string(),
                    name: name.to_string(),
                    size,
                    crc32,
                    md5,
                    sha1,
                })
            } else {
                None
            }
        })
        .collect();

    records.sort();

    Ok(records)
}

fn extract_item_name<P: AsRef<Path>>(path: &P) -> Option<&str> {
    let stem = path.as_ref().file_stem()?;
    let value = stem.to_str()?;
    value.strip_suffix("_files")
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("XML parsing error")]
    Roxmltree(#[from] roxmltree::Error),
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Invalid file path")]
    InvalidPath(PathBuf),
}
