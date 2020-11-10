use crate::modules::traits::{Transform, Ser};
use std::str::from_utf8;
use serde_json::from_str;
use serde::{Deserialize, Serialize};


impl Transform for Vec<u8> {
    fn translate(&self) -> String {
        from_utf8(&self).unwrap().to_string()
    }
}

impl Ser for String{
    fn make<'a ,T>(&self) ->  T
    where
        T: Deserialize<'a>,
        T: Serialize
    {
        from_str(&self).unwrap()
    }
}