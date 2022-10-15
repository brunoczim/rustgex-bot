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

pub trait Sender {
    type MessageId: Id;
    type ChatId: Id;
    type Error: Error;

    fn send<'fut>(
        &'fut self,
        message: &'fut NewMessage<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<(), Self::Error>>;
}

impl<'this, S> Sender for &'this S
where
    S: Sender + ?Sized,
{
    type MessageId = S::MessageId;
    type ChatId = S::ChatId;
    type Error = S::Error;

    fn send<'fut>(
        &'fut self,
        message: &'fut NewMessage<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<(), Self::Error>> {
        (**self).send(message)
    }
}

impl<'this, S> Sender for &'this mut S
where
    S: Sender + ?Sized,
{
    type MessageId = S::MessageId;
    type ChatId = S::ChatId;
    type Error = S::Error;

    fn send<'fut>(
        &'fut self,
        message: &'fut NewMessage<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<(), Self::Error>> {
        (**self).send(message)
    }
}

impl<S> Sender for Box<S>
where
    S: Sender + ?Sized,
{
    type MessageId = S::MessageId;
    type ChatId = S::ChatId;
    type Error = S::Error;

    fn send<'fut>(
        &'fut self,
        message: &'fut NewMessage<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<(), Self::Error>> {
        (**self).send(message)
    }
}

impl<S> Sender for Rc<S>
where
    S: Sender + ?Sized,
{
    type MessageId = S::MessageId;
    type ChatId = S::ChatId;
    type Error = S::Error;

    fn send<'fut>(
        &'fut self,
        message: &'fut NewMessage<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<(), Self::Error>> {
        (**self).send(message)
    }
}

impl<S> Sender for Arc<S>
where
    S: Sender + ?Sized,
{
    type MessageId = S::MessageId;
    type ChatId = S::ChatId;
    type Error = S::Error;

    fn send<'fut>(
        &'fut self,
        message: &'fut NewMessage<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<(), Self::Error>> {
        (**self).send(message)
    }
}

pub trait Receiver {
    type MessageId: Id;
    type ChatId: Id;
    type Error: Error;

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

impl<'this, R> Receiver for &'this R
where
    R: Receiver + ?Sized,
{
    type MessageId = R::MessageId;
    type ChatId = R::ChatId;
    type Error = R::Error;

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

impl<'this, R> Receiver for &'this mut R
where
    R: Receiver + ?Sized,
{
    type MessageId = R::MessageId;
    type ChatId = R::ChatId;
    type Error = R::Error;

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

impl<R> Receiver for Box<R>
where
    R: Receiver + ?Sized,
{
    type MessageId = R::MessageId;
    type ChatId = R::ChatId;
    type Error = R::Error;

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

impl<R> Receiver for Rc<R>
where
    R: Receiver + ?Sized,
{
    type MessageId = R::MessageId;
    type ChatId = R::ChatId;
    type Error = R::Error;

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

impl<R> Receiver for Arc<R>
where
    R: Receiver + ?Sized,
{
    type MessageId = R::MessageId;
    type ChatId = R::ChatId;
    type Error = R::Error;

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
