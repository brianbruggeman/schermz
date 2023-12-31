use serde_json::Value as JsonValue;

use crate::{Schema, SchemaObject, SchemaValueType};

#[derive(Debug, Clone)]
pub enum ValueType {
    Null,
    Bool,
    Number,
    String(usize),
    Object(SchemaObject),
    Array(Vec<ValueType>),
}

impl ValueType {
    pub fn from_json(json: &JsonValue) -> Self {
        match json {
            JsonValue::Null => Self::Null,
            JsonValue::Bool(_) => Self::Bool,
            JsonValue::Number(_) => Self::Number,
            JsonValue::String(_) => {
                let str = json.as_str().unwrap();
                Self::String(str.len())
            }
            JsonValue::Object(_) => Self::Object(SchemaObject::from_json(json)),
            JsonValue::Array(arr) => {
                let values = arr.iter().map(Self::from_json).collect();
                Self::Array(values)
            }
        }
    }

    pub fn to_schema_value_type(&self, merge_objects: bool) -> SchemaValueType {
        match self {
            ValueType::Null => SchemaValueType::Primitive("NULL".into()),
            ValueType::Bool => SchemaValueType::Primitive("BOOL".into()),
            ValueType::Number => SchemaValueType::Primitive("NUMBER".into()),
            ValueType::Object(obj) => SchemaValueType::Object(Schema::from_objects("object".into(), vec![obj.clone()], merge_objects)),
            ValueType::Array(arr) => {
                let mut value_types = arr
                    .iter()
                    .map(|value_type| value_type.to_schema_value_type(merge_objects))
                    .collect::<Vec<SchemaValueType>>();

                value_types.dedup();

                SchemaValueType::Array(value_types)
            }
            _ => panic!("Invalid value type"),
        }
    }
}
