use crate::modules::settings::Settings;
use tokio::net::TcpListener;
use std::error::Error;
use crate::modules::traits::Transform;
use tokio::prelude::io::{AsyncWriteExt, AsyncReadExt};
use crate::modules::http::Http;

mod modules;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings: Settings = Settings::new("src/config/server.json".to_string()).await?;
    let link: String = settings.make_ip();
    println!("start at http://{}",link);
    loop {
        let listener: TcpListener = TcpListener::bind(settings.make_ip()).await?;
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(
            async move {
                let mut buff = [0; 2048];
                let mut http: Http = Http::new();
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
                    let response_parse = http.parse_request(buff, socket.peer_addr().ok().unwrap().to_string());
                    let  response = &*http.send_response(format!("<title>Test Rust HTTP Server</title>\n\n<h1> u ip is {}<h1>",response_parse.ip).to_string());
                    println!("{}",response.to_vec().translate());
                    if let Err(e) = socket.write_all(response).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
            }
        );
    }
}