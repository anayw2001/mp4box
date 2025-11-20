pub mod boxes;
pub mod parser;
pub mod registry;
pub mod util;

pub use boxes::{BoxHeader, BoxKey, BoxRef, FourCC, NodeKind};
pub use parser::{parse_children, read_box_header};
pub use registry::{BoxValue, Registry};
