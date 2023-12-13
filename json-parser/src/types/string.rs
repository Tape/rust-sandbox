use super::JsonValue;

/// A JSON string.
pub struct JsonString(String);

impl JsonString {
    pub fn new(value: &str) -> Self {
        JsonString(value.to_string())
    }
}

impl JsonValue for JsonString {
    fn as_string(&self) -> crate::Result<&str> {
        Ok(&self.0)
    }

    fn is_string(&self) -> bool {
        return true;
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_type_assertions() {
        let json_string = JsonString::new("test");
        assert_eq!(json_string.is_array(), false);
        assert_eq!(json_string.is_bool(), false);
        assert_eq!(json_string.is_null(), false);
        assert_eq!(json_string.is_number(), false);
        assert_eq!(json_string.is_object(), false);
        assert_eq!(json_string.is_string(), true);
    }

    #[test]
    fn test_type_conversions() {
        let json_string = JsonString::new("test");
        assert_eq!(json_string.as_bool().is_err(), true);
        assert_eq!(json_string.as_float().is_err(), true);
        assert_eq!(json_string.as_int().is_err(), true);
        assert_eq!(json_string.as_string(), Ok("test"));
    }
}
