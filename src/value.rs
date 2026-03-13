use crate::{parser, printer};
use indexmap::IndexMap;
use nom::{
    Finish,
    error::{ErrorKind, ParseError},
};
use std::{fmt, str::FromStr};

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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "config : ")?;
        let mut res = String::new();
        printer::print(&mut res, self, 4);
        write!(f, "{res};")?;
        Ok(())
    }
}

impl FromStr for Value {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parser::root::<nom::error::Error<&str>>(input)
            .finish()
            .map(|(_, o)| o)
            .map_err(|e| format!("{e}"))
    }
}

impl Value {
    pub fn obj_from_str(input: &str) -> Result<IndexMap<String, Value>, nom::error::Error<&str>> {
        parser::root::<nom::error::Error<&str>>(input)
            .finish()
            .and_then(|(_, o)| match o {
                Value::Object(map) => Ok(map),
                _ => Err(nom::error::Error::from_error_kind(
                    "Config did not have a object in the root",
                    ErrorKind::Fail,
                )),
            })
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
