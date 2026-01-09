//! Subset picker for SET parameters
//!
//! Implements the subset picking syntax for hierarchical coded sets.

use serde_json::Value;

/// Subset picker for SET parameters
#[derive(Debug)]
pub struct SubsetPicker {
    selectors: Vec<Selector>,
}

#[derive(Debug)]
enum Selector {
    /// Level selector
    Level(LevelSelector),
    /// Exclusion level selector
    Exclusion(LevelSelector),
    /// Additive level selector
    Additive(LevelSelector),
}

#[derive(Debug)]
enum LevelSelector {
    /// Range selector [start-end) or (start-end] etc.
    Range {
        start: Option<String>,
        end: Option<String>,
        start_inclusive: bool,
        end_inclusive: bool,
    },
    /// Enumeration selector: code1,code2<subselector>,...
    Enumeration(Vec<(String, Option<Box<LevelSelector>>)>),
    /// Regex selector /pattern/flags
    Regex(String, String),
}

impl SubsetPicker {
    /// Create a new subset picker from selector string
    pub fn new(selector_str: &str) -> Self {
        let selectors = Self::parse_selectors(selector_str);
        Self { selectors }
    }

    /// Parse selector string into selectors
    fn parse_selectors(selector_str: &str) -> Vec<Selector> {
        if selector_str.is_empty() {
            return Vec::new();
        }

        let mut selectors = Vec::new();
        let parts: Vec<&str> = selector_str.split(':').filter(|s| !s.is_empty()).collect();

        for part in parts {
            if part.starts_with('!') {
                // Exclusion selector
                if let Some(level) = Self::parse_level_selector(&part[1..]) {
                    selectors.push(Selector::Exclusion(level));
                }
            } else if part.starts_with('+') {
                // Additive selector
                if let Some(level) = Self::parse_level_selector(&part[1..]) {
                    selectors.push(Selector::Additive(level));
                }
            } else {
                // Normal level selector
                if let Some(level) = Self::parse_level_selector(part) {
                    selectors.push(Selector::Level(level));
                }
            }
        }

        selectors
    }

    /// Parse a level selector
    fn parse_level_selector(s: &str) -> Option<LevelSelector> {
        if s.is_empty() {
            return None;
        }

        // Check for regex pattern
        if s.starts_with('/') {
            if let Some(end_slash) = s[1..].find('/') {
                let pattern = &s[1..end_slash + 1];
                let flags = &s[end_slash + 2..];
                return Some(LevelSelector::Regex(pattern.to_string(), flags.to_string()));
            }
        }

        // Check for range pattern
        if s.starts_with('[') || s.starts_with('(') {
            return Self::parse_range_selector(s);
        }

        // Otherwise, it's an enumeration
        Some(Self::parse_enumeration_selector(s))
    }

    /// Parse range selector
    fn parse_range_selector(s: &str) -> Option<LevelSelector> {
        let start_inclusive = s.starts_with('[');
        let end_inclusive = s.ends_with(']');

        // Remove brackets
        let inner = s
            .trim_start_matches(['[', '('])
            .trim_end_matches([']', ')']);

        // Split by -
        let parts: Vec<&str> = inner.splitn(2, '-').collect();

        let start = if parts[0].is_empty() {
            None
        } else {
            Some(parts[0].to_string())
        };

        let end = if parts.len() > 1 && !parts[1].is_empty() {
            Some(parts[1].to_string())
        } else {
            None
        };

        Some(LevelSelector::Range {
            start,
            end,
            start_inclusive,
            end_inclusive,
        })
    }

    /// Parse enumeration selector
    fn parse_enumeration_selector(s: &str) -> LevelSelector {
        let mut items = Vec::new();
        let mut current = String::new();
        let mut depth = 0;

        for c in s.chars() {
            match c {
                '<' => {
                    depth += 1;
                    current.push(c);
                }
                '>' => {
                    depth -= 1;
                    current.push(c);
                }
                ',' if depth == 0 => {
                    if !current.is_empty() {
                        items.push(Self::parse_enum_item(&current));
                        current.clear();
                    }
                }
                _ => current.push(c),
            }
        }

        if !current.is_empty() {
            items.push(Self::parse_enum_item(&current));
        }

        LevelSelector::Enumeration(items)
    }

    /// Parse a single enumeration item (code<subselector>)
    fn parse_enum_item(s: &str) -> (String, Option<Box<LevelSelector>>) {
        if let Some(start) = s.find('<') {
            let code = s[..start].to_string();
            let sub = &s[start + 1..s.len() - 1];
            let sub_selector = Self::parse_level_selector(sub).map(Box::new);
            (code, sub_selector)
        } else {
            (s.to_string(), None)
        }
    }

    /// Pick subset from a SET array
    pub fn pick(&self, set: &[Value]) -> Vec<Value> {
        if self.selectors.is_empty() {
            return set.to_vec();
        }

        let mut result = set.to_vec();

        for selector in &self.selectors {
            result = match selector {
                Selector::Level(level) => self.apply_level_selector(&result, level),
                Selector::Exclusion(level) => self.apply_exclusion_selector(&result, level),
                Selector::Additive(level) => {
                    let mut combined = result.clone();
                    combined.extend(self.apply_level_selector(set, level));
                    combined
                }
            };
        }

        result
    }

    /// Apply a level selector
    fn apply_level_selector(&self, set: &[Value], selector: &LevelSelector) -> Vec<Value> {
        match selector {
            LevelSelector::Range {
                start,
                end,
                start_inclusive,
                end_inclusive,
            } => self.apply_range_selector(set, start.as_deref(), end.as_deref(), *start_inclusive, *end_inclusive),
            LevelSelector::Enumeration(items) => {
                self.apply_enumeration_selector(set, items)
            }
            LevelSelector::Regex(pattern, _flags) => self.apply_regex_selector(set, pattern),
        }
    }

    /// Apply exclusion selector
    fn apply_exclusion_selector(&self, set: &[Value], selector: &LevelSelector) -> Vec<Value> {
        let to_exclude = self.apply_level_selector(set, selector);
        set.iter()
            .filter(|v| !to_exclude.contains(v))
            .cloned()
            .collect()
    }

    /// Apply range selector
    fn apply_range_selector(
        &self,
        set: &[Value],
        start: Option<&str>,
        end: Option<&str>,
        start_inclusive: bool,
        end_inclusive: bool,
    ) -> Vec<Value> {
        set.iter()
            .filter(|v| {
                let code = v.get("code").and_then(|c| c.as_str()).unwrap_or("");

                let start_ok = match start {
                    Some(s) => {
                        if start_inclusive {
                            code >= s
                        } else {
                            code > s
                        }
                    }
                    None => true,
                };

                let end_ok = match end {
                    Some(e) => {
                        if end_inclusive {
                            code <= e
                        } else {
                            code < e
                        }
                    }
                    None => true,
                };

                start_ok && end_ok
            })
            .cloned()
            .collect()
    }

    /// Apply enumeration selector
    fn apply_enumeration_selector(
        &self,
        set: &[Value],
        items: &[(String, Option<Box<LevelSelector>>)],
    ) -> Vec<Value> {
        let mut result = Vec::new();

        for (code, sub_selector) in items {
            for v in set {
                let item_code = v.get("code").and_then(|c| c.as_str()).unwrap_or("");

                if item_code == code || item_code.starts_with(&format!("{}.", code)) {
                    if let Some(sub) = sub_selector {
                        // Apply sub-selector to children
                        if item_code.starts_with(&format!("{}.", code)) {
                            let matches = self.apply_level_selector(&[v.clone()], sub);
                            result.extend(matches);
                        } else {
                            result.push(v.clone());
                        }
                    } else {
                        result.push(v.clone());
                    }
                }
            }
        }

        result
    }

    /// Apply regex selector
    fn apply_regex_selector(&self, set: &[Value], pattern: &str) -> Vec<Value> {
        let re = match regex::Regex::new(pattern) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };

        set.iter()
            .filter(|v| {
                let code = v.get("code").and_then(|c| c.as_str()).unwrap_or("");
                re.is_match(code)
            })
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_test_set() -> Vec<Value> {
        vec![
            json!({"code": "1", "value": "value1"}),
            json!({"code": "1.Q-1", "value": "value1.1"}),
            json!({"code": "1.Q-2", "value": "value1.2"}),
            json!({"code": "2", "value": "value2"}),
            json!({"code": "2.Q-2", "value": "value2.1"}),
            json!({"code": "2.Q-3.1", "value": "value2.1.1"}),
            json!({"code": "2.Q-4", "value": "value2.2"}),
            json!({"code": "3", "value": "value3"}),
            json!({"code": "4", "value": "value4"}),
        ]
    }

    #[test]
    fn test_empty_selector() {
        let set = create_test_set();
        let picker = SubsetPicker::new("");
        let result = picker.pick(&set);
        assert_eq!(result.len(), set.len());
    }

    #[test]
    fn test_enumeration_selector() {
        let set = create_test_set();
        let picker = SubsetPicker::new(":1,4");
        let result = picker.pick(&set);

        let codes: Vec<&str> = result
            .iter()
            .filter_map(|v| v.get("code").and_then(|c| c.as_str()))
            .collect();

        assert!(codes.contains(&"1"));
        assert!(codes.contains(&"1.Q-1"));
        assert!(codes.contains(&"1.Q-2"));
        assert!(codes.contains(&"4"));
        assert!(!codes.contains(&"2"));
        assert!(!codes.contains(&"3"));
    }

    #[test]
    fn test_range_selector() {
        let set = create_test_set();
        let picker = SubsetPicker::new(":[1-3]");
        let result = picker.pick(&set);

        let codes: Vec<&str> = result
            .iter()
            .filter_map(|v| v.get("code").and_then(|c| c.as_str()))
            .collect();

        assert!(codes.contains(&"1"));
        assert!(codes.contains(&"2"));
        assert!(codes.contains(&"3"));
        assert!(!codes.contains(&"4"));
    }
}
