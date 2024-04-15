use std::fs;

fn main() {
    let config = include_str!("../../tests/2.vproj");
    let res = libconfig_rs::Value::from_str(config).unwrap();
    let new = res.to_string();
    fs::write("test/2_new.vproj", new).unwrap();
}
