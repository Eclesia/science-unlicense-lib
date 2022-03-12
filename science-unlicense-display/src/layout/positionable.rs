//
// Public Domain - unlicense.science
//
use crate::layout::LayoutConstraint;

///
/// A positionable is similar to a scene node but holds descriptive informations
/// 
pub struct Positionable {    
    pub min_x : f64,
    pub min_y : f64,
    pub best_x : f64,
    pub best_y : f64,
    pub max_x : f64,
    pub max_y : f64,
    pub constraint : Box<dyn LayoutConstraint>
}
