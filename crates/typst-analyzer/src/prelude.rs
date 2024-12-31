pub use crate::backend::position_to_offset;
pub use anyhow::{anyhow, Error};
pub use typst_analyzer_analysis as typ_analysis;
pub use typst_analyzer_analysis::node::kind_walker;
pub use typst_analyzer_analysis::typ_logger;

pub type OkSome<T> = Result<Option<T>, anyhow::Error>;
