extern crate tokio;
use std::fmt;
use serde::{
    Serialize,
    Deserialize
};
use self::tokio::prelude::io::AsyncReadExt;
use std::str::from_utf8;
use serde_json::{from_str, Value};



#[derive(Serialize, Deserialize)]
pub struct Settings{
    ip: String,
    port: i8
}

impl Settings{
    #[warn(dead_code)]
    pub fn init(ip: String, port: i8) -> Settings {
        Self {
            ip,
            port
        }

    }
    pub async fn new(path_to_file: String) -> Result<Settings, Box<dyn std::error::Error>> {
            let mut buffer = vec![];
            let _buf = tokio::fs::File::open(path_to_file).await?.read_buf(&mut buffer).await?;
            let  data =  from_utf8(&buffer)?;
            let s: Settings  = from_str(data)?;

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


