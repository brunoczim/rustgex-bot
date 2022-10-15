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
{
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReplyTarget<M, C>
where
    M: Id,
    C: Id,
{
    Message(Box<Message<M, C>>),
    MessageId(M),
    Prunned,
    NotReplying,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageData<M, C>
where
    M: Id,
    C: Id,
{
    pub chat_id: C,
    pub content: String,
    pub reply_target: ReplyTarget<M, C>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Message<M, C>
where
    M: Id,
    C: Id,
{
    pub id: M,
    pub data: MessageData<M, C>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NewMessage<M, C>
where
    M: Id,
    C: Id,
{
    pub data: MessageData<M, C>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bot {
    pub handle: String,
}
