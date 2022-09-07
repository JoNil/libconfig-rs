# Simple libconfig parser

```rust
    let config = "config : { test : [1, 2, 3]; }";
    let res = libconfig_rs::from_str(config).unwrap();
    println!("{:#?}", res);
```