//
// Public Domain - unlicense.science
//

use crate::api::model::Presentable;

pub trait MultiplicityType : Presentable {

    fn get_min_occurences(&self) -> u32;

    fn get_max_occurences(&self) -> u32;

}
