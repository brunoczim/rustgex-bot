use std::{error::Error, fmt, rc::Rc, sync::Arc};

use crate::{
    command::Command,
    domain::{Bot, Id, Message, MessageData, NewMessage, ReplyTarget},
    future::DynFuture,
    port::Sender,
    request,
};

pub trait Handler: fmt::Debug {
    type MessageId: Id;
    type ChatId: Id;
    type Error: Error;

    fn run<'fut>(
        &'fut self,
        bot: &'fut Bot,
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
        bot: &'fut Bot,
        input_message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>> {
        (**self).run(bot, input_message)
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
        bot: &'fut Bot,
        input_message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>> {
        (**self).run(bot, input_message)
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
        bot: &'fut Bot,
        input_message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>> {
        (**self).run(bot, input_message)
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
        bot: &'fut Bot,
        input_message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>> {
        (**self).run(bot, input_message)
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
        bot: &'fut Bot,
        input_message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>> {
        (**self).run(bot, input_message)
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
    pub sender: S,
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
    type Error = S::Error;

    fn run<'fut>(
        &'fut self,
        bot: &'fut Bot,
        input_message: &'fut Message<Self::MessageId, Self::ChatId>,
    ) -> DynFuture<'fut, Result<bool, Self::Error>> {
        Box::pin(async move {
            match self.request_parser.parse(bot, input_message) {
                Some(parse_result) => {
                    let output_message = match parse_result {
                        Ok(request) => match self.command.execute(request) {
                            Ok(message) => message,
                            Err(error) => NewMessage {
                                data: MessageData {
                                    content: error.to_string(),
                                    chat_id: input_message.data.chat_id,
                                    reply_target: ReplyTarget::MessageId(
                                        input_message.id,
                                    ),
                                },
                            },
                        },
                        Err(error) => NewMessage {
                            data: MessageData {
                                content: error.to_string(),
                                chat_id: input_message.data.chat_id,
                                reply_target: ReplyTarget::MessageId(
                                    input_message.id,
                                ),
                            },
                        },
                    };

                    self.sender.send(&output_message).await?;
                    Ok(true)
                },
                None => Ok(false),
            }
        })
    }
}
