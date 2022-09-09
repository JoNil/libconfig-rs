use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Test {
    a: i32,
    b: i32,
    c: f32,
}

fn main() {
    let config = r#"config : {
        a : 1;
        b : 2;
        c : 3.3;
    };
    "#;
    let res = libconfig_rs::serde::from_str::<Test>(config).unwrap();

    assert!(res.a == 1);
    assert!(res.b == 2);
    assert!(res.c == 3.3);
}
