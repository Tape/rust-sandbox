use super::JsonValue;

/// A JSON number wrapping an integer.
pub struct JsonInteger(i64);

impl JsonInteger {
    /// Creates a new JSON number with an integer value.
    pub fn new(value: i64) -> Self {
        JsonInteger(value)
    }
}

impl JsonValue for JsonInteger {
    fn as_float(&self) -> crate::Result<f64> {
        Ok(self.0 as f64)
    }

    fn as_int(&self) -> crate::Result<i64> {
        Ok(self.0)
    }

    fn is_number(&self) -> bool {
        return true;
    }
}

/// A JSON number wrapping a float.
pub struct JsonFloat(f64);

impl JsonFloat {
    /// Creates a new JSON number with a float value.
    pub fn new(value: f64) -> Self {
        JsonFloat(value)
    }
}

impl JsonValue for JsonFloat {
    fn as_float(&self) -> crate::Result<f64> {
        Ok(self.0)
    }

    fn as_int(&self) -> crate::Result<i64> {
        Ok(self.0 as i64)
    }

    fn is_number(&self) -> bool {
        return true;
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn test_int_type_assertions() {
        let json_number = JsonInteger::new(42);
        assert_eq!(json_number.is_array(), false);
        assert_eq!(json_number.is_bool(), false);
        assert_eq!(json_number.is_null(), false);
        assert_eq!(json_number.is_number(), true);
        assert_eq!(json_number.is_object(), false);
        assert_eq!(json_number.is_string(), false);
    }

    #[test]
    fn test_int_type_conversions() {
        let json_number = JsonInteger::new(42);
        assert_eq!(json_number.as_bool().is_err(), true);
        assert_eq!(json_number.as_float(), Ok(42.0));
        assert_eq!(json_number.as_int(), Ok(42));
        assert_eq!(json_number.as_string().is_err(), true);
    }

    #[test]
    fn test_float_type_assertions() {
        let json_number = JsonFloat::new(42.0);
        assert_eq!(json_number.is_array(), false);
        assert_eq!(json_number.is_bool(), false);
        assert_eq!(json_number.is_null(), false);
        assert_eq!(json_number.is_number(), true);
        assert_eq!(json_number.is_object(), false);
        assert_eq!(json_number.is_string(), false);
    }

    #[test]
    fn test_float_type_conversions() {
        let json_number = JsonFloat::new(42.5);
        assert_eq!(json_number.as_bool().is_err(), true);
        assert_eq!(json_number.as_float(), Ok(42.5));
        assert_eq!(json_number.as_int(), Ok(42));
        assert_eq!(json_number.as_string().is_err(), true);
    }
}
