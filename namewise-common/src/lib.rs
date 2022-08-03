use std::convert::Infallible;

#[derive(Debug, Clone, strum::Display)]
pub enum NamewiseError {
    #[strum(serialize = "_0")]
    MissingField(String),
}

impl From<Infallible> for NamewiseError {
    fn from(_: Infallible) -> Self {
        panic!("An Infallible should never happen")
    }
}

impl std::error::Error for NamewiseError {}
