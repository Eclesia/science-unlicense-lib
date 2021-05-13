//
// Public Domain - unlicense.science
//
use crate::api::model::Presentable;
use std::any::Any;
use std::any::TypeId;

///
/// There are 3 variantes of self defined structures.
/// Nodes, Parameters and Documents.
///
/// ValueType is a common abstraction
///
pub trait ValueType : Presentable {

    fn get_value_class(&self) -> TypeId;

    fn get_default_value(&self) -> Box<dyn Any>;

}
