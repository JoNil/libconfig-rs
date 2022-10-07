use indexmap::IndexMap;
use nom::{
    error::{ErrorKind, ParseError, VerboseError},
    Finish,
};
use std::fmt::Write;

mod parser;
mod printer;
pub mod serde;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArrayType {
    Array,
    List,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>, ArrayType),
    Object(IndexMap<String, Value>),
}

impl Value {
    #[inline]
    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            Value::Bool(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_bool_mut(&mut self) -> Option<&mut bool> {
        match self {
            Value::Bool(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_int(&self) -> Option<&i64> {
        match self {
            Value::Int(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_int_mut(&mut self) -> Option<&mut i64> {
        match self {
            Value::Int(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_float(&self) -> Option<&f64> {
        match self {
            Value::Float(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_float_mut(&mut self) -> Option<&mut f64> {
        match self {
            Value::Float(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_str_mut(&mut self) -> Option<&mut String> {
        match self {
            Value::String(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_vec(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(v, _) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_vec_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::Array(v, _) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_obj(&self) -> Option<&IndexMap<String, Value>> {
        match self {
            Value::Object(v) => Some(v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_obj_mut(&mut self) -> Option<&mut IndexMap<String, Value>> {
        match self {
            Value::Object(v) => Some(v),
            _ => None,
        }
    }
}

pub fn from_str(input: &str) -> Result<Value, VerboseError<&str>> {
    parser::root::<VerboseError<&str>>(input)
        .finish()
        .map(|(_, o)| o)
}

pub fn obj_from_str(input: &str) -> Result<IndexMap<String, Value>, VerboseError<&str>> {
    parser::root::<VerboseError<&str>>(input)
        .finish()
        .and_then(|(_, o)| match o {
            Value::Object(map) => Ok(map),
            _ => Err(VerboseError::from_error_kind(
                "Config did not have a object in the root",
                ErrorKind::Fail,
            )),
        })
}

pub fn to_libconfig_str(value: &Value) -> String {
    let mut res = String::new();
    write!(&mut res, "config : ").unwrap();
    printer::print(&mut res, value, 4);
    write!(&mut res, ";").unwrap();
    res
}

#[cfg(test)]
mod tests {
    use super::Value;
    use crate::ArrayType;

    #[test]
    fn test_empty() {
        let config = "config : {};";
        let res = super::from_str(config).unwrap();
        assert_eq!(res, Value::Object(indexmap::IndexMap::new()))
    }

    #[test]
    fn test_bool() {
        let config = "config : true;";
        let res = super::from_str(config).unwrap();
        assert_eq!(res, Value::Bool(true))
    }

    #[test]
    fn test_int() {
        let config = "config : 123;";
        let res = super::from_str(config).unwrap();
        assert_eq!(res, Value::Int(123))
    }

    #[test]
    fn test_long() {
        let config = "config : 4000000L;";
        let res = super::from_str(config).unwrap();
        assert_eq!(res, Value::Int(4000000))
    }

    #[test]
    fn test_float() {
        let config = "config : 123.1;";
        let res = super::from_str(config).unwrap();
        assert_eq!(res, Value::Float(123.1))
    }

    #[test]
    fn test_float_2() {
        let config = "config : 1e-5;";
        let res = super::from_str(config).unwrap();
        assert_eq!(res, Value::Float(0.00001))
    }

    #[test]
    fn test_string() {
        let config = "config : \"Test\";";
        let res = super::from_str(config).unwrap();
        assert_eq!(res, Value::String(String::from("Test")))
    }

    #[test]
    fn test_object() {
        let config = "config : { test : 123; };";
        let res = super::from_str(config).unwrap();

        let mut inner = indexmap::IndexMap::new();
        inner.insert("test".into(), Value::Int(123));

        assert_eq!(res, Value::Object(inner))
    }

    #[test]
    fn test_object_string() {
        let config = "config : { test : \"Test\"; };";
        let res = super::from_str(config).unwrap();

        let mut inner = indexmap::IndexMap::new();
        inner.insert("test".into(), Value::String(String::from("Test")));

        assert_eq!(res, Value::Object(inner))
    }

    #[test]
    fn test_list() {
        let config = "config : { test : (1, 2, 3); };";
        let res = super::from_str(config).unwrap();

        let mut inner = indexmap::IndexMap::new();
        inner.insert(
            "test".into(),
            Value::Array(
                vec![Value::Int(1), Value::Int(2), Value::Int(3)],
                ArrayType::List,
            ),
        );

        assert_eq!(res, Value::Object(inner))
    }

    #[test]
    fn test_array() {
        let config = "config : { test : [1, 2, 3]; };";
        let res = super::from_str(config).unwrap();

        let mut inner = indexmap::IndexMap::new();
        inner.insert(
            "test".into(),
            Value::Array(
                vec![Value::Int(1), Value::Int(2), Value::Int(3)],
                ArrayType::Array,
            ),
        );

        assert_eq!(res, Value::Object(inner))
    }

    #[test]
    fn test_vproj_1() {
        let config = include_str!("../tests/1.vproj");
        let res = super::from_str(config).unwrap();
        assert!(matches!(res, Value::Object(_)))
    }

    #[test]
    fn test_vproj_2() {
        let config = include_str!("../tests/2.vproj");
        let res = super::from_str(config).unwrap();
        assert!(matches!(res, Value::Object(_)))
    }
}
