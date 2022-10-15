use crate::{config::Config, domain::Id, handler::Handler, port::Receiver};
use std::{error::Error, fmt, sync::Arc};

#[derive(Debug, Clone)]
pub enum AppError<R, H> {
    Receiver(R),
    Handler(H),
}

impl<R, H> fmt::Display for AppError<R, H>
where
    R: fmt::Display,
    H: fmt::Display,
{
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Receiver(cause) => write!(fmtr, "{}", cause),
            Self::Handler(cause) => write!(fmtr, "{}", cause),
        }
    }
}

impl<R, H> Error for AppError<R, H>
where
    R: Error,
    H: Error,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Receiver(cause) => cause.source(),
            Self::Handler(cause) => cause.source(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct App<'handlers, M, C, E>
where
    M: Id,
    C: Id,
    E: Error,
{
    config: Config,
    handlers: Vec<
        Arc<
            dyn Handler<MessageId = M, ChatId = C, Error = E>
                + Send
                + Sync
                + 'handlers,
        >,
    >,
}

impl<'handlers, M, C, E> App<'handlers, M, C, E>
where
    M: Id,
    C: Id,
    E: Error,
{
    pub fn new(config: Config) -> Self {
        Self { config, handlers: Vec::new() }
    }

    pub fn handler<H>(mut self, handler: H) -> Self
    where
        H: Handler<MessageId = M, ChatId = C, Error = E>
            + Send
            + Sync
            + 'handlers,
    {
        self.handlers.push(Arc::new(handler));
        self
    }

    pub async fn run<R>(self, receiver: R) -> Result<(), AppError<R::Error, E>>
    where
        R: Receiver<MessageId = M, ChatId = C>,
    {
        while let Ok(input_message) =
            receiver.receive().await.map_err(AppError::Receiver)?
        {
            for handler in &self.handlers {
                if handler
                    .run(&self.config, &input_message)
                    .await
                    .map_err(AppError::Handler)?
                {
                    break;
                }
            }
        }

        Ok(())
    }
}
