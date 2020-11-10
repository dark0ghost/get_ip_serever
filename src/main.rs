use crate::modules::settings::Settings;
use tokio::net::TcpListener;
use tokio::prelude::*;
use crate::modules::handler::Handler;
use log::{
    info, trace,
};
use crate::modules::traits::Transform;


mod modules;


#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error>>{
    let settings: Settings = Settings::new("src/doc/server.json".parse().unwrap()).await?;
    let _handler: Handler = Handler::new();
    let  listener: TcpListener = TcpListener::bind(settings.make_ip()).await?;
    info!("start server");
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(
            async move {
            let mut buf: Vec<u8> = vec![];
            loop {
                let n = match socket.read(&mut buf).await {

                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        trace!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };
                println!("{}",buf.translate())
            }
        }
        );
    }
}