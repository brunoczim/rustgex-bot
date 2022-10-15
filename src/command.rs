use crate::domain::{Id, NewMessage};
use std::{error::Error, rc::Rc, sync::Arc};

pub trait Command<R, M, C>
where
    M: Id,
    C: Id,
{
    type Error: Error;

    fn execute(&self, request: R) -> Result<NewMessage<M, C>, Self::Error>;
}

impl<'this, Co, R, M, C> Command<R, M, C> for &'this Co
where
    Co: Command<R, M, C> + ?Sized,
    M: Id,
    C: Id,
{
    type Error = Co::Error;

    fn execute(&self, request: R) -> Result<NewMessage<M, C>, Self::Error> {
        (**self).execute(request)
    }
}

impl<'this, Co, R, M, C> Command<R, M, C> for &'this mut Co
where
    Co: Command<R, M, C> + ?Sized,
    M: Id,
    C: Id,
{
    type Error = Co::Error;

    fn execute(&self, request: R) -> Result<NewMessage<M, C>, Self::Error> {
        (**self).execute(request)
    }
}

impl<'this, Co, R, M, C> Command<R, M, C> for Box<Co>
where
    Co: Command<R, M, C> + ?Sized,
    M: Id,
    C: Id,
{
    type Error = Co::Error;

    fn execute(&self, request: R) -> Result<NewMessage<M, C>, Self::Error> {
        (**self).execute(request)
    }
}

impl<'this, Co, R, M, C> Command<R, M, C> for Rc<Co>
where
    Co: Command<R, M, C> + ?Sized,
    M: Id,
    C: Id,
{
    type Error = Co::Error;

    fn execute(&self, request: R) -> Result<NewMessage<M, C>, Self::Error> {
        (**self).execute(request)
    }
}

impl<'this, Co, R, M, C> Command<R, M, C> for Arc<Co>
where
    Co: Command<R, M, C> + ?Sized,
    M: Id,
    C: Id,
{
    type Error = Co::Error;

    fn execute(&self, request: R) -> Result<NewMessage<M, C>, Self::Error> {
        (**self).execute(request)
    }
}
