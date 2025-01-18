mod actions;
pub mod bibliography;
pub mod completion;
pub mod definition;
pub mod dict;
pub mod error;
mod hints;
pub mod node;

pub use completion::resources::*;
pub use hints::handle::*;

pub(crate) type OkSome<T> = Result<Option<T>, anyhow::Error>;
