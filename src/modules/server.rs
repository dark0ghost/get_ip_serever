use crate::modules::traits::{Route};





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
