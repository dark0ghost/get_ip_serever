extern crate tokio;
use std::fmt;
use serde::{
    Serialize,
    Deserialize
};
use self::tokio::prelude::io::AsyncReadExt;


#[derive(Serialize, Deserialize)]
pub(crate) struct Settings{
    ip: String,
    port: i8
}

impl Settings{
    pub fn init(ip: String, port: i8) -> Settings {
        Self {
            ip,
            port
        }

    }
    pub async fn new(path_to_file: String) -> Result<Settings, Box<dyn std::error::Error>> {
            let mut buffer:Vec<String> = vec![];
            tokio::fs::File::open(path_to_file).await?.read_buf(&mut buffer);
            let mut data: String = "".to_string();
            for i in buffer{
                data+= &*i;
            }

            Ok(Settings{
                ip: "".parse().unwrap(),
                port: 1,
            })
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


