extern crate tokio;
use std::fmt;
use serde::{
    Serialize,
    Deserialize
};
use self::tokio::prelude::io::AsyncReadExt;
use std::str::from_utf8;
use serde_json::{from_str};




#[derive(Debug, Serialize, Deserialize)]
pub struct Settings{
    ip: String,
    port: i32
}

impl Settings{
    #[warn(dead_code)]
    pub fn init(ip: String, port: i32) -> Settings {
        Self {
                ip,
                port
        }
    }
    pub async fn new(path_to_file: String) -> Result<Self, Box<dyn std::error::Error>> {
            let mut buffer: Vec<u8> = vec![];
            let _buf = tokio::fs::File::open(path_to_file).await?.read_buf(&mut buffer).await?;
            let  data: &str =  from_utf8(&buffer)?;
            let s: Settings   = from_str(&data)?;
            Ok(s)
    }
    pub fn make_ip(&self) -> String {
        format!("{}:{}",self.ip,self.port)
    }

}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}:{}",self.ip,self.port)
    }
}


