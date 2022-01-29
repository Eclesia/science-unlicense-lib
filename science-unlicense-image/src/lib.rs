// Public Domain - unlicense.science

pub mod api {
    pub mod colorspace {
    }
    pub mod model {
        mod imagemodel; pub use imagemodel::ImageModel;
        mod directmodel; pub use directmodel::DirectModel;
    }
    
    pub const MODEL_RAW : &str = "RAW";
    pub const MODEL_COLOR : &str = "COLOR";
    mod image; pub use image::Image;
}

pub mod imp {
    mod defaultimage; pub use defaultimage::DefaultImage;
}

///
/// Crate initialisation
///
pub fn init() {
    science_unlicense_geometry::init();
}

///
/// Tests
///
#[cfg(test)]
mod tests {
    
}
