use std::str::from_utf8;
use crate::modules::traits::Print;
use crate::modules::data::Request;

pub struct Http{
}

impl Http{

    pub fn new() -> Self{
        Self{}
    }

    pub fn parse_request(&mut self, request: [u8;2048],ip: String) -> Request {
        let mut data_parse: Vec<Vec<&str>> = vec![];
        for i in  from_utf8(&request){
            let  data: Vec<&str> = i.split_whitespace().collect();
            data.println();
            data_parse.push(data);
        }

        let method = data_parse[0][0];
        let url_request = data_parse[0][1];
        let host =  data_parse[0][4];

        Request{
            ip,
            method: method.to_string(),
            host: host.to_string(),
            url_request: url_request.to_string()
        }
    }

    pub fn send_response(&self, body: String) -> Vec<u8> {
        format!("HTTP/1.1 200 OK\r\n
        {}
        ",body).into_bytes()
    }
}