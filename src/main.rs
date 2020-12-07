use crate::modules::settings::Settings;
use crate::modules::handler::Handler;
use tokio::net::TcpListener;
use std::error::Error;
use crate::modules::traits::Transform;
use tokio::prelude::io::{AsyncWriteExt, AsyncReadExt};
use crate::modules::http::Http;



mod modules;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings: Settings = Settings::new("src/config/server.json".to_string()).await?;
    let handler: Handler = Handler::new();
    let link = settings.make_ip();
    println!("start at http://{}",link);
    loop {
        let listener: TcpListener = TcpListener::bind(settings.make_ip()).await?;
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(
            async move {
                let mut buff = [0; 2048];
                let http: Http = Http::new();
                    let _n = match socket.read(&mut buff).await {
                        Ok(n) if n == 0 => {
                            println!("error: {}",n);
                            return
                        },
                        Ok(n) => n,
                        Err(e) => {
                            eprintln!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };
                    if let Err(e) = http.parse_request(buff){
                        return;
                    }
                    let  response = &*http.send_response("<title>Test C++ HTTP Server</title>\n\n".to_string());
                    println!("{}",response.to_vec().translate());
                    if let Err(e) = socket.write_all(response).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
            }
        );
    }
}