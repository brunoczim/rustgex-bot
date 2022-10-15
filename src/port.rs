use crate::domain::{Id, Message};
use core::fmt;
use std::{error::Error, future::Future, pin::Pin};

pub type DynFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Debug, Clone, Copy)]
pub struct Disconnected;

impl fmt::Display for Disconnected {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "message channel disconnected")
    }
}

impl Error for Disconnected {}

pub trait MessageChannel: Send + Sync {
    type MessageId: Id;
    type ChatId: Id;
    type Error: Error + Send + Sync;

    fn send<'fut>(
        &'fut self,
        message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<(), Self::Error>>;

    fn receive<'fut>(
        &'fut self,
    ) -> DynFuture<
        'fut,
        Result<
            Result<Message<Self::MessageId, Self::ChatId>, Disconnected>,
            Self::Error,
        >,
    >;
}
