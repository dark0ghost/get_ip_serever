pub struct Request {
    pub ip: String,
    pub method: String,
    pub host: String,
    pub content_type: String,
    pub connection: bool,
    pub content_length: isize,
    pub data: String
}