use crate::modules::traits::{Transform, Ser, Print};
use std::str::{from_utf8};
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



impl Print for Vec<&str> {

    fn print(&self)  -> &Self{
        let mut str: String = String::new();
        for i in self{
           str+= &*(i.trim().to_owned() + " ");
        }

        &self
    }

    fn println(&self)  -> &Self{
       for i in self{
           println!("{}",i.trim())
       }
        &self
    }
}

