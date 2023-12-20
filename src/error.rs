use anyhow::Context;

pub enum LightsparkClientError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

