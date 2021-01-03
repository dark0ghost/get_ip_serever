use crate::modules::traits::{Transform, Ser, Print, Route};
use std::str::{from_utf8};
use serde_json::from_str;
use serde::{Deserialize, Serialize};
use crate::modules::handler::Handler;


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
       println!("{}",str);
        &self
    }

    fn println(&self)  -> &Self{
       for i in self{
           println!("{}",i.trim())
       }
        &self
    }
}


impl Route for Handler {
    fn add_route<T>(&self, _route: Vec<T>) {
        unimplemented!()
    }
}
