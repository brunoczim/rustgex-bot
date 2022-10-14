use std::{fmt, future::Future, pin::Pin};
use telegram_bot::{CanSendMessage, Update};

pub type DynFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

pub enum Response<T, E> {
    Unrecognized,
    Success(T),
    Error(E),
}

pub trait Command: Sized {
    type Output: CanSendMessage;
    type Error: fmt::Display;

    fn execute(
        &self,
        update: &Update,
    ) -> DynFuture<Response<Self::Output, Self::Error>>;
}
