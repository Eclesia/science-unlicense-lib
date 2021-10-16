// Public Domain - unlicense.science

pub mod api {
    pub mod io {
        mod backtrackread; pub use backtrackread::BackTrackRead;
        mod dataread; pub use dataread::DataRead;
        mod datawrite; pub use datawrite::DataWrite;
        mod readable; pub use readable::Readable;
        mod writable; pub use writable::Writable;
    }
    pub mod path {
        mod path; pub use path::Path;
    }
    pub mod store {
        mod format; pub use format::Format;
                    pub use format::FormatRegistryEntry;
        mod formathandle; pub use formathandle::FormatHandle;
                          pub mod formats;
        mod resource; pub use resource::Resource;
        mod resourceset; pub use resourceset::ResourceSet;
                         pub use resourceset::ResourceSetHandle;
        mod store; pub use store::Store;
                   pub use store::StoreHandle;
        mod simpleresource; pub use simpleresource::SimpleResource;
    }
}

///
/// Crate initialisation
///
pub fn init() {
    //science_unlicense_common::init(); Done by Math module already
    science_unlicense_math::init();
}

#[cfg(test)]
mod tests {

}
