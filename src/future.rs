use std::{future::Future, pin::Pin};

pub type DynFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
