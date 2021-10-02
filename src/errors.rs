use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResultError {
    #[error("Length of input and node does not match.")]
    DoesNotMatch
}