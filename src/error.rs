use anyhow::Context;

pub enum LightsparkClientError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}


pub enum PlaidError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

