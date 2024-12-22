use thiserror::Error;

#[derive(Error, Debug)]
pub enum TyError {
    #[error("Error: this")]
    NotFound,
    #[error("Error: 0")]
    Invalid,
    #[error("Error: 0")]
    SyntaxError,
}
