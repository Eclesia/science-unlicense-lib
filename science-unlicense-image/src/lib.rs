// Public Domain - unlicense.science

pub mod api {
    mod colorspace {
    }
    mod model {
        mod imagemodel; pub use imagemodel::ImageModel;
    }
    
    mod image; pub use image::Image;
}

///
/// Crate initialisation
///
pub fn init() {
    science_unlicense_geometry::init();
}

#[cfg(test)]
mod tests {
    
}
