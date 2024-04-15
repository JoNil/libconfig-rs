use libconfig_rs::{ArrayType, Value};
use std::str::FromStr;

#[test]
fn test_empty() {
    let config = "config : {};";
    let res = Value::from_str(config).unwrap();
    assert_eq!(res, Value::Object(indexmap::IndexMap::new()));
}

#[test]
fn test_bool() {
    let config = "config : true;";
    let res = Value::from_str(config).unwrap();
    assert_eq!(res, Value::Bool(true))
}

#[test]
fn test_int() {
    let config = "config : 123;";
    let res = Value::from_str(config).unwrap();
    assert_eq!(res, Value::Int(123))
}

#[test]
fn test_long() {
    let config = "config : 4000000L;";
    let res = Value::from_str(config).unwrap();
    assert_eq!(res, Value::Int(4000000))
}

#[test]
fn test_float() {
    let config = "config : 123.1;";
    let res = Value::from_str(config).unwrap();
    assert_eq!(res, Value::Float(123.1))
}

#[test]
fn test_float_2() {
    let config = "config : 1e-5;";
    let res = Value::from_str(config).unwrap();
    assert_eq!(res, Value::Float(0.00001))
}

#[test]
fn test_string() {
    let config = "config : \"Test\";";
    let res = Value::from_str(config).unwrap();
    assert_eq!(res, Value::String(String::from("Test")))
}

#[test]
fn test_object() {
    let config = "config : { test : 123; };";
    let res = Value::from_str(config).unwrap();

    let mut inner = indexmap::IndexMap::new();
    inner.insert("test".into(), Value::Int(123));

    assert_eq!(res, Value::Object(inner))
}

#[test]
fn test_object_string() {
    let config = "config : { test : \"Test\"; };";
    let res = Value::from_str(config).unwrap();

    let mut inner = indexmap::IndexMap::new();
    inner.insert("test".into(), Value::String(String::from("Test")));

    assert_eq!(res, Value::Object(inner))
}

#[test]
fn test_list() {
    let config = "config : { test : (1, 2, 3); };";
    let res = Value::from_str(config).unwrap();

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
    let res = Value::from_str(config).unwrap();

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
    let res = Value::from_str(config).unwrap();
    assert!(matches!(res, Value::Object(_)))
}

#[test]
fn test_vproj_2() {
    let config = include_str!("../tests/2.vproj");
    let res = Value::from_str(config).unwrap();
    assert!(matches!(res, Value::Object(_)))
}
