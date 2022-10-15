use std::{fmt, hash::Hash};

pub trait Id:
    fmt::Debug
    + fmt::Display
    + Clone
    + Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Hash
    + Send
    + Sync
{
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message<M, C>
where
    M: Id,
    C: Id,
{
    pub id: M,
    pub chat_id: C,
    pub content: String,
    pub replying_to: Option<Box<Self>>,
}
