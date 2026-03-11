mod html;
mod parser;
mod plain;
mod types;

pub use html::to_html;
pub use parser::parse_motd;
pub use plain::to_plain_text;
pub use types::{McColor, MotdNode, MotdStyle};

/// Parse a MOTD from raw MC protocol data (can be a string or Chat Component JSON).
/// Returns HTML, plain text, and raw representations.
#[derive(Debug, Clone, serde::Serialize)]
pub struct MotdRendered {
    pub raw: String,
    pub clean: String,
    pub html: String,
}

/// Render a MOTD from a serde_json::Value (as received from SLP).
pub fn render_motd(value: &serde_json::Value) -> MotdRendered {
    let nodes = parse_motd(value);
    let html_str = to_html(&nodes);
    let clean = to_plain_text(&nodes);

    let raw = match value {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    };

    MotdRendered {
        raw,
        clean,
        html: html_str,
    }
}
