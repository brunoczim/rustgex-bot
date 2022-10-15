use std::{borrow::Borrow, env, error::Error, fmt, num};

#[derive(Debug)]
pub struct Builder {
    config: Config,
}

impl Builder {
    pub fn new(token: String) -> Self {
        Self {
            config: Config { token, handle: None, max_failures_per_minute: 30 },
        }
    }

    pub fn handle(self, handle: String) -> Self {
        Self { config: Config { handle: Some(handle), ..self.config } }
    }

    pub fn max_failures_per_minute(self, count: u128) -> Self {
        Self {
            config: Config { max_failures_per_minute: count, ..self.config },
        }
    }

    pub fn finish(self) -> Config {
        self.config
    }
}

const TOKEN_VAR: &str = "TELEGRAM_BOT_TOKEN";
const HANDLE_VAR: &str = "TELEGRAM_BOT_HANDLE";
const MAX_FAILURES_VAR: &str = "TELEGRAM_BOT_MAX_FAILURES_PER_MINUTE";

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum EnvError {
    MissingToken(env::VarError),
    MaxFailuresParse(num::ParseIntError),
}

impl fmt::Display for EnvError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingToken(cause) => write!(
                fmtr,
                "error finding environment variable {}: {}",
                TOKEN_VAR, cause
            ),
            Self::MaxFailuresParse(cause) => write!(
                fmtr,
                "error parsing environment variable {}: {}",
                MAX_FAILURES_VAR, cause
            ),
        }
    }
}

impl Error for EnvError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::MissingToken(cause) => Some(cause),
            Self::MaxFailuresParse(cause) => Some(cause),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    token: String,
    handle: Option<String>,
    max_failures_per_minute: u128,
}

impl Config {
    pub fn from_env() -> Result<Self, EnvError> {
        let mut builder =
            Builder::new(env::var(TOKEN_VAR).map_err(EnvError::MissingToken)?);
        if let Some(handle) = env::var(HANDLE_VAR).ok() {
            builder = builder.handle(handle);
        }
        if let Some(count) = env::var(MAX_FAILURES_VAR).ok() {
            builder = builder.max_failures_per_minute(
                count.parse().map_err(EnvError::MaxFailuresParse)?,
            );
        }
        Ok(builder.finish())
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn handle(&self) -> Option<&str> {
        self.handle.as_ref().map(String::borrow)
    }

    pub fn max_failures_per_minute(&self) -> u128 {
        self.max_failures_per_minute
    }
}
