use std::fmt;
use serde_json::json;

pub(crate) struct Settings{
    ip: String,
    port: i8
}

impl Settings{
    pub fn new(use_file: bool, path_to_file: String) -> Settings {
        if !use_file {
            Self {
                ip: "".to_string(),
                port: 80
            }
        }else{
            json!(1);

            Self {
                ip: "".to_string(),
                port: 80
            }
        }
    }
    fn make_ip(self) -> String {
        format!("{}:{}",self.ip,self.port)
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}:{}",self.ip,self.port)
    }
}
