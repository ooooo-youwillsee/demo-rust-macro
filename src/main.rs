#![feature(trace_macros)]
use std::collections::HashMap;

use demo_rust_macro::json;
use demo_rust_macro::json::Json;
use Json::{Array, Object, Str};

trace_macros!(true);

fn main() {
    test_json_marco();
}

fn test_json_marco() {
    let v = json!(null);
    println!("{:?}", v);
    assert_eq!(Json::Null, v);

    let v = json!("1");
    println!("{:?}", v);
    assert_eq!(Str("1".into()), v);

    let v = json!(["1", "2", "3"]);
    println!("{:?}", v);

    let arr = vec![Str("1".into()), Str("2".into()), Str("3".into())];
    assert_eq!(Array(arr), v);

    let v = json!({
            "k1": "v1",
            "k2": "v2",
            "k3": "v3"
        });
    println!("{:?}", v);

    let mut obj = HashMap::new();
    obj.insert("k1".to_string(), Str("v1".to_string()));
    obj.insert("k2".to_string(), Str("v2".to_string()));
    obj.insert("k3".to_string(), Str("v3".to_string()));
    assert_eq!(Object(obj), v);
}
