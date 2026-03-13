extern crate libconfig_rs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    let res = libconfig_rs::from_str::<Test1>(config).unwrap();

    assert!(res.a == 1);
    assert!(res.b == 2);
    assert!(res.c == 3.3);
}

#[derive(Deserialize)]
struct Test2(i32);

#[test]
fn test2() {
    let config = r#"config : ( 5 );"#;
    let res = libconfig_rs::from_str::<Test2>(config).unwrap();

    assert!(res.0 == 5);
}

#[derive(Deserialize)]
struct Test3(i32, f32);

#[test]
fn test3() {
    let config = r#"config : ( 5, 1.0 );"#;
    let res = libconfig_rs::from_str::<Test3>(config).unwrap();

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
    let res = libconfig_rs::from_str::<Test4>(config).unwrap();

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
        let res = libconfig_rs::from_str::<Test5>(config).unwrap();

        assert!(res.a.is_none());
    }

    {
        let config = r#"config : {
            a : ( 2 );
        };
        "#;
        let res = libconfig_rs::from_str::<Test5>(config).unwrap();

        assert!(res.a == Some(2));
    }
}

#[derive(Deserialize)]
struct Test6 {
    #[serde(rename = "*a")]
    a: (),
}

#[test]
fn test6() {
    {
        let config = r#"config : {
            *a : ();
        };
        "#;
        libconfig_rs::from_str::<Test6>(config).unwrap();
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
        let t = libconfig_rs::from_str::<Test7>(config).unwrap();
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
        let t = libconfig_rs::from_str::<Test7>(config).unwrap();
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
        let t = libconfig_rs::from_str::<Test7>(config).unwrap();
        assert_eq!(t.a, TestEnum1::C);
        assert_eq!(t.b, TestEnum2::C("3".to_owned()));
        assert_eq!(t.c, TestEnum3::C { c: "3".to_owned() });
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Test8 {
    a: HashMap<i32, String>,
}

#[test]
fn test8() {
    {
        let config = r#"config : {
            a : ( (1, "one"), (2, "two") );
        };"#;
        let t = libconfig_rs::from_str::<Test8>(config).unwrap();
        assert_eq!(t.a[&1], "one");
        assert_eq!(t.a[&2], "two");
    }
}
