use std::convert::Infallible;

#[derive(Debug, derive_more::Display, thiserror::Error)]
pub enum NamewiseError {
    #[display(fmt = "Missing field error: {_0}")]
    MissingField(String),
    #[display(fmt = "Generic error: {_0}")]
    Generic(Box<dyn std::error::Error + Send + Sync>),
}

impl From<Infallible> for NamewiseError {
    fn from(_: Infallible) -> Self {
        panic!("An Infallible should never happen")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_error_message() {
        assert_eq!(
            NamewiseError::MissingField("x is missing".to_string()).to_string(),
            "Missing field error: x is missing"
        );
    }
}
