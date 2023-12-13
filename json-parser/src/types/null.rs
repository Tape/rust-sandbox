use super::JsonValue;

/// A JSON null.
pub struct JsonNull;

impl JsonNull {
    pub fn new() -> Self {
        JsonNull
    }
}

impl JsonValue for JsonNull {
    fn is_null(&self) -> bool {
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_assertions() {
        let json_null = JsonNull::new();
        assert_eq!(json_null.is_array(), false);
        assert_eq!(json_null.is_bool(), false);
        assert_eq!(json_null.is_null(), true);
        assert_eq!(json_null.is_number(), false);
        assert_eq!(json_null.is_object(), false);
        assert_eq!(json_null.is_string(), false);
    }

    #[test]
    fn test_type_conversions() {
        let json_null = JsonNull::new();
        assert_eq!(json_null.as_bool().is_err(), true);
        assert_eq!(json_null.as_float().is_err(), true);
        assert_eq!(json_null.as_int().is_err(), true);
        assert_eq!(json_null.as_string().is_err(), true);
    }
}
