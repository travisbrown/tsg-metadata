use chrono::{Datelike, NaiveDate, NaiveDateTime, Timelike};
use regex::Regex;

lazy_static::lazy_static! {
    static ref BZ2_FILE_NAME_RE: Regex =
      Regex::new(r#"^(\d{4})/(\d{2})/(\d{2})/(\d{2})/(\d{2})\.json\.bz2$"#).unwrap();
}

pub fn extract_timestamp(name: &str) -> Result<NaiveDateTime, Error> {
    let timestamp = BZ2_FILE_NAME_RE
        .captures(name)
        .and_then(|captures| {
            let year = captures.get(1).and_then(|m| m.as_str().parse().ok())?;
            let month = captures.get(2).and_then(|m| m.as_str().parse().ok())?;
            let day = captures.get(3).and_then(|m| m.as_str().parse().ok())?;
            let hour = captures.get(4).and_then(|m| m.as_str().parse().ok())?;
            let minute = captures.get(5).and_then(|m| m.as_str().parse().ok())?;
            let date = NaiveDate::from_ymd_opt(year, month, day)?;

            date.and_hms_opt(hour, minute, 0)
        })
        .ok_or_else(|| Error::InvalidFormat(name.to_string()))?;

    let timestamp_s = timestamp.timestamp();
    let round_tripped =
        NaiveDateTime::from_timestamp_opt(timestamp_s, 0).filter(|result| *result == timestamp);

    round_tripped.ok_or(Error::InvalidDateTime(timestamp))
}

pub fn to_name(timestamp: NaiveDateTime) -> String {
    format!(
        "{:04}/{:02}/{:02}/{:02}/{:02}.json.bz2",
        timestamp.year(),
        timestamp.month(),
        timestamp.day(),
        timestamp.hour(),
        timestamp.minute()
    )
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid format")]
    InvalidFormat(String),
    #[error("Invalid date / time")]
    InvalidDateTime(NaiveDateTime),
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    #[test]
    fn extract_timestamp_good() {
        let expected = NaiveDate::from_ymd(2021, 1, 1).and_hms(10, 29, 0);
        let result = super::extract_timestamp("2021/01/01/10/29.json.bz2").unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn extract_timestamp_bad_date() {
        let result = super::extract_timestamp("2021/13/01/10/29.json.bz2");

        assert!(result.is_err());
    }

    #[test]
    fn extract_timestamp_bad_extension() {
        let result = super::extract_timestamp("2021/01/01/10/29.json.bz");

        assert!(result.is_err());
    }
}
