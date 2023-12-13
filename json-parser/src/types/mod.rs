pub use bool::*;
pub use null::*;
pub use number::*;
pub use string::*;

use crate::{ParserError, Result};

mod bool;
mod null;
mod number;
mod string;

/// Trait to represent a JSON value. Provides helpful methods for validating and converting values.
/// Types per spec: https://www.json.org/json-en.html
pub trait JsonValue {
    /// Extracts the boolean value if the value is a boolean.
    fn as_bool(&self) -> Result<bool> {
        Err(ParserError::Cast("bool"))
    }

    /// Returns the value as a float if the value is a number.
    fn as_float(&self) -> Result<f64> {
        Err(ParserError::Cast("float"))
    }

    /// Returns the value as an integer if the value is a number.
    fn as_int(&self) -> Result<i64> {
        Err(ParserError::Cast("int"))
    }

    /// Returns the value as a string if the value is a string.
    fn as_string(&self) -> Result<&str> {
        Err(ParserError::Cast("string"))
    }

    /// Returns true if the value is an array.
    fn is_array(&self) -> bool {
        return false;
    }

    /// Returns true if the value is a boolean.
    fn is_bool(&self) -> bool {
        return false;
    }

    /// Returns true if the value is null.
    fn is_null(&self) -> bool {
        return false;
    }

    /// Returns true if the value is a number.
    fn is_number(&self) -> bool {
        return false;
    }

    /// Returns true if the value is an object.
    fn is_object(&self) -> bool {
        return false;
    }

    /// Returns true if the value is a string.
    fn is_string(&self) -> bool {
        return false;
    }
}
