use core::fmt;
use std::error::Error;

use crate::{
    command::Command,
    config::Config,
    domain::{Id, Message},
    future::DynFuture,
    port::MessageChannel,
    request,
};

pub trait Handler {
    type MessageId: Id;
    type ChatId: Id;
    type Error: Error;

    fn run<'fut>(
        &'fut self,
        config: &'fut Config,
        message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>>;
}

#[derive(Debug, Clone)]
pub struct DefaultHandler<R, Co, C>
where
    R: request::Parser<C::MessageId, C::ChatId>,
    Co: Command<R::Request, C::MessageId, C::ChatId>,
    C: MessageChannel,
{
    pub request_parser: R,
    pub command: Co,
    pub channel: C,
}

impl<R, Co, C> Handler for DefaultHandler<R, Co, C>
where
    R: request::Parser<C::MessageId, C::ChatId> + Send + Sync,
    Co: Command<R::Request, C::MessageId, C::ChatId> + Send + Sync,
    C: MessageChannel + Send + Sync,
    R::Request: Send,
    C::MessageId: Send + Sync,
    C::ChatId: Send + Sync,
    R::Error: Send,
    Co::Error: Send,
    C::Error: Send,
{
    type MessageId = C::MessageId;
    type ChatId = C::ChatId;
    type Error = DefaultHandlerError<R::Error, Co::Error, C::Error>;

    fn run<'fut>(
        &'fut self,
        config: &'fut Config,
        message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>> {
        Box::pin(async move {
            match self
                .request_parser
                .parse(config, message)
                .map_err(DefaultHandlerError::Request)?
            {
                Some(request) => {
                    let message = self
                        .command
                        .execute(request)
                        .map_err(DefaultHandlerError::Command)?;
                    self.channel
                        .send(&message)
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
pub enum DefaultHandlerError<R, Co, C> {
    Request(R),
    Command(Co),
    Channel(C),
}

impl<R, Co, C> fmt::Display for DefaultHandlerError<R, Co, C>
where
    R: fmt::Display,
    Co: fmt::Display,
    C: fmt::Display,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Request(cause) => write!(fmtr, "{}", cause),
            Self::Command(cause) => write!(fmtr, "{}", cause),
            Self::Channel(cause) => write!(fmtr, "{}", cause),
        }
    }
}

impl<R, Co, C> Error for DefaultHandlerError<R, Co, C>
where
    R: Error,
    Co: Error,
    C: Error,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Request(cause) => cause.source(),
            Self::Command(cause) => cause.source(),
            Self::Channel(cause) => cause.source(),
        }
    }
}
