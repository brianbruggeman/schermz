use serde_json::Value as JsonValue;

use crate::{SchemaObjectKey, ValueType};

#[derive(Debug, Clone)]
pub struct SchemaObject {
    pub(crate) keys: Vec<SchemaObjectKey>,
}

impl SchemaObject {
    pub fn from_json(json: &JsonValue) -> Self {
        let mut keys = Vec::new();

        for (key, value) in json.as_object().unwrap() {
            keys.push(SchemaObjectKey {
                id: key.clone(),
                v_type: ValueType::from_json(value),
            });
        }
        Self { keys }
    }
}
