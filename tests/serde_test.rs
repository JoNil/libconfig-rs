extern crate libconfig_rs;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Test1 {
    a: i32,
    b: i32,
    c: f32,
}

#[test]
fn test1() {
    let config = r#"config : {
        a : 1;
        b : 2;
        c : 3.3;
    };
    "#;
    let res = libconfig_rs::serde::deserialize::from_str::<Test1>(config).unwrap();

    assert!(res.a == 1);
    assert!(res.b == 2);
    assert!(res.c == 3.3);
}

#[derive(Deserialize)]
struct Test2(i32);

#[test]
fn test2() {
    let config = r#"config : ( 5 );"#;
    let res = libconfig_rs::serde::deserialize::from_str::<Test2>(config).unwrap();

    assert!(res.0 == 5);
}

#[derive(Deserialize)]
struct Test3(i32, f32);

#[test]
fn test3() {
    let config = r#"config : ( 5, 1.0 );"#;
    let res = libconfig_rs::serde::deserialize::from_str::<Test3>(config).unwrap();

    assert!(res.0 == 5);
    assert!(res.1 == 1.0);
}

#[derive(Deserialize)]
struct Test4 {
    a: Vec<i32>,
}

#[test]
fn test4() {
    let config = r#"config : {
        a : ( 1, 2, 3 );
    };
    "#;
    let res = libconfig_rs::serde::deserialize::from_str::<Test4>(config).unwrap();

    assert!(res.a[0] == 1);
    assert!(res.a[1] == 2);
    assert!(res.a[2] == 3);
}

#[derive(Deserialize)]
struct Test5 {
    a: Option<i32>,
}

#[test]
fn test5() {
    {
        let config = r#"config : {
            a : ( );
        };
        "#;
        let res = libconfig_rs::serde::deserialize::from_str::<Test5>(config).unwrap();

        assert!(res.a.is_none());
    }

    {
        let config = r#"config : {
            a : ( 2 );
        };
        "#;
        let res = libconfig_rs::serde::deserialize::from_str::<Test5>(config).unwrap();

        assert!(res.a == Some(2));
    }
}

#[derive(Deserialize)]
struct Test6 {
    _a: (),
}

#[test]
fn test6() {
    {
        let config = r#"config : {
            _a : ();
        };
        "#;
        libconfig_rs::serde::deserialize::from_str::<Test6>(config).unwrap();
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
enum TestEnum1 {
    A,
    B,
    C,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum TestEnum2 {
    A(i32),
    B(f32),
    C(String),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum TestEnum3 {
    A { a: i32 },
    B { b: f32 },
    C { c: String },
}

#[derive(Serialize, Deserialize)]
struct Test7 {
    a: TestEnum1,
    b: TestEnum2,
    c: TestEnum3,
}

#[test]
fn test7() {
    {
        let config = r#"config : {
            a : "A";
            b : {
                A : ( 1 );
            };
            c : {
                A : { a : 1; };
            };
        };"#;
        let t = libconfig_rs::serde::deserialize::from_str::<Test7>(config).unwrap();
        assert_eq!(t.a, TestEnum1::A);
        assert_eq!(t.b, TestEnum2::A(1));
        assert_eq!(t.c, TestEnum3::A { a: 1 });
    }

    {
        let config = r#"config : {
            a : "B";
            b : {
                B : ( 2.0 );
            };
            c : {
                B : { b : 2.0; };
            };
        };"#;
        let t = libconfig_rs::serde::deserialize::from_str::<Test7>(config).unwrap();
        assert_eq!(t.a, TestEnum1::B);
        assert_eq!(t.b, TestEnum2::B(2.0));
        assert_eq!(t.c, TestEnum3::B { b: 2.0 });
    }

    {
        let config = r#"config : {
            a : "C";
            b : {
                C : ( "3" );
            };
            c : {
                C : { c : "3"; };
            };
        };"#;
        let t = libconfig_rs::serde::deserialize::from_str::<Test7>(config).unwrap();
        assert_eq!(t.a, TestEnum1::C);
        assert_eq!(t.b, TestEnum2::C("3".to_owned()));
        assert_eq!(t.c, TestEnum3::C { c: "3".to_owned() });
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Test8 {
    a: i8,
    b: i16,
    c: i32,
    d: i64,
}

#[test]
fn test8() {
    {
        let test = Test8 {
            a: -42,
            b: -2,
            c: 3,
            d: 4,
        };
        let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
        println!("{}", ser);
        let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
        assert_eq!(test, der);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Test9 {
    a: u8,
    b: u16,
    c: u32,
    d: u64,
}

#[test]
fn test9() {
    {
        let test = Test9 {
            a: 42,
            b: 2,
            c: 3,
            d: 4,
        };
        let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
        println!("{}", ser);
        let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
        assert_eq!(test, der);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Test10 {
    c: f32,
    d: f64,
}

#[test]
fn test10() {
    {
        let test = Test10 { c: -3.0, d: 4.0 };
        let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
        println!("{}", ser);
        let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
        assert_eq!(test, der);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Test11 {
    a: Test10,
    b: Test9,
    c: Test8,
}

#[test]
fn test11() {
    {
        let test = Test11 {
            a: Test10 { c: -3.0, d: 4.0 },
            b: Test9 {
                a: 42,
                b: 2,
                c: 3,
                d: 4,
            },
            c: Test8 {
                a: -42,
                b: -2,
                c: 3,
                d: 4,
            },
        };
        let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
        println!("{}", ser);
        let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
        assert_eq!(test, der);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Test12 {
    a: TestEnum1,
    b: TestEnum2,
    c: TestEnum3,
}

#[test]
fn test12() {
    {
        let test = Test12 {
            a: TestEnum1::A,
            b: TestEnum2::A(42),
            c: TestEnum3::A { a: 32 },
        };
        let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
        println!("{}", ser);
        // TODO: deserialize not working for enums
        // let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
        // assert_eq!(test, der);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Test13 {
    a: i8,
    b: i16,
    c: i32,
    d: i64,
    e: u8,
    f: u16,
    g: u32,
    h: u64,
    i: f32,
    j: f64,
    k: String,
    l: char,
    m: bool,
    n: [i32; 3],
    o: [f32; 2],
    p: Option<i32>,
    q: Option<i32>,
    r: (),
    s: UnitStruct,
    t: Struct,
    u: StructInStruct,
    v: Enum,
    x: Enum,
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
enum Enum {
    A,
    B(i32),
    C { a: i32, b: f32 },
    D(Struct),
}

#[test]
fn test13() {
    {
        let test = Test13 {
            a: 0,
            b: 1,
            c: -1,
            d: 42,
            e: 0,
            f: 1,
            g: 42,
            h: 84,
            i: 0.02,
            j: -42.04,
            k: "THIS IS A STRING".to_string(),
            l: 'c',
            m: true,
            n: [42; 3],
            o: [42.0; 2],
            p: Some(42),
            q: None,
            r: (),
            s: UnitStruct,
            t: Struct { a: 0, b: 42.2 },
            u: StructInStruct {
                a: Struct { a: 0, b: 42.2 },
            },
            v: Enum::A,
            x: Enum::B(32),
        };
        let ser = libconfig_rs::serde::serialize::to_string(&test).unwrap();
        println!("{}", ser);
        let der = libconfig_rs::serde::deserialize::from_str(&ser).unwrap();
        assert_eq!(test, der);
    }
}
