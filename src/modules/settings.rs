extern crate tokio;
use std::fmt;
use serde::{
    Serialize,
    Deserialize
};
use self::tokio::prelude::io::AsyncReadExt;
use crate::modules::traits::{Transform, Ser};
use serde_json::from_str;


#[derive(Serialize, Deserialize)]
pub struct Settings {
    ip:  String,
    port: i32
}

impl<'a>  Settings {

    pub async fn new(path_to_file: String) -> Result<Self, Box<dyn std::error::Error>> {
            let mut buffer: Vec<u8> = vec![];
            let _buf = tokio::fs::File::open(path_to_file).await?.read_buf(&mut buffer).await?;
            let  data =  buffer.translate();
            let s: Settings =  data.make::<Settings>();
            Ok(s)
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


