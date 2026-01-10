//! Custom Handlebars helpers
//!
//! Implements helpers for subset picking and other operations.

use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
    RenderErrorReason, ScopedJson,
};
use serde_json::Value;

use super::subset::SubsetPicker;

/// Register all custom helpers
pub fn register_helpers(hb: &mut Handlebars) {
    hb.register_helper("subset", Box::new(SubsetHelper));
    hb.register_helper("eq", Box::new(EqHelper));
    hb.register_helper("ne", Box::new(NeHelper));
    hb.register_helper("contains", Box::new(ContainsHelper));
    hb.register_helper("default", Box::new(DefaultHelper));
    hb.register_helper("json", Box::new(JsonHelper));
    hb.register_helper("join", Box::new(JoinHelper));
}

/// Subset helper for SET parameter subsetting
/// Usage: {{subset set_param ":selector1:selector2"}}
struct SubsetHelper;

impl HelperDef for SubsetHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let set_value = h
            .param(0)
            .ok_or(RenderErrorReason::ParamNotFoundForIndex("subset", 0))?;

        let selector = h
            .param(1)
            .and_then(|v| v.value().as_str())
            .unwrap_or("");

        let set_array = set_value.value().as_array().ok_or(
            RenderErrorReason::ParamTypeMismatchForName("subset", "0".into(), "array".into())
        )?;

        let picker = SubsetPicker::new(selector);
        let result = picker.pick(set_array);

        // Render as YAML-like list
        for item in result {
            if let (Some(code), Some(value)) = (item.get("code"), item.get("value")) {
                out.write(&format!(
                    "- {}: {}\n",
                    code.as_str().unwrap_or(""),
                    value.as_str().unwrap_or("")
                ))?;
            }
        }

        Ok(())
    }
}

/// Equality helper
/// Usage: {{#if (eq a b)}}...{{/if}}
struct EqHelper;

impl HelperDef for EqHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let a = h.param(0).map(|v| v.value());
        let b = h.param(1).map(|v| v.value());

        let result = match (a, b) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        };

        Ok(ScopedJson::Derived(Value::Bool(result)))
    }
}

/// Not-equal helper
/// Usage: {{#if (ne a b)}}...{{/if}}
struct NeHelper;

impl HelperDef for NeHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let a = h.param(0).map(|v| v.value());
        let b = h.param(1).map(|v| v.value());

        let result = match (a, b) {
            (Some(a), Some(b)) => a != b,
            _ => true,
        };

        Ok(ScopedJson::Derived(Value::Bool(result)))
    }
}

/// Contains helper for arrays
/// Usage: {{#if (contains array item)}}...{{/if}}
struct ContainsHelper;

impl HelperDef for ContainsHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'rc>, RenderError> {
        let arr = h.param(0).and_then(|v| v.value().as_array());
        let item = h.param(1).map(|v| v.value());

        let result = match (arr, item) {
            (Some(arr), Some(item)) => arr.contains(item),
            _ => false,
        };

        Ok(ScopedJson::Derived(Value::Bool(result)))
    }
}

/// Default value helper
/// Usage: {{default value "fallback"}}
struct DefaultHelper;

impl HelperDef for DefaultHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let value = h.param(0).map(|v| v.value());
        let default = h.param(1).map(|v| v.value());

        let result = match value {
            Some(v) if !v.is_null() && v != &Value::String(String::new()) => v,
            _ => default.unwrap_or(&Value::Null),
        };

        match result {
            Value::String(s) => out.write(s)?,
            Value::Number(n) => out.write(&n.to_string())?,
            Value::Bool(b) => out.write(&b.to_string())?,
            _ => out.write(&result.to_string())?,
        }

        Ok(())
    }
}

/// JSON stringify helper
/// Usage: {{json value}}
struct JsonHelper;

impl HelperDef for JsonHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let value = h
            .param(0)
            .map(|v| v.value())
            .unwrap_or(&Value::Null);

        let json_str = serde_json::to_string_pretty(value)
            .map_err(|e| RenderErrorReason::Other(format!("json: {}", e)))?;

        out.write(&json_str)?;
        Ok(())
    }
}

/// Join array helper
/// Usage: {{join array ", "}}
struct JoinHelper;

impl HelperDef for JoinHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let arr = h.param(0).and_then(|v| v.value().as_array());
        let sep = h
            .param(1)
            .and_then(|v| v.value().as_str())
            .unwrap_or(", ");

        if let Some(arr) = arr {
            let strings: Vec<String> = arr
                .iter()
                .map(|v| match v {
                    Value::String(s) => s.clone(),
                    _ => v.to_string(),
                })
                .collect();
            out.write(&strings.join(sep))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_helper() {
        let mut hb = Handlebars::new();
        register_helpers(&mut hb);
        hb.register_template_string("test", "{{#if (eq a b)}}equal{{else}}not equal{{/if}}")
            .unwrap();

        let data = serde_json::json!({"a": 1, "b": 1});
        let result = hb.render("test", &data).unwrap();
        assert_eq!(result, "equal");

        let data = serde_json::json!({"a": 1, "b": 2});
        let result = hb.render("test", &data).unwrap();
        assert_eq!(result, "not equal");
    }

    #[test]
    fn test_default_helper() {
        let mut hb = Handlebars::new();
        register_helpers(&mut hb);
        hb.register_template_string("test", "{{default value \"fallback\"}}")
            .unwrap();

        let data = serde_json::json!({"value": "actual"});
        let result = hb.render("test", &data).unwrap();
        assert_eq!(result, "actual");

        let data = serde_json::json!({});
        let result = hb.render("test", &data).unwrap();
        assert_eq!(result, "fallback");
    }

    #[test]
    fn test_join_helper() {
        let mut hb = Handlebars::new();
        register_helpers(&mut hb);
        hb.register_template_string("test", "{{join items \", \"}}")
            .unwrap();

        let data = serde_json::json!({"items": ["a", "b", "c"]});
        let result = hb.render("test", &data).unwrap();
        assert_eq!(result, "a, b, c");
    }
}
