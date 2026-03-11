pub mod bedrock;
pub mod dns;
pub mod error;
pub mod query;
pub mod slp;
pub mod types;

// Re-export primary API at crate root
pub use bedrock::{BedrockConfig, BedrockResponse, ping_bedrock};
pub use dns::{ResolvedAddress, resolve_address};
pub use error::{McProtocolError, Result};
pub use query::{BasicStat, FullStat, QueryConfig, query_basic, query_full};
pub use slp::{MotdComponent, SlpConfig, SlpResponse, ping_java};
