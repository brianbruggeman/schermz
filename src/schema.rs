use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use itertools::Itertools;
use serde_json::Value as JsonValue;

use crate::{SchemaObject, SchemaValueType, ValueType};

#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub name: String,
    pub map: HashMap<String, Vec<SchemaValueType>>,
}

type CollectedObjects = HashMap<String, Vec<SchemaObject>>;

impl Schema {
    // Groups objects by a hash created from their keys
    fn group_objects_by_keys_fingerprint(objects: Vec<SchemaObject>) -> Vec<Vec<SchemaObject>> {
        objects
            .into_iter()
            .group_by(|obj| {
                let mut hasher = DefaultHasher::new();
                let sorted_keys = obj
                    .keys
                    .clone()
                    .into_iter()
                    .map(|obj_key| obj_key.id)
                    .sorted()
                    .collect::<Vec<String>>();
                let stringified_keys = sorted_keys.join("");
                stringified_keys.hash(&mut hasher);
                hasher.finish()
            })
            .into_iter()
            .map(|(_, gr)| gr.collect_vec())
            .collect()
    }

    fn create_map(objects: Vec<SchemaObject>, merge_objects: bool) -> HashMap<String, Vec<SchemaValueType>> {
        let mut map = HashMap::<String, Vec<SchemaValueType>>::new();
        let mut string_lens = HashMap::<String, Vec<usize>>::new();
        let mut object_types = CollectedObjects::new();
        let mut array_object_types = CollectedObjects::new();
        let mut array_primitive_types_map = HashMap::<String, Vec<SchemaValueType>>::new();
        let mut array_string_lens_map = HashMap::<String, Vec<usize>>::new();

        for obj in objects {
            for key in &obj.keys {
                match &key.v_type {
                    ValueType::Object(obj) => {
                        // Collect all objects with the same key id into a vector
                        // so we can merge them together into a single schema
                        object_types
                            .entry(key.id.clone())
                            .or_default()
                            .push(obj.clone());
                    }
                    ValueType::Array(arr) => {
                        for value_type in arr {
                            match value_type {
                                ValueType::Object(obj) => {
                                    array_object_types
                                        .entry(key.id.clone())
                                        .or_default()
                                        .push(obj.clone());
                                }
                                ValueType::String(len) => {
                                    array_string_lens_map
                                        .entry(key.id.clone())
                                        .or_default()
                                        .push(*len);
                                }
                                primitive_type => {
                                    let entry = array_primitive_types_map
                                        .entry(key.id.clone())
                                        .or_default();
                                    let vtype = primitive_type.to_schema_value_type(merge_objects);
                                    if !entry.contains(&vtype) {
                                        entry.push(vtype);
                                    }
                                }
                            }
                        }
                    }
                    ValueType::String(len) => {
                        string_lens
                            .entry(key.id.clone())
                            .or_default()
                            .push(*len);
                    }
                    primitive_type => {
                        let entry = map.entry(key.id.clone()).or_default();
                        let vtype = primitive_type.to_schema_value_type(merge_objects);
                        if !entry.contains(&vtype) {
                            entry.push(vtype);
                        }
                    }
                }
            }
        }

        for (key, value) in string_lens {
            let min = value.iter().min().unwrap();
            let max = value.iter().max().unwrap();
            map.entry(key)
                .or_default()
                .push(SchemaValueType::String(*min, *max));
        }

        for (key, value) in object_types {
            match merge_objects {
                true => {
                    let name = key.clone();
                    map.entry(key)
                        .or_default()
                        .push(SchemaValueType::Object(Schema::from_objects(name, value, true)));
                }
                false => {
                    for objects_group in Self::group_objects_by_keys_fingerprint(value) {
                        map.entry(key.clone())
                            .or_default()
                            .push(SchemaValueType::Object(Schema::from_objects(key.clone(), objects_group, false)));
                    }
                }
            }
        }

        for (key, value) in array_object_types {
            let mut all_array_types = Vec::new();

            match merge_objects {
                true => {
                    let schema = Schema::from_objects(key.clone(), value, true);
                    all_array_types = vec![SchemaValueType::Object(schema)];
                }
                false => {
                    for objects_group in Self::group_objects_by_keys_fingerprint(value) {
                        all_array_types.push(SchemaValueType::Object(Schema::from_objects(key.clone(), objects_group, false)));
                    }
                }
            }

            if let Some(primitive_types) = array_primitive_types_map.get_mut(&key) {
                all_array_types.append(primitive_types);
            }
            if let Some(string_lens) = array_string_lens_map.get_mut(&key) {
                let min = string_lens.iter().min().unwrap();
                let max = string_lens.iter().max().unwrap();
                all_array_types.push(SchemaValueType::String(*min, *max));
            }
            map.entry(key)
                .or_default()
                .push(SchemaValueType::Array(all_array_types));
        }

        map
    }

    pub(crate) fn from_objects(name: String, objects: Vec<SchemaObject>, merge_objects: bool) -> Self {
        Self {
            name,
            map: Self::create_map(objects, merge_objects),
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

    pub fn from_json(json: &JsonValue, merge_objects: bool) -> Self {
        match json {
            JsonValue::Object(_) => Self::from_objects("root".into(), vec![SchemaObject::from_json(json)], merge_objects),
            JsonValue::Array(_) => {
                let objects = json
                    .as_array()
                    .unwrap()
                    .iter()
                    .filter_map(|el| match el {
                        JsonValue::Object(_) => Some(SchemaObject::from_json(el)),
                        _ => None,
                    })
                    .collect::<Vec<SchemaObject>>();

                Self::from_objects("root".into(), objects, merge_objects)
            }
            _ => panic!("Invalid JSON"),
        }
    }
}