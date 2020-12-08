mod error;
mod instruction;
mod program;
mod vm;

pub use error::*;
pub use instruction::*;
pub use program::*;
pub use vm::*;

pub type ParseResult<T> = std::result::Result<T, ParseError>;
pub type CodeParseResult<T> = std::result::Result<T, CodeParseError>;
pub type ExecutionResult<T> = std::result::Result<T, ExecutionError>;
