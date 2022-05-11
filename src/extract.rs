use regex::Regex;
use serde_json::Value;

pub fn find_users(value: &Value, acc: &mut Vec<(u64, String)>) -> Result<(), Error> {
    match value {
        Value::Array(values) => {
            for value in values {
                find_users(&value, acc)?;
            }
            Ok(())
        }
        Value::Object(fields) => {
            if let Some(screen_name_value) = fields.get("screen_name") {
                let screen_name = screen_name_value
                    .as_str()
                    .ok_or_else(|| Error::InvalidScreenNameField(screen_name_value.clone()))?;

                match fields.get("id_str") {
                    Some(id_str_value) => {
                        let id = id_str_value
                            .as_str()
                            .and_then(|id_str| id_str.parse::<u64>().ok())
                            .ok_or_else(|| Error::InvalidIdStrField(id_str_value.clone()))?;

                        acc.push((id, screen_name.to_string()));
                    }
                    None => {
                        return Err(Error::MissingIdStrField(value.clone()));
                    }
                }
            }
            for value in fields.values() {
                find_users(&value, acc)?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("screen_name without id_str")]
    MissingIdStrField(Value),
    #[error("Invalid id_str")]
    InvalidIdStrField(Value),
    #[error("Invalid screen_name")]
    InvalidScreenNameField(Value),
}
