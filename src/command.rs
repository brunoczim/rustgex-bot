use crate::{
    domain::{Id, Message},
    request::Request,
};
use std::error::Error;

pub trait Command<R, M, C>
where
    R: Request<M, C>,
    M: Id,
    C: Id,
{
    type Error: Error;

    fn execute(&mut self, request: R) -> Result<Message<M, C>, Self::Error>;
}
