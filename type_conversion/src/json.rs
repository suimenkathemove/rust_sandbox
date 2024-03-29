use serde::{de::Deserialize, Serialize};
use serde_json::Error;

pub fn from_json<'a, T: Deserialize<'a>>(json: &'a str) -> Result<T, Error> {
    let value = serde_json::from_str(json)?;
    Ok(value)
}

pub fn to_json<T: Serialize>(value: &T) -> Result<String, Error> {
    let json = serde_json::to_string(value)?;
    Ok(json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn json_to_hash_map() {
        let expected = HashMap::from([("foo".to_string(), "bar".to_string())]);
        let actual: HashMap<String, String> = from_json(r#"{ "foo": "bar" }"#).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn hash_map_to_json() {
        let expected = r#"{"foo":"bar"}"#;
        let actual = to_json(&HashMap::from([("foo", "bar")])).unwrap();
        assert_eq!(expected, actual);
    }
}
