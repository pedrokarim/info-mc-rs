use crate::types::MotdNode;

/// Render MOTD nodes to plain text, stripping all formatting.
pub fn to_plain_text(nodes: &[MotdNode]) -> String {
    nodes.iter().map(|n| n.text.as_str()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MotdStyle;

    #[test]
    fn test_plain_text() {
        let nodes = vec![
            MotdNode {
                text: "Hello ".to_string(),
                style: MotdStyle::default(),
            },
            MotdNode {
                text: "World".to_string(),
                style: MotdStyle::default(),
            },
        ];
        assert_eq!(to_plain_text(&nodes), "Hello World");
    }
}
