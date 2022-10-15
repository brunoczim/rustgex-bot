use crate::{
    config::Config,
    domain::{Id, Message},
};
use std::{error::Error, rc::Rc, sync::Arc};

pub trait Parser<M, C>
where
    M: Id,
    C: Id,
{
    type Error: Error;
    type Request;

    fn parse(
        &self,
        config: &Config,
        message: &Message<M, C>,
    ) -> Result<Option<Self::Request>, Self::Error>;
}

impl<'this, P, M, C> Parser<M, C> for &'this P
where
    P: Parser<M, C> + ?Sized,
    M: Id,
    C: Id,
{
    type Error = P::Error;
    type Request = P::Request;

    fn parse(
        &self,
        config: &Config,
        message: &Message<M, C>,
    ) -> Result<Option<Self::Request>, Self::Error> {
        (**self).parse(config, message)
    }
}

impl<'this, P, M, C> Parser<M, C> for &'this mut P
where
    P: Parser<M, C> + ?Sized,
    M: Id,
    C: Id,
{
    type Error = P::Error;
    type Request = P::Request;

    fn parse(
        &self,
        config: &Config,
        message: &Message<M, C>,
    ) -> Result<Option<Self::Request>, Self::Error> {
        (**self).parse(config, message)
    }
}

impl<P, M, C> Parser<M, C> for Box<P>
where
    P: Parser<M, C> + ?Sized,
    M: Id,
    C: Id,
{
    type Error = P::Error;
    type Request = P::Request;

    fn parse(
        &self,
        config: &Config,
        message: &Message<M, C>,
    ) -> Result<Option<Self::Request>, Self::Error> {
        (**self).parse(config, message)
    }
}

impl<P, M, C> Parser<M, C> for Rc<P>
where
    P: Parser<M, C> + ?Sized,
    M: Id,
    C: Id,
{
    type Error = P::Error;
    type Request = P::Request;

    fn parse(
        &self,
        config: &Config,
        message: &Message<M, C>,
    ) -> Result<Option<Self::Request>, Self::Error> {
        (**self).parse(config, message)
    }
}

impl<P, M, C> Parser<M, C> for Arc<P>
where
    P: Parser<M, C> + ?Sized,
    M: Id,
    C: Id,
{
    type Error = P::Error;
    type Request = P::Request;

    fn parse(
        &self,
        config: &Config,
        message: &Message<M, C>,
    ) -> Result<Option<Self::Request>, Self::Error> {
        (**self).parse(config, message)
    }
}
