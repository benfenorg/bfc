use crate::SecretSharingError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
/// Enhanced internal data structure
/// Note!!! Must match the names in the JSON file
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct JsonData {
    #[serde(rename = "numericKeys", deserialize_with = "parse_numeric_keys")]
    numeric_keys: HashMap<u64, u64>,
}

/// Core parsing function - enhanced error handling
fn parse_numeric_keys<'de, D>(deserializer: D) -> Result<HashMap<u64, u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let str_map: HashMap<String, u64> = HashMap::deserialize(deserializer)?;

    str_map
        .into_iter()
        .map(|(k, v)| {
            k.parse::<u64>()
                .map_err(|_| serde::de::Error::custom(format!("Invalid u64 key: '{}'", k)))
                .and_then(|num| {
                    if num == 0 {
                        Err(serde::de::Error::custom("Key cannot be zero".to_string()))
                    } else {
                        Ok((num, v))
                    }
                })
        })
        .collect()
}

/// Public interface: Read JSON from file and parse as HashMap<u64, u64>
pub fn read_numeric_json<P: AsRef<Path>>(path: P) -> Result<HashMap<u64, u64>, SecretSharingError> {
    // Check if file exists
    if !path.as_ref().exists() {
        return Err(SecretSharingError::FileNotFound(
            path.as_ref().to_string_lossy().into_owned(),
        ));
    }

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Validate JSON structure
    let data: JsonData = serde_json::from_str(&contents)?;

    // Validate data validity
    if data.numeric_keys.is_empty() {
        return Err(SecretSharingError::ValidationFailed(
            "Empty key-value pairs".into(),
        ));
    }

    Ok(data.numeric_keys)
}

/// Public interface: Parse JSON string directly
pub fn parse_numeric_json(json_str: &str) -> Result<HashMap<u64, u64>, SecretSharingError> {
    let data: JsonData = serde_json::from_str(json_str)?;
    Ok(data.numeric_keys)
}

/// New: Get corresponding mask value by key
pub fn get_mask_by_key(data: &HashMap<u64, u64>, key: u64) -> Result<u64, SecretSharingError> {
    data.get(&key)
        .copied()
        .ok_or_else(|| SecretSharingError::InvalidKey(key))
}

/// New: Write HashMap to JSON file
pub fn write_numeric_json<P: AsRef<Path>>(
    path: P,
    data: &HashMap<u64, u64>,
) -> Result<(), SecretSharingError> {
    let json_data = JsonData {
        numeric_keys: data.clone(),
    };
    let json_str = serde_json::to_string_pretty(&json_data)?;
    let mut file = File::create(path)?;
    file.write_all(json_str.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_valid_json() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(
            file,
            r#"{{"numericKeys": {{"1": 123456789, "2": 987654321, "3": 555555555}}}}"#
        )
        .unwrap();

        let result = read_numeric_json(file.path());
        assert!(result.is_ok());
        let data = result.unwrap();
        assert_eq!(data.len(), 3);
        assert_eq!(data[&1], 123456789);
    }

    #[test]
    fn test_parse_invalid_key() {
        let json_str = r#"{"numericKeys": {"abc": 123}}"#;
        let result = parse_numeric_json(json_str);
        assert!(matches!(result, Err(SecretSharingError::Json(_))));
    }

    #[test]
    fn test_get_mask_by_key() {
        let mut data = HashMap::new();
        data.insert(1, 100);
        data.insert(2, 200);

        assert_eq!(get_mask_by_key(&data, 1).unwrap(), 100);
        assert!(matches!(
            get_mask_by_key(&data, 3),
            Err(SecretSharingError::InvalidKey(3))
        ));
    }
}
