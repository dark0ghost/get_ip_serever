use serde::{Deserialize, Serialize};
use serde::export::fmt::Display;

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

}

pub(crate) trait Print{
    fn print(&self);
    fn println(&self);
}

