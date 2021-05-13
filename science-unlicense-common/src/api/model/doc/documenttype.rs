//
// Public Domain - unlicense.science
//

use crate::api::model::Presentable;
use std::collections::HashMap;
use crate::api::model::doc::FieldType;

pub trait DocumentType : Presentable {

    ///
    /// Indicate if document type is strict.
    /// Document created with a strict type can not have any extra fields.
    ///
    /// @return true if strict
    ///
    fn is_strict(&self) -> bool;

    ///
    /// List document local field types.
    /// This include only this type fields, not parent fields.
    ///
    /// @return FieldType array, never null, can be empty.
    ///
    fn get_local_fields(&self) -> &Vec<Box<dyn FieldType>>;

    ///
    /// List document field types.
    /// This include this type and parent types.
    ///
    /// @return FieldType array, never null, can be empty.
    ///
    fn get_fields(&self) -> &Vec<Box<dyn FieldType>>;

    ///
    /// Get field type.
    ///
    /// @param name
    /// @return null if field do not exist
    /// @throws FieldNotFoundException if field does not exist and document type is strict
    ///
    fn get_field(&self, path: &str) -> Result<&Box<dyn FieldType>, String>;

    ///
    /// Dictionary of document type attributes.
    /// Keys are of Chars type.
    /// Values can be anything.
    ///
    /// @return Dictionary, never null, can be empty.
    ///
    fn get_attributes(&self) -> &HashMap<String, String>;

    ///
    /// Get parent types.
    ///
    /// @return DocumentType array, never null, can be empty
    ///
    fn get_super(&self) -> &Vec<Box<dyn DocumentType>>;
}