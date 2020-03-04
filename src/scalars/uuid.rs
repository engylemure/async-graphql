use crate::{Result, Scalar, Value};
use uuid::Uuid;

impl Scalar for Uuid {
    fn type_name() -> &'static str {
        "UUID"
    }

    fn parse(value: &Value) -> Option<Self> {
        match value {
            Value::String(s) => Some(Uuid::parse_str(&s).ok()?),
            _ => None,
        }
    }

    fn parse_from_json(value: &serde_json::Value) -> Option<Self> {
        match value {
            serde_json::Value::String(s) => Some(Uuid::parse_str(&s).ok()?),
            _ => None,
        }
    }

    fn to_json(&self) -> Result<serde_json::Value> {
        Ok(self.to_string().into())
    }
}