use super::JsonValue;

/// A JSON boolean.
pub struct JsonBool(bool);

impl JsonBool {
    pub fn new(value: bool) -> Self {
        JsonBool(value)
    }
}

impl JsonValue for JsonBool {
    fn as_bool(&self) -> crate::Result<bool> {
        Ok(self.0)
    }

    fn is_bool(&self) -> bool {
        return true;
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_type_assertions() {
        let json_bool = JsonBool::new(true);
        assert_eq!(json_bool.is_array(), false);
        assert_eq!(json_bool.is_bool(), true);
        assert_eq!(json_bool.is_null(), false);
        assert_eq!(json_bool.is_number(), false);
        assert_eq!(json_bool.is_object(), false);
        assert_eq!(json_bool.is_string(), false);
    }

    #[test]
    fn test_type_conversions() {
        let json_bool = JsonBool::new(true);
        assert_eq!(json_bool.as_bool(), Ok(true));
        assert_eq!(json_bool.as_float().is_err(), true);
        assert_eq!(json_bool.as_int().is_err(), true);
        assert_eq!(json_bool.as_string().is_err(), true);
    }
}
