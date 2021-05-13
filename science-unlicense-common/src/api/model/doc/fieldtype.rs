use crate::api::model::ValueType;
use crate::api::model::MultiplicityType;
use std::collections::HashMap;
use crate::api::model::doc::DocumentType;

//
// Public Domain - unlicense.science
//
pub trait FieldType : ValueType + MultiplicityType {

    ///
    /// If the value type is of type Document the reference type
    /// indicate the type of document referenced.
    ///
    /// @return DocumentType
    ///
    fn get_reference_type(&self) -> dyn DocumentType;

    ///
    /// Dictionary of field attributes.
    /// Keys are of Chars type.
    ///
    /// @return Dictionary, never null, can be empty.
    ///
    fn get_attributes(&self) -> HashMap<String,String>;

    // ///
    // /// Parameter constraints.
    // /// @return Sequence, never null.
    // ///
    //fn getConstraints() - Vec<?>;

}