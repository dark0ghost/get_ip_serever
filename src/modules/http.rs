use crate::modules::data::Request;
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

    pub fn handle_request(){

    }

    pub fn send_head_response(&self, body: String) -> Vec<u8> {
        format!("HTTP/1.1 200 OK\r\n
        {}
        ",body).into_bytes()
    }


    pub fn send_error(){

    }

}