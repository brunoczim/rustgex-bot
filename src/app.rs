use crate::{domain::Id, handler::Handler};
use std::{error::Error, sync::Arc};

pub struct App<M, C, E>
where
    M: Id,
    C: Id,
    E: Error,
{
    handlers: Vec<
        Arc<dyn Handler<MessageId = M, ChatId = C, Error = E> + Send + Sync>,
    >,
}

impl<M, C, E> App<M, C, E>
where
    M: Id,
    C: Id,
    E: Error,
{
    fn new<I>(handlers: I) -> Self
    where
        I: IntoIterator<
            Item = Arc<
                dyn Handler<MessageId = M, ChatId = C, Error = E> + Send + Sync,
            >,
        >,
    {
        Self { handlers: handlers.into_iter().collect() }
    }
}
