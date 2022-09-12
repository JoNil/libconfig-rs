use std::fs;

fn main() {
    let config = include_str!("../../tests/2.vproj");
    let res = libconfig_rs::from_str(config).unwrap();
    let new = libconfig_rs::to_libconfig_str(&res);
    fs::write("test/2_new.vproj", new).unwrap();
}
