use std::error::Error;
use mio::Events;
use std::net::TcpListener;


mod handler;
mod settings;
use settings::Settings;
use handler::Handler;


fn main() -> Result<(), Box<dyn Error>> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    const SETTINGS: Settings = Settings::new();
    const HANDLER: Handler = Handler;
    let mut server = TcpListener::bind(SETTINGS)?;


}
