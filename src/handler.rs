use core::fmt;
use std::{error::Error, rc::Rc, sync::Arc};

use crate::{
    command::Command,
    config::Config,
    domain::{Id, Message},
    future::DynFuture,
    port::Sender,
    request,
};

pub trait Handler {
    type MessageId: Id;
    type ChatId: Id;
    type Error: Error;

    fn run<'fut>(
        &'fut self,
        config: &'fut Config,
        input_message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>>;
}

impl<'this, H> Handler for &'this H
where
    H: Handler + ?Sized,
{
    type MessageId = H::MessageId;
    type ChatId = H::ChatId;
    type Error = H::Error;

    fn run<'fut>(
        &'fut self,
        config: &'fut Config,
        input_message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>> {
        (**self).run(config, input_message)
    }
}

impl<'this, H> Handler for &'this mut H
where
    H: Handler + ?Sized,
{
    type MessageId = H::MessageId;
    type ChatId = H::ChatId;
    type Error = H::Error;

    fn run<'fut>(
        &'fut self,
        config: &'fut Config,
        input_message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>> {
        (**self).run(config, input_message)
    }
}

impl<H> Handler for Box<H>
where
    H: Handler + ?Sized,
{
    type MessageId = H::MessageId;
    type ChatId = H::ChatId;
    type Error = H::Error;

    fn run<'fut>(
        &'fut self,
        config: &'fut Config,
        input_message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>> {
        (**self).run(config, input_message)
    }
}

impl<H> Handler for Rc<H>
where
    H: Handler + ?Sized,
{
    type MessageId = H::MessageId;
    type ChatId = H::ChatId;
    type Error = H::Error;

    fn run<'fut>(
        &'fut self,
        config: &'fut Config,
        input_message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>> {
        (**self).run(config, input_message)
    }
}

impl<H> Handler for Arc<H>
where
    H: Handler + ?Sized,
{
    type MessageId = H::MessageId;
    type ChatId = H::ChatId;
    type Error = H::Error;

    fn run<'fut>(
        &'fut self,
        config: &'fut Config,
        input_message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>> {
        (**self).run(config, input_message)
    }
}

#[derive(Debug, Clone)]
pub struct DefaultHandler<R, C, S>
where
    R: request::Parser<S::MessageId, S::ChatId>,
    C: Command<R::Request, S::MessageId, S::ChatId>,
    S: Sender,
{
    pub request_parser: R,
    pub command: C,
    pub channel: S,
}

impl<R, C, S> Handler for DefaultHandler<R, C, S>
where
    R: request::Parser<S::MessageId, S::ChatId> + Send + Sync,
    C: Command<R::Request, S::MessageId, S::ChatId> + Send + Sync,
    S: Sender + Send + Sync,
    R::Request: Send,
    S::MessageId: Send + Sync,
    S::ChatId: Send + Sync,
    R::Error: Send,
    C::Error: Send,
    S::Error: Send,
{
    type MessageId = S::MessageId;
    type ChatId = S::ChatId;
    type Error = DefaultHandlerError<R::Error, C::Error, S::Error>;

    fn run<'fut>(
        &'fut self,
        config: &'fut Config,
        input_message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>> {
        Box::pin(async move {
            match self
                .request_parser
                .parse(config, input_message)
                .map_err(DefaultHandlerError::Request)?
            {
                Some(request) => {
                    let output_message = self
                        .command
                        .execute(request)
                        .map_err(DefaultHandlerError::Command)?;
                    self.channel
                        .send(&output_message)
                        .await
                        .map_err(DefaultHandlerError::Channel)?;
                    Ok(true)
                },
                None => Ok(false),
            }
        })
    }
}

#[derive(Debug, Clone)]
pub enum DefaultHandlerError<R, C, S> {
    Request(R),
    Command(C),
    Channel(S),
}

impl<R, C, S> fmt::Display for DefaultHandlerError<R, C, S>
where
    R: fmt::Display,
    C: fmt::Display,
    S: fmt::Display,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Request(cause) => write!(fmtr, "{}", cause),
            Self::Command(cause) => write!(fmtr, "{}", cause),
            Self::Channel(cause) => write!(fmtr, "{}", cause),
        }
    }
}

impl<R, C, S> Error for DefaultHandlerError<R, C, S>
where
    R: Error,
    C: Error,
    S: Error,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Request(cause) => cause.source(),
            Self::Command(cause) => cause.source(),
            Self::Channel(cause) => cause.source(),
        }
    }
}
