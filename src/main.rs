use crate::modules::settings::Settings;
use crate::modules::handler::Handler;
use crate::modules::server::Server;


mod modules;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>>  {
    let settings: Settings = Settings::new("src/doc/server.json".parse()?).await?;
    let handler: Handler = Handler::new();
    let link = settings.make_ip();
    println!("start at http://{}",link);
    let server: Server<Handler> = Server::new(link,handler);
    server.start_server().await?;
    //Ok(())

}