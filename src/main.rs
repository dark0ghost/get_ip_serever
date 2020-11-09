use crate::modules::settings::Settings;
use tokio::net::TcpListener;
use tokio::prelude::*;
use crate::modules::handler::Handler;

mod modules;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error>>{
    let settings: Settings = Settings::new(false,"".to_string());
    let handler: Handler = Handler::new();
    let  listener = TcpListener::bind(settings.make_ip()).await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {

                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}