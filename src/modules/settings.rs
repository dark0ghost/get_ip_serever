extern crate tokio;
use std::fmt;
use serde::{
    Serialize,
    Deserialize
};
use self::tokio::prelude::io::AsyncReadExt;
use crate::modules::traits::{Transform, Ser};
use std::error::Error;


#[derive(Serialize, Deserialize)]
pub struct Settings {
    ip:  String,
    port: i32
}

impl<'a>  Settings {

    pub async fn new(path_to_file: String) -> Result<Self, Box<dyn Error>>{
            let mut buffer: Vec<u8> = vec![];
            let _buf = tokio::fs::File::open(path_to_file).await?.read_buf(&mut buffer).await?;
            let  data =  buffer.translate();
            Ok(data.make::<Settings>())
    }

    pub fn make_ip(&self) -> String {
        format!("{}:{}",self.ip,self.port)
    }

}

impl fmt::Display for Settings{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}:{}",self.ip,self.port)
    }
}


