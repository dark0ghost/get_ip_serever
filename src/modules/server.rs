use crate::modules::traits::{Route};
use tokio::net::{
     TcpStream
};
use tokio::prelude::io::{AsyncReadExt};
use crate::modules::http::Http;


pub(crate) struct Server<T: Route> {
   pub full_ip: String,
    handler: T
}

impl<T: Route> Server<T> {
    pub(crate)  fn new(full_ip: String, handler: T,) -> Server<T> {
        Server{
            full_ip,
            handler
        }

    }

}
