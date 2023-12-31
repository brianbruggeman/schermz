use crate::ValueType;

#[derive(Debug, Clone)]
pub struct SchemaObjectKey {
    pub id: String,
    pub v_type: ValueType,
}
