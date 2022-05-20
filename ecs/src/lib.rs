use std::fmt::Debug;

pub mod entity;
pub mod manager;

pub(crate) mod type_query;

pub use type_query::TypesQueryable;

pub trait Tag {
    fn value(self) -> String;
}

impl<T> Tag for T
where
    T: Debug,
{
    fn value(self) -> String {
        format!("{:?}", self)
    }
}
