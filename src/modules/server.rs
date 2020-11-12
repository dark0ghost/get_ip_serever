use crate::modules::traits::{
    Transform, Route
};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::io::{AsyncWriteExt};


pub(crate) struct Server<T: Route>{
   full_ip: String,
   work: bool,
   handler: T

}

impl<T: Route> Server<T> {
    pub(crate)  fn new(full_ip: String,handler: T) -> Server<T> {
        Server{
           full_ip,
            work: true,
            handler
        }

    }
    async fn read_socket(&self, &mut socket: TcpStream, &mut buff: Vec<u8>) -> usize  {
        match socket.read(&mut *buff).await {
            Ok(n) if n == 0 =>  1 as usize,
            Ok(n) => n,
            Err(e) => {
                panic!("failed to read from socket; err = {:?}", e)
            }
        }
    }

    pub(crate) async fn start_server(&self) -> Result<(), Box<dyn std::error::Error>> {
        let  listener: TcpListener = TcpListener::bind(&self.full_ip).await?;
        while self.work {
            let (mut socket, _) = listener.accept().await?;
            tokio::spawn(
                async move {
                    let  buf: Vec<u8> = vec![];
                    loop {
                        let n = self.read_socket(&socket, &buf);
                        println!("{}",buf.translate());
                        if let Err(e) = socket.write_all(&buf[0..n]).await {
                            eprintln!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                    }
                }
            );
        }
        Result::Ok(())
    }

    pub(crate) fn start_stop(&self){

    }
}