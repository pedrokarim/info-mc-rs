use crate::types::MotdNode;

/// Render MOTD nodes to HTML with inline styles.
pub fn to_html(nodes: &[MotdNode]) -> String {
    let mut html = String::new();

    for node in nodes {
        if node.text.is_empty() {
            continue;
        }

        let mut styles = Vec::new();

        if let Some(ref color) = node.style.color {
            styles.push(format!("color:{}", color.to_css()));
        }
        if node.style.bold {
            styles.push("font-weight:bold".to_string());
        }
        if node.style.italic {
            styles.push("font-style:italic".to_string());
        }

        let mut decorations = Vec::new();
        if node.style.underlined {
            decorations.push("underline");
        }
        if node.style.strikethrough {
            decorations.push("line-through");
        }
        if !decorations.is_empty() {
            styles.push(format!("text-decoration:{}", decorations.join(" ")));
        }

        let escaped_text = html_escape(&node.text);

        if node.style.obfuscated {
            html.push_str(&format!(
                "<span class=\"mc-obfuscated\" style=\"{}\">{}</span>",
                styles.join(";"),
                escaped_text
            ));
        } else if styles.is_empty() {
            html.push_str(&escaped_text);
        } else {
            html.push_str(&format!(
                "<span style=\"{}\">{}</span>",
                styles.join(";"),
                escaped_text
            ));
        }
    }

    html
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{McColor, MotdStyle};

    #[test]
    fn test_simple_html() {
        let nodes = vec![
            MotdNode {
                text: "Hello ".to_string(),
                style: MotdStyle {
                    color: Some(McColor::Named("gold".to_string())),
                    bold: true,
                    ..Default::default()
                },
            },
            MotdNode {
                text: "World".to_string(),
                style: MotdStyle {
                    color: Some(McColor::Named("green".to_string())),
                    ..Default::default()
                },
            },
        ];

        let html = to_html(&nodes);
        assert!(html.contains("color:#FFAA00"));
        assert!(html.contains("font-weight:bold"));
        assert!(html.contains("color:#55FF55"));
        assert!(html.contains("Hello "));
        assert!(html.contains("World"));
    }

    #[test]
    fn test_html_escape() {
        let nodes = vec![MotdNode {
            text: "<script>alert('xss')</script>".to_string(),
            style: MotdStyle::default(),
        }];
        let html = to_html(&nodes);
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;"));
    }
}
