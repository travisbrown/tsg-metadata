use std::io::{Read, Seek};

pub mod tar;
pub mod zip;

pub trait Archive {
    type Entry;
    type Error;

    fn list_entries(&mut self) -> Result<Vec<Self::Entry>, Self::Error>;
    fn extract_bz2(&mut self, target: Self::Entry) -> Result<Option<Vec<String>>, Self::Error>;
}
