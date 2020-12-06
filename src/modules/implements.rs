use crate::modules::traits::{Transform, Ser, Print};
use std::str::{from_utf8, SplitWhitespace};
use serde_json::from_str;
use serde::{Deserialize, Serialize};
use serde::export::fmt::Display;
use serde::export::Formatter;
use std::fmt;
use serde::de::Unexpected::Str;


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



impl Print for Vec<&str> {

    fn print(&self) {
        let mut str: String = String::new();
        for i in self{
           str+= &*(i.trim().to_owned() + " ");
        }
       println!("{}",str)
    }

    fn println(&self) {
       for i in self{
           println!("{}",i.trim())
       }
    }
}

