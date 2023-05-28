use serde_json::Value as JsonValue;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum ValueType {
    Null,
    Bool,
    Number,
    String,
    Object(SchemaObject),
    Array(Vec<ValueType>),
}

#[derive(Debug, Clone)]
struct SchemaObjectKey {
    pub id: String,
    pub v_type: ValueType,
}

#[derive(Debug, Clone)]
pub struct SchemaObject {
    keys: Vec<SchemaObjectKey>,
}

impl ValueType {
    pub fn from_json(json: &JsonValue) -> Self {
        match json {
            JsonValue::Null => Self::Null,
            JsonValue::Bool(_) => Self::Bool,
            JsonValue::Number(_) => Self::Number,
            JsonValue::String(_) => Self::String,
            JsonValue::Object(_) => Self::Object(SchemaObject::from_json(json)),
            JsonValue::Array(arr) => {
                let values = arr.iter().map(Self::from_json).collect();
                Self::Array(values)
            }
        }
    }

    pub fn to_schema_value_type(&self) -> SchemaValueType {
        match self {
            ValueType::Null => SchemaValueType::Primitive("NULL".into()),
            ValueType::Bool => SchemaValueType::Primitive("BOOL".into()),
            ValueType::Number => SchemaValueType::Primitive("NUMBER".into()),
            ValueType::String => SchemaValueType::Primitive("STRING".into()),
            ValueType::Object(obj) => {
                SchemaValueType::Object(Schema::from_objects("object".into(), vec![obj.clone()]))
            }
            ValueType::Array(arr) => {
                let mut value_types = arr
                    .iter()
                    .map(|value_type| value_type.to_schema_value_type())
                    .collect::<Vec<SchemaValueType>>();

                value_types.dedup();

                SchemaValueType::Array(value_types)
            }
        }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub enum SchemaValueType {
    Primitive(String),
    Array(Vec<SchemaValueType>),
    Object(Schema),
}

impl SchemaValueType {
    pub fn to_json(&self) -> JsonValue {
        match self {
            SchemaValueType::Primitive(name) => JsonValue::String(name.clone()),
            SchemaValueType::Array(types) => {
                let mut arr = Vec::new();
                for vtype in types {
                    arr.push(vtype.to_json());
                }
                JsonValue::Array(arr)
            }
            SchemaValueType::Object(schema) => schema.to_json(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub name: String,
    pub map: HashMap<String, Vec<SchemaValueType>>,
}

type CollectedObjects = HashMap<String, Vec<SchemaObject>>;

impl Schema {
    fn create_map(objects: Vec<SchemaObject>) -> HashMap<String, Vec<SchemaValueType>> {
        let mut map = HashMap::<String, Vec<SchemaValueType>>::new();
        let mut object_types = CollectedObjects::new();

        for obj in objects {
            for key in &obj.keys {
                match &key.v_type {
                    ValueType::Object(obj) => {
                        // Collect all objects with the same key id into a vector
                        // so we can merge them together into a single schema
                        object_types
                            .entry(key.id.clone())
                            .or_insert_with(Vec::new)
                            .push(obj.clone());
                    }
                    primitive_type => {
                        let entry = map.entry(key.id.clone()).or_insert_with(Vec::new);
                        let vtype = primitive_type.to_schema_value_type();
                        if !entry.contains(&vtype) {
                            entry.push(vtype);
                        }
                    }
                }
            }
        }

        for (key, value) in object_types {
            let name = key.clone();
            map.entry(key)
                .or_insert_with(Vec::new)
                .push(SchemaValueType::Object(Schema::from_objects(name, value)));
        }

        map
    }

    fn from_objects(name: String, objects: Vec<SchemaObject>) -> Self {
        Self {
            name,
            map: Self::create_map(objects),
        }
    }

    pub fn to_json(&self) -> JsonValue {
        let mut map = serde_json::Map::new();

        for (key, value) in &self.map {
            let mut entry = serde_json::Map::new();
            let mut types = Vec::new();

            for vtype in value {
                types.push(vtype.to_json());
            }

            entry.insert("types".into(), serde_json::Value::Array(types));
            map.insert(key.clone(), serde_json::Value::Object(entry));
        }

        serde_json::Value::Object(map)
    }

    pub fn from_json(json: &JsonValue) -> Self {
        match json {
            JsonValue::Object(_) => {
                Self::from_objects("root".into(), vec![SchemaObject::from_json(json)])
            }
            JsonValue::Array(_) => {
                let objects = json
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(SchemaObject::from_json)
                    .collect::<Vec<SchemaObject>>();

                Self::from_objects("root".into(), objects)
            }
            _ => panic!("Invalid input JSON"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_input() {
        Schema::from_json(&serde_json::Value::Null);
    }

    #[test]
    fn test_schema_from_object() {
        let json = serde_json::json!({
            "name": "John Doe",
            "age": 43,
            "address": {
                "street": "10 Downing Street",
                "city": "London"
            },
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        });

        insta::assert_yaml_snapshot!(Schema::from_json(&json).to_json());
    }

    #[test]
    fn test_schema_from_array() {
        let json = serde_json::json!([
            {
                "name": "John Doe",
                "age": 43,
                "address": {
                    "street": "10 Downing Street",
                    "city": "London"
                },
                "phones": [
                    "+44 1234567",
                    "+44 2345678"
                ]
            },
            {
                "name": "Jane Doe",
                "age": "66",
                "address": null,
                "phones": null
            }
        ]);

        insta::assert_yaml_snapshot!(Schema::from_json(&json).to_json());
    }
}
