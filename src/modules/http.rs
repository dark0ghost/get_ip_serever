use crate::modules::data::Request;

pub struct Http{
}

impl Http{
    pub fn new() -> Self{
        Self{
        }
    }
    pub fn parse_request(&self,request: Request){

    }

    pub fn handle_request(){

    }

    pub fn send_head_response(&self, body: String) -> Vec<u8> {
        format!("HTTP/1.1 200 OK\
        Version: HTTP/1.1\
        Content-Type: text/html; charset=utf-8
        Content-Length:{}
        ",body.len()).into_bytes()
    }


    pub fn send_error(){

    }

}