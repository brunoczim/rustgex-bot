use crate::{
    command::Command,
    domain::{Bot, Id, Message, MessageData, NewMessage, ReplyTarget},
    request,
};
use std::{error::Error, fmt};

#[derive(Debug)]
pub enum Unfallible {}

impl fmt::Display for Unfallible {
    fn fmt(&self, _fmtr: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl Error for Unfallible {}

#[derive(Debug, Clone, Copy)]
pub struct HelpRequestParser;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HelpRequest<M, C>
where
    M: Id,
    C: Id,
{
    original_message_id: M,
    chat_id: C,
}

impl<M, C> request::Parser<M, C> for HelpRequestParser
where
    M: Id,
    C: Id,
{
    type Request = HelpRequest<M, C>;
    type Error = Unfallible;

    fn parse(
        &self,
        bot: &Bot,
        message: &Message<M, C>,
    ) -> Result<Option<Self::Request>, Self::Error> {
        let matches_without_handle = message.data.content.trim() == "/help";
        let matches_with_handle = message
            .data
            .content
            .split_once("@")
            .map(|(head, tail)| head == "/help" && tail == bot.handle)
            .unwrap_or(false);
        if matches_with_handle || matches_without_handle {
            Ok(Some(HelpRequest {
                original_message_id: message.id,
                chat_id: message.data.chat_id,
            }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HelpCommand;

impl<M, C> Command<HelpRequest<M, C>, M, C> for HelpCommand
where
    M: Id,
    C: Id,
{
    type Error = Unfallible;

    fn execute(
        &self,
        request: HelpRequest<M, C>,
    ) -> Result<NewMessage<M, C>, Self::Error> {
        Ok(NewMessage {
            data: MessageData {
                chat_id: request.chat_id,
                content: String::from(
                    "This bot performs replacements on messages based on \
                     regular expressions.\n\n- /help -- shows this \
                     message\n\n- s/regex/replacement/flags -- performs a \
                     replacement in the previous message or in the message \
                     you're replying to.",
                ),
                reply_target: ReplyTarget::MessageId(
                    request.original_message_id,
                ),
            },
        })
    }
}
