use nom::{error::VerboseError, Finish};
use std::collections::HashMap;

mod parser;

#[derive(Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

impl Value {
    #[inline]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(v) => Some(*v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(v) => Some(*v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(v) => Some(*v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(v) => Some(&*v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_vec(&self) -> Option<&Vec<Value>> {
        match self {
            Value::Array(v) => Some(&*v),
            _ => None,
        }
    }

    #[inline]
    pub fn as_obj(&self) -> Option<&HashMap<String, Value>> {
        match self {
            Value::Object(v) => Some(&*v),
            _ => None,
        }
    }
}

pub fn from_str(input: &str) -> Result<Value, VerboseError<&str>> {
    parser::root::<VerboseError<&str>>(input)
        .finish()
        .map(|(_, o)| o)
}

#[cfg(test)]
mod tests {
    use super::Value;
    use std::collections::HashMap;

    #[test]
    fn test_empty() {
        let config = "config : {};";
        let res = super::from_str(config).unwrap();
        assert_eq!(res, Value::Object(HashMap::new()))
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
    fn test_float() {
        let config = "config : 123.1;";
        let res = dbg!(super::from_str(config).unwrap());
        assert_eq!(res, Value::Float(123.1))
    }

    #[test]
    fn test_string() {
        let config = "config : \"Test\";";
        let res = dbg!(super::from_str(config).unwrap());
        assert_eq!(res, Value::String(String::from("Test")))
    }

    #[test]
    fn test_object() {
        let config = "config : { test : 123; };";
        let res = dbg!(super::from_str(config).unwrap());

        let mut inner = HashMap::new();
        inner.insert("test".into(), Value::Int(123));

        assert_eq!(res, Value::Object(inner))
    }

    #[test]
    fn test_object_string() {
        let config = "config : { test : \"Test\"; };";
        let res = dbg!(super::from_str(config).unwrap());

        let mut inner = HashMap::new();
        inner.insert("test".into(), Value::String(String::from("Test")));

        assert_eq!(res, Value::Object(inner))
    }

    #[test]
    fn test_list() {
        let config = "config : { test : (1, 2, 3); };";
        let res = dbg!(super::from_str(config).unwrap());

        let mut inner = HashMap::new();
        inner.insert(
            "test".into(),
            Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]),
        );

        assert_eq!(res, Value::Object(inner))
    }

    #[test]
    fn test_array() {
        let config = "config : { test : [1, 2, 3]; };";
        let res = dbg!(super::from_str(config).unwrap());

        let mut inner = HashMap::new();
        inner.insert(
            "test".into(),
            Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]),
        );

        assert_eq!(res, Value::Object(inner))
    }

    #[test]
    fn test_vproj_1() {
        let config = include_str!("../test/1.vproj");
        let res = super::from_str(config).unwrap();
        assert!(matches!(res, Value::Object(_)))
    }

    #[test]
    fn test_vproj_2() {
        let config = include_str!("../test/2.vproj");
        let res = super::from_str(config).unwrap();
        assert!(matches!(res, Value::Object(_)))
    }
}
