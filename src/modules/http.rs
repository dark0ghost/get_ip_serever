use std::str::from_utf8;
use std::error::Error;
use crate::modules::traits::Print;

pub struct Http{
}

impl Http{
    pub fn new() -> Self{
        Self{
        }
    }
    pub fn parse_request(&self, request: [u8;2048]) -> Result<(), Box<dyn Error>> {


        for i in  from_utf8(&request){
            let data: Vec<&str> = i.split_whitespace().collect();
            data.print();
        }
         Ok(())
    }
    #[warn(dead_code)]
    pub fn handle_request(){

    }

    pub fn send_response(&self, body: String) -> Vec<u8> {
        format!("HTTP/1.1 200 OK\r\n
        {}
        ",body).into_bytes()
    }

    pub fn send_error(code: i8) -> Vec<u8> {
        format!("HTTP/1.1 {} OK\r\n",code).into_bytes()
    }

}