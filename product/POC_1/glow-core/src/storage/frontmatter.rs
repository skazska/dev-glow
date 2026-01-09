//! Frontmatter parsing and rendering
//!
//! Handles YAML frontmatter in Markdown files.

use crate::error::{GlowError, Result};
use crate::model::Step;

/// Frontmatter delimiter
const FRONTMATTER_DELIMITER: &str = "---";

/// Parse frontmatter from a markdown file content
pub fn parse_frontmatter(content: &str) -> Result<(serde_json::Value, String)> {
    let trimmed = content.trim_start();

    if !trimmed.starts_with(FRONTMATTER_DELIMITER) {
        return Ok((serde_json::Value::Null, content.to_string()));
    }

    // Find the end of frontmatter
    let after_first_delimiter = &trimmed[FRONTMATTER_DELIMITER.len()..];
    
    // Skip the newline after the first delimiter
    let after_newline = if after_first_delimiter.starts_with('\n') {
        &after_first_delimiter[1..]
    } else if after_first_delimiter.starts_with("\r\n") {
        &after_first_delimiter[2..]
    } else {
        after_first_delimiter
    };

    // Find the closing delimiter
    if let Some(end_pos) = after_newline.find(&format!("\n{}", FRONTMATTER_DELIMITER)) {
        let yaml_content = &after_newline[..end_pos];
        let remaining_start = end_pos + 1 + FRONTMATTER_DELIMITER.len();
        let remaining = if remaining_start < after_newline.len() {
            // Skip newline after closing delimiter
            let rest = &after_newline[remaining_start..];
            if rest.starts_with('\n') {
                &rest[1..]
            } else if rest.starts_with("\r\n") {
                &rest[2..]
            } else {
                rest
            }
        } else {
            ""
        };

        let frontmatter: serde_json::Value = serde_yaml::from_str(yaml_content)?;
        Ok((frontmatter, remaining.to_string()))
    } else {
        Err(GlowError::InvalidFrontmatter {
            path: std::path::PathBuf::from("<content>"),
        })
    }
}

/// Render frontmatter to markdown format
pub fn render_frontmatter(frontmatter: &serde_json::Value, content: &str) -> Result<String> {
    if frontmatter.is_null() {
        return Ok(content.to_string());
    }

    let yaml = serde_yaml::to_string(frontmatter)?;
    
    Ok(format!(
        "{}\n{}{}\n{}",
        FRONTMATTER_DELIMITER,
        yaml,
        FRONTMATTER_DELIMITER,
        content
    ))
}

/// Parse a step from frontmatter
pub fn parse_step_frontmatter(content: &str) -> Result<(Step, String)> {
    let (frontmatter, body) = parse_frontmatter(content)?;
    
    if frontmatter.is_null() {
        return Err(GlowError::InvalidFrontmatter {
            path: std::path::PathBuf::from("<content>"),
        });
    }

    let step: Step = serde_json::from_value(frontmatter)?;
    Ok((step, body))
}

/// Render a step to markdown with frontmatter
pub fn render_step_frontmatter(step: &Step, content: &str) -> Result<String> {
    let frontmatter = serde_json::to_value(step)?;
    render_frontmatter(&frontmatter, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_frontmatter() {
        let content = r#"---
title: Test
value: 42
---

# Content here
"#;
        let (fm, body) = parse_frontmatter(content).unwrap();
        assert_eq!(fm["title"], "Test");
        assert_eq!(fm["value"], 42);
        assert!(body.contains("# Content here"));
    }

    #[test]
    fn test_parse_no_frontmatter() {
        let content = "# Just content\n\nNo frontmatter here.";
        let (fm, body) = parse_frontmatter(content).unwrap();
        assert!(fm.is_null());
        assert_eq!(body, content);
    }

    #[test]
    fn test_render_frontmatter() {
        let fm = serde_json::json!({
            "title": "Test",
            "value": 42
        });
        let result = render_frontmatter(&fm, "# Content").unwrap();
        assert!(result.starts_with("---"));
        assert!(result.contains("title: Test"));
        assert!(result.contains("# Content"));
    }

    #[test]
    fn test_roundtrip() {
        let original_fm = serde_json::json!({
            "attr": {
                "id": "TEST-001",
                "status": "todo"
            }
        });
        let original_content = "# Test step\n\nDescription here.";

        let rendered = render_frontmatter(&original_fm, original_content).unwrap();
        let (parsed_fm, parsed_content) = parse_frontmatter(&rendered).unwrap();

        assert_eq!(parsed_fm["attr"]["id"], "TEST-001");
        assert!(parsed_content.contains("# Test step"));
    }
}
