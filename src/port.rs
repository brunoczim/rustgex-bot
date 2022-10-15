use crate::{
    domain::{Id, Message, NewMessage},
    future::DynFuture,
};
use core::fmt;
use std::{error::Error, rc::Rc, sync::Arc};

#[derive(Debug, Clone, Copy)]
pub struct Disconnected;

impl fmt::Display for Disconnected {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "message channel disconnected")
    }
}

impl Error for Disconnected {}

pub trait MessageChannel {
    type MessageId: Id;
    type ChatId: Id;
    type Error: Error;

    fn send<'fut>(
        &'fut self,
        message: &'fut NewMessage<Self::MessageId, Self::ChatId>,
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

impl<'this, M> MessageChannel for &'this M
where
    M: MessageChannel + ?Sized,
{
    type MessageId = M::MessageId;
    type ChatId = M::ChatId;
    type Error = M::Error;

    fn send<'fut>(
        &'fut self,
        message: &'fut NewMessage<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<(), Self::Error>> {
        (**self).send(message)
    }

    fn receive<'fut>(
        &'fut self,
    ) -> DynFuture<
        'fut,
        Result<
            Result<Message<Self::MessageId, Self::ChatId>, Disconnected>,
            Self::Error,
        >,
    > {
        (**self).receive()
    }
}

impl<'this, M> MessageChannel for &'this mut M
where
    M: MessageChannel + ?Sized,
{
    type MessageId = M::MessageId;
    type ChatId = M::ChatId;
    type Error = M::Error;

    fn send<'fut>(
        &'fut self,
        message: &'fut NewMessage<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<(), Self::Error>> {
        (**self).send(message)
    }

    fn receive<'fut>(
        &'fut self,
    ) -> DynFuture<
        'fut,
        Result<
            Result<Message<Self::MessageId, Self::ChatId>, Disconnected>,
            Self::Error,
        >,
    > {
        (**self).receive()
    }
}

impl<M> MessageChannel for Box<M>
where
    M: MessageChannel + ?Sized,
{
    type MessageId = M::MessageId;
    type ChatId = M::ChatId;
    type Error = M::Error;

    fn send<'fut>(
        &'fut self,
        message: &'fut NewMessage<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<(), Self::Error>> {
        (**self).send(message)
    }

    fn receive<'fut>(
        &'fut self,
    ) -> DynFuture<
        'fut,
        Result<
            Result<Message<Self::MessageId, Self::ChatId>, Disconnected>,
            Self::Error,
        >,
    > {
        (**self).receive()
    }
}

impl<M> MessageChannel for Rc<M>
where
    M: MessageChannel + ?Sized,
{
    type MessageId = M::MessageId;
    type ChatId = M::ChatId;
    type Error = M::Error;

    fn send<'fut>(
        &'fut self,
        message: &'fut NewMessage<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<(), Self::Error>> {
        (**self).send(message)
    }

    fn receive<'fut>(
        &'fut self,
    ) -> DynFuture<
        'fut,
        Result<
            Result<Message<Self::MessageId, Self::ChatId>, Disconnected>,
            Self::Error,
        >,
    > {
        (**self).receive()
    }
}

impl<M> MessageChannel for Arc<M>
where
    M: MessageChannel + ?Sized,
{
    type MessageId = M::MessageId;
    type ChatId = M::ChatId;
    type Error = M::Error;

    fn send<'fut>(
        &'fut self,
        message: &'fut NewMessage<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<(), Self::Error>> {
        (**self).send(message)
    }

    fn receive<'fut>(
        &'fut self,
    ) -> DynFuture<
        'fut,
        Result<
            Result<Message<Self::MessageId, Self::ChatId>, Disconnected>,
            Self::Error,
        >,
    > {
        (**self).receive()
    }
}
