extern crate libconfig_rs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestInteger {
    a: i8,
    b: i16,
    c: i32,
    d: i64,
}

#[test]
fn test_integer() {
    let test = TestInteger {
        a: -42,
        b: -2,
        c: 3,
        d: 4,
    };
    let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
    let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
    assert_eq!(test, der);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestUnsignedInteger {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
}

#[test]
fn test_unsigned_integer() {
    let test = TestUnsignedInteger {
        a: 42,
        b: 2,
        c: 3,
        d: 4,
    };
    let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
    let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
    assert_eq!(test, der);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestFloat {
    a: f32,
    b: f64,
}

#[test]
fn test_float() {
    let test = TestFloat { a: -3.0, b: 4.0 };
    let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
    let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
    assert_eq!(test, der);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestBool {
    a: bool,
    b: bool,
}

#[test]
fn test_bool() {
    let test = TestBool { a: true, b: false };
    let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
    let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
    assert_eq!(test, der);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestChar {
    a: char,
    b: char,
}

#[test]
fn test_char() {
    let test = TestChar { a: 'a', b: ' ' };
    let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
    let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
    assert_eq!(test, der);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestString {
    a: String,
    b: String,
}

#[test]
fn test_string() {
    let test = TestString {
        a: "".to_string(),
        b: "THIS IS A STRING".to_string(),
    };
    let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
    let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
    assert_eq!(test, der);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestArray {
    a: [i32; 2],
    b: [f32; 3],
    c: Vec<i32>,
    d: Vec<f32>,
}

#[test]
fn test_array() {
    let test = TestArray {
        a: [1, 2],
        b: [1.1, 2.2, 3.0],
        c: vec![1, 2],
        d: vec![1.1, 2.2, 3.0],
    };
    let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
    let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
    assert_eq!(test, der);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestOption {
    a: Option<()>,
    b: Option<i32>,
    c: Option<i32>,
}

#[test]
fn test_option() {
    let test = TestOption {
        a: Some(()),
        b: None,
        c: None,
    };
    let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
    let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
    assert_eq!(test, der);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestUnit {
    a: (),
}

#[test]
fn test_unit() {
    let test = TestUnit { a: () };
    let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
    let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
    assert_eq!(test, der);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestTuple {
    a: ((), i32),
    b: (Struct, i32),
    c: (f32, i32, bool),
}

#[test]
fn test_tuple() {
    let test = TestTuple {
        a: ((), 42),
        b: (Struct { a: 42, b: 42.2 }, 42),
        c: (0.5, 42, false),
    };
    let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
    let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
    assert_eq!(test, der);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestHashmap {
    a: HashMap<String, String>,
    b: HashMap<i32, String>,
    c: HashMap<String, i32>,
}

#[test]
fn test_hashmap() {
    let mut a = HashMap::new();
    let mut b = HashMap::new();
    let mut c = HashMap::new();
    a.insert("ka".to_string(), "va".to_string());
    b.insert(42, "vb".to_string());
    c.insert("kc".to_string(), 42);
    let test = TestHashmap { a, b, c };
    let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
    let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
    assert_eq!(test, der);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct UnitStruct;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Struct {
    a: i32,
    b: f64,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct StructInStruct {
    a: Struct,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestStruct {
    a: UnitStruct,
    b: Struct,
    c: StructInStruct,
}

#[test]
fn test_struct() {
    let test = TestStruct {
        a: UnitStruct,
        b: Struct { a: 0, b: 42.2 },
        c: StructInStruct {
            a: Struct { a: 0, b: 42.2 },
        },
    };
    let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
    let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
    assert_eq!(test, der);
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Enum {
    A,
    B(i32),
    C { a: i32, b: f32 },
    D(Struct),
    E(StructInStruct),
}
#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct TestEnumVariants {
    a: Enum,
    b: Enum,
    c: Enum,
    d: Enum,
    e: Enum,
}

#[test]
fn test_enum_variants() {
    let test = TestEnumVariants {
        a: Enum::A,
        b: Enum::B(42),
        c: Enum::C { a: 42, b: 42.2 },
        d: Enum::D(Struct { a: 0, b: 42.2 }),
        e: Enum::E(StructInStruct {
            a: Struct { a: 0, b: 42.2 },
        }),
    };
    let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
    let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
    assert_eq!(test, der);
}

#[test]
fn test_enum() {
    let test = Enum::E(StructInStruct {
        a: Struct { a: 0, b: 42.2 },
    });
    let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
    let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
    assert_eq!(test, der);
}
