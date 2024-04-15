# libconfig parser with serde support
```rust

    #[derive(Serialize, Deserialize)]
    struct TestInteger {
        a: i32,
    }

    let test = TestInteger {
        a: 42,
    };

    let ser = libconfig_rs::to_string(&test).unwrap();
    let der = libconfig_rs::from_str(&ser).unwrap();

    assert_eq!(test, der);


    let config = "config : { test : [1, 2, 3]; }";
    let res = libconfig_rs::Value::from_str(config).unwrap();
    println!("{:#?}", res);
```