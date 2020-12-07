use serde::{Deserialize, Serialize};

pub(crate) trait Transform{
    fn translate(&self) -> &str;
}

pub(crate) trait Ser<'a>{
    fn make<T>(&self) ->  T
        where
            T: Deserialize<'a>,
            T: Serialize;
}

pub(crate) trait Route{
fn add_route<T>(&self,route: Vec<T>);

}

pub(crate) trait Print{
    fn print(&self) -> &Self;
    fn println(&self) -> &Self;
}

