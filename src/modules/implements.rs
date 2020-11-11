use crate::modules::traits::{Transform, Ser};
use std::str::from_utf8;
use serde_json::from_str;
use serde::{Deserialize, Serialize};


impl Transform for Vec<u8> {
    fn translate(&self) -> &str {
        from_utf8(&self).unwrap()
    }
}

impl<'a>Ser<'a> for &'a str{
    fn make<T>(&self) ->  T
    where
        T: Deserialize<'a>,
        T: Serialize
    {
        from_str(self).unwrap()
    }
}