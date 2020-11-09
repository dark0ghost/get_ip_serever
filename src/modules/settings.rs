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
    pub async fn new(use_file: bool, path_to_file: String) -> Settings {
        if !use_file {
            let mut buffer:Vec<String> = vec![];
            tokio::fs::File::open(path_to_file).await?.read_buf(&mut buffer);
            let mut data: String = "".to_string();
            for i in buffer{
                data+= &*i;
            }

            Settings{
               ip: "".parse().unwrap(),
               port: 1,
            }

        }else{


            Self {
                ip: (*"".to_string()).parse().unwrap(),
                port: 80
            }
        }
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


