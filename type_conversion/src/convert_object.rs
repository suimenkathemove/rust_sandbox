use serde::{Deserialize, Serialize};

pub fn convert_object<T, U>(value: &T) -> Result<U, serde_json::Error>
where
    T: Serialize,
    U: for<'de> Deserialize<'de>,
{
    let json_string = serde_json::to_string(value)?;
    let object = serde_json::from_str(&json_string)?;
    Ok(object)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Foo {
        foo: i32,
        bar: i32,
    }

    type Bar = HashMap<String, i32>;

    #[test]
    fn struct_to_hash_map() {
        let expected = Bar::from([("foo".to_string(), 1), ("bar".to_string(), 2)]);

        let foo = Foo { foo: 1, bar: 2 };
        let actual = convert_object(&foo).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn hash_map_to_struct() {
        let expected = Foo { foo: 1, bar: 2 };

        let bar = Bar::from([("foo".to_string(), 1), ("bar".to_string(), 2)]);
        let actual = convert_object(&bar).unwrap();

        assert_eq!(expected, actual);
    }
}
