fn main() {
    let config = include_str!("../../test/2.vproj");
    let res = libconfig_rs::from_str(config).unwrap();
    println!("{:#?}", res);
}
