use serde::{Deserialize, Serialize};

pub(crate) trait Transform{
    fn translate(&self) -> String;
}

pub(crate) trait Ser{
    fn make<'a, T>(&self) ->  T
        where
            T: Deserialize<'a>,
            T: Serialize;
}