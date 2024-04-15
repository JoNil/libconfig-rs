use crate::{parser, printer};
use indexmap::IndexMap;
use nom::{
    error::{ErrorKind, ParseError, VerboseError},
    Finish,
};
use std::fmt::Write;

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
    #[allow(clippy::should_implement_trait)]
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

    pub fn to_string(&self) -> String {
        let mut res = String::new();
        write!(&mut res, "config : ").unwrap();
        printer::print(&mut res, self, 4);
        write!(&mut res, ";").unwrap();
        res
    }

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
