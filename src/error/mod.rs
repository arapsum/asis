#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Request line is empty")]
    EmptyRequest,
    #[error("{0}")]
    MalformedRequestLine(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
