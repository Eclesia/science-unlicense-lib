//
// Public Domain - unlicense.science
//

use std::any::Any;
use traitcast::TraitcastFrom;

pub trait Polymorph : TraitcastFrom {

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

}