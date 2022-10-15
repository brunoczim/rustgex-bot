use crate::domain::{Id, Message};
use std::error::Error;

pub trait Request<M, C>: Sized
where
    M: Id,
    C: Id,
{
    type Error: Error;

    fn parse(message: &Message<M, C>) -> Option<Result<Self, Self::Error>>;
}
