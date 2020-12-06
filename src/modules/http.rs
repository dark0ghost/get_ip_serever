use crate::modules::data::Request;
use std::str::from_utf8;

pub struct Http{
}

impl Http{
    pub fn new() -> Self{
        Self{
        }
    }
    pub fn parse_request(&self, request: [u8;2048]){
       for i in from_utf8(&request){
        println!("{}",i)
    }
    }

    pub fn handle_request(){

    }

    pub fn send_head_response(&self, body: String) -> Vec<u8> {
        format!("HTTP/1.1 200 OK\r\n
        Version: HTTP/1.1\r\n
        Content-Type: text/html; charset=utf-8\r\n
        Content-Length:{}
        \r\n\r\n\
        {}
        ",body.len(),body).into_bytes()
    }


    pub fn send_error(){

    }

}