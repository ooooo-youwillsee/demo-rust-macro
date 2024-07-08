use std::collections::HashMap;

use crate::json::Json::{Number, Str};

#[derive(Debug, PartialEq)]
pub enum Json {
    Null,
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

impl From<String> for Json {
    fn from(value: String) -> Self {
        Str(value)
    }
}

impl From<&str> for Json {
    fn from(value: &str) -> Self {
        Str(value.into())
    }
}

impl From<i32> for Json {
    fn from(value: i32) -> Self {
        Number(value as f64)
    }
}

#[macro_export]
macro_rules! json {
    (null) => {Json::Null};
    ([ $( $value:tt ),* ]) => {
        Json::Array(
            vec![$( json!($value) ),*]
        )
    };
    ({ $( $key:tt : $value:tt ),* }) => {
        Json::Object(
            vec![$( ($key.to_string(), json!($value)) ),*].into_iter().collect()
        )
    };
    ($value:tt) => {
        Json::from($value)
    }
}
