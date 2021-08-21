//
// Public Domain - unlicense.science
//
pub struct Extentf64 {
    values : Vec<f64>
}

impl Extentf64 {

    pub fn extents(&self) -> &Vec<f64> {
        return &self.values;
    }

}

