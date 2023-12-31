//! # schermz - JSON Schema Inference Library
//!
//! `schermz` is a Rust library designed to analyze JSON data and generate a comprehensive schema representation.
//! It's particularly useful for understanding the structure of large and complex JSON files, offering insights
//! into the various data types and their occurrences.
//!
//! ## Core Features
//!
//! - **Schema Generation**: Create a schema from JSON data, revealing the structure and types present in the JSON.
//! - **Object Merging**: Option to merge similar objects into a unified schema representation.
//! - **Type Analysis**: Detailed analysis of value types including strings, numbers, arrays, and objects.
//! - **Flexible Array Handling**: Arrays are processed to identify the diversity of contained types.
//! - **Nested Object Support**: Handles complex nested structures within JSON data.
//!
//! ## Modules
//!
//! - `schema`: Defines the `Schema` struct and related functionality for schema creation and manipulation.
//! - `schema_object`: Represents individual JSON objects within the schema.
//! - `schema_object_key`: Handles keys in JSON objects, aiding in structuring the schema.
//! - `schema_value_type`: Enumerates possible types for values in the schema (e.g., string, number, object).
//! - `value_type`: Identifies and categorizes the types of values found in the JSON data.
//!
//! ## Usage
//!
//! The library provides a straightforward API to convert JSON data into a schema representation:
//!
//! ```rust
//! use schermz::Schema;
//! use serde_json::json;
//!
//! let json_data = json!({
//!     // JSON data here
//! });
//! let schema = Schema::from_json(&json_data, true);
//! ```
//!
//! The `from_json` function is the primary entry point, taking a `serde_json::Value` and a boolean flag
//! to indicate whether similar objects should be merged.
//!
//! ## Testing
//!
//! The library includes comprehensive tests covering various scenarios and JSON structures, ensuring reliability
//! and correctness of the schema generation process.
//!
//! ## Note
//!
//! This library is part of the `schermz` toolset but focuses on the programmatic aspect of schema generation,
//! making it suitable for integration into other Rust applications.
//!
//! ## License
//!
//! `schermz` is open-source and available under [license terms here].
//!
//! ## Contributions
//!
//! Contributions to the library are welcome. Please follow the contribution guidelines outlined in the repository.
//!
//! [End of docstring]

mod schema;
mod schema_object;
mod schema_object_key;
mod schema_value_type;
mod value_type;

pub use schema::Schema;
pub use schema_object::SchemaObject;
pub use schema_object_key::SchemaObjectKey;
pub use schema_value_type::SchemaValueType;
pub use value_type::ValueType;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_input() {
        Schema::from_json(&serde_json::Value::Null, false);
    }

    #[test]
    fn test_schema_from_object() {
        let json = serde_json::json!({
            "name": "John Doe",
            "title": "",
            "age": 43,
            "address": {
                "street": "10 Downing Street",
                "city": "London"
            },
            "phones": [
                "+44 1234567",
                "+44 2345678",
                123456,
                { "mobile": "+44 3456789" }
            ]
        });

        insta::assert_json_snapshot!(Schema::from_json(&json, true).to_json());
    }

    #[test]
    fn test_schema_from_array_merged() {
        let json = serde_json::json!([
            {
                "name": "Sherlock Holmes",
                "title": "",
                "age": 34,
                "personal_data": {
                    "gender": "male",
                    "marital_status": "single",
                },
                "address": {
                    "street": "10 Downing Street",
                    "city": "London",
                    "zip": "12345",
                    "country_code": "UK",
                },
                "phones": [
                    "+44 1234567",
                    "+44 2345678",
                    12311,
                    { "mobile": "+44 3456789" }
                ]
            },
            {
                "name": "Tony Soprano",
                "title": "",
                "age": 39,
                "personal_data": {
                    "gender": "male",
                    "marital_status": "married",
                },
                "address": {
                    "street": "14 Aspen Drive",
                    "city": "Caldwell",
                    "zip": "NJ 07006",
                    "country": "USA",
                    "state": "New Jersey",
                    "country_code": "US",
                },
                "phones": [
                    "+1 1234567",
                    "+1 2345678",
                    "+1 11111111111",
                    "+1 301234566",
                    11224234,
                    { "mobile": "+1 3456789" }
                ]
            },
            {
                "name": "Angela Merkel",
                "title": "",
                "age": 65,
                "personal_data": {
                    "gender": "female",
                    "marital_status": "married",
                },
                "address": {
                    "street": "Gr. Weg 3",
                    "city": "Potsdam",
                    "zip": "14467",
                    "country": "Germany",
                    "state": "Brandenburg",

                },
                "phones": [
                    "+49 1234222567",
                    "+49 2343231678",
                    "+49 1111131111111",
                    "+49 301212334566",
                    9999222,
                    { "mobile": "+49 343156789", "fax": "+49 343156780" }
                ]
            },
            {
                "name": "Jane Doe",
                "title": "Dr.",
                "age": "73",
                "personal_data": {
                    "gender": "female",
                },
                "address": null,
                "phones": null
            }
        ]);

        insta::assert_json_snapshot!(Schema::from_json(&json, true).to_json());
    }

    #[test]
    fn test_schema_from_array_unmerged() {
        let json = serde_json::json!([
            {
                "name": "Sherlock Holmes",
                "title": "",
                "age": 34,
                "personal_data": {
                    "gender": "male",
                    "marital_status": "single",
                },
                "address": {
                    "street": "10 Downing Street",
                    "city": "London",
                    "zip": "12345",
                    "country_code": "UK",
                },
                "phones": [
                    "+44 1234567",
                    "+44 2345678",
                    12311,
                    { "mobile": "+44 3456789" }
                ]
            },
            {
                "name": "Tony Soprano",
                "title": "",
                "age": 39,
                "personal_data": {
                    "gender": "male",
                    "marital_status": "married",
                },
                "address": {
                    "street": "14 Aspen Drive",
                    "city": "Caldwell",
                    "zip": "NJ 07006",
                    "country": "USA",
                    "state": "New Jersey",
                    "country_code": "US",
                },
                "phones": [
                    "+1 1234567",
                    "+1 2345678",
                    "+1 11111111111",
                    "+1 301234566",
                    11224234,
                    { "mobile": "+1 3456789" }
                ]
            },
            {
                "name": "Angela Merkel",
                "title": "",
                "age": 65,
                "personal_data": {
                    "gender": "female",
                    "marital_status": "married",
                },
                "address": {
                    "street": "Gr. Weg 3",
                    "city": "Potsdam",
                    "zip": "14467",
                    "country": "Germany",
                    "state": "Brandenburg",

                },
                "phones": [
                    "+49 1234222567",
                    "+49 2343231678",
                    "+49 1111131111111",
                    "+49 301212334566",
                    9999222,
                    { "mobile": "+49 343156789", "fax": "+49 343156780" }
                ]
            },
            {
                "name": "Jane Doe",
                "title": "Dr.",
                "age": "73",
                "personal_data": {
                    "gender": "female",
                },
                "address": null,
                "phones": null
            }
        ]);

        insta::assert_json_snapshot!(Schema::from_json(&json, false).to_json());
    }
}