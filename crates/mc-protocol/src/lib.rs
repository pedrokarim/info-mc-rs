pub mod bedrock;
pub mod dns;
pub mod error;
pub mod query;
pub mod slp;
pub mod types;

// Re-export primary API at crate root
pub use bedrock::{ping_bedrock, BedrockConfig, BedrockResponse};
pub use dns::{resolve_address, ResolvedAddress};
pub use error::{McProtocolError, Result};
pub use query::{query_basic, query_full, BasicStat, FullStat, QueryConfig};
pub use slp::{ping_java, MotdComponent, SlpConfig, SlpResponse};
