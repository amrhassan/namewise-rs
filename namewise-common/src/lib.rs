use std::convert::Infallible;

#[derive(Debug, strum::Display, thiserror::Error)]
pub enum NamewiseError {
    #[strum(serialize = "_0")]
    MissingField(String),
    Generic(Box<dyn std::error::Error + Send + Sync>),
}

impl From<Infallible> for NamewiseError {
    fn from(_: Infallible) -> Self {
        panic!("An Infallible should never happen")
    }
}
