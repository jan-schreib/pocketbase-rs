use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Reqwest error")]
    HttpError(#[from] reqwest::Error),
}
