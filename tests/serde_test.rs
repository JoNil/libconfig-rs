extern crate libconfig_rs;

use serde::Deserialize;

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
    let res = libconfig_rs::serde::from_str::<Test1>(config).unwrap();

    assert!(res.a == 1);
    assert!(res.b == 2);
    assert!(res.c == 3.3);
}

#[derive(Deserialize)]
struct Test2(i32);

#[test]
fn test2() {
    let config = r#"config : ( 5 );"#;
    let res = libconfig_rs::serde::from_str::<Test2>(config).unwrap();

    assert!(res.0 == 5);
}

#[derive(Deserialize)]
struct Test3(i32, f32);

#[test]
fn test3() {
    let config = r#"config : ( 5, 1.0 );"#;
    let res = libconfig_rs::serde::from_str::<Test3>(config).unwrap();

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
    let res = libconfig_rs::serde::from_str::<Test4>(config).unwrap();

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
        let res = libconfig_rs::serde::from_str::<Test5>(config).unwrap();

        assert!(res.a == None);
    }

    {
        let config = r#"config : {
            a : ( 2 );
        };
        "#;
        let res = libconfig_rs::serde::from_str::<Test5>(config).unwrap();

        assert!(res.a == Some(2));
    }
}
