use crate::modules::traits::{Route};




#[warn(dead_code)]
pub(crate) struct Server<T: Route> {
   pub full_ip: String,
    handler: T
}

impl<T: Route> Server<T> {
    pub(crate)  fn _new(full_ip: String, handler: T,) -> Server<T> {
        Server{
            full_ip,
            handler
        }

    }

}
