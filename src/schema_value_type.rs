use serde_json::Value as JsonValue;

use crate::Schema;

#[derive(Debug, Clone, PartialEq)]
pub enum SchemaValueType {
    Primitive(String),
    String(usize, usize),
    Array(Vec<SchemaValueType>),
    Object(Schema),
}

impl SchemaValueType {
    pub fn to_json(&self) -> JsonValue {
        match self {
            SchemaValueType::Primitive(name) => JsonValue::String(name.clone()),
            SchemaValueType::String(min, max) => {
                if min == max {
                    return JsonValue::String(format!("STRING({})", min));
                }

                JsonValue::String(format!("STRING({}, {})", min, max))
            }
            SchemaValueType::Array(v_types) => {
                let types = v_types
                    .iter()
                    .map(|v| v.to_json())
                    .collect::<Vec<JsonValue>>();

                serde_json::json!({ "ARRAY": types })
            }
            SchemaValueType::Object(schema) => schema.to_json(),
        }
    }
}
