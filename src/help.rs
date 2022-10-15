/*
use crate::{
    config::Config,
    domain::{Id, Message},
    request::Request,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HelpRequest;

impl<M, C> Request<M, C> for HelpRequest
where
    M: Id,
    C: Id,
{
    type Error = Unfallible;

    fn parse(
        config: &Config,
        message: &Message<M, C>,
    ) -> Option<Result<Self, Self::Error>> {
        let matches_without_handle = message.content.trim() == "/help";
        let matches_with_handle = config
            .handle()
            .and_then(|handle| {
                message
                    .content
                    .split_once("@")
                    .map(|(head, tail)| head == "/help" && tail == handle)
            })
            .unwrap_or(false);
        if matches_with_handle || matches_without_handle {
            Some(Ok(Self))
        } else {
            None
        }
    }
}
*/
