use std::{env, error::Error, fmt};

const TOKEN_VAR: &str = "TELEGRAM_BOT_TOKEN";
const HANDLE_VAR: &str = "TELEGRAM_BOT_HANDLE";

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum EnvError {
    MissingToken(env::VarError),
    MissingHandle(env::VarError),
}

impl fmt::Display for EnvError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingToken(cause) => write!(
                fmtr,
                "error finding environment variable {}: {}",
                TOKEN_VAR, cause
            ),
            Self::MissingHandle(cause) => write!(
                fmtr,
                "error finding environment variable {}: {}",
                HANDLE_VAR, cause
            ),
        }
    }
}

impl Error for EnvError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::MissingToken(cause) => Some(cause),
            Self::MissingHandle(cause) => Some(cause),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub token: String,
    pub handle: String,
}

impl Environment {
    pub fn load() -> Result<Self, EnvError> {
        let token = env::var(TOKEN_VAR).map_err(EnvError::MissingToken)?;
        let handle = env::var(HANDLE_VAR).map_err(EnvError::MissingHandle)?;
        Ok(Self { token, handle })
    }
}
