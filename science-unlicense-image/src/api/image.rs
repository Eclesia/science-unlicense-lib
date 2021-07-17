//
// Public Domain - unlicense.science
//
use crate::api::model::ImageModel;
use science_unlicense_math::api::Tuple;
use science_unlicense_geometry::api::Extenti64;
use std::iter::Map;
use science_unlicense_geometry::api::tuple::TupleArray;

///
/// An image is a regular multidimensional array of values.
/// Usual images are in 2 dimensions and have 3 or 4 bands mapping RGBA colors.
///
pub trait Image {

    ///
    /// Get image extent.
    /// An image is a N dimension grid.
    ///
    /// @return number and size of the image
    ///
    fn get_extent(&self) -> Extenti64;

    ///
    /// Images are N dimension raw byte buffers.
    /// ImageModel produce views of the image for a given purpose.
    ///
    fn get_models(&self) -> Map<String, Box<&dyn ImageModel>>;

    ///
    /// Get mandatory raw model.
    ///
    /// @return ImageModel, never null.
    ///
    fn get_raw_model(&self) -> Box<&dyn ImageModel>;

    ///
    /// Get optional color model.
    ///
    /// @return ImageModel, can be null.
    ///
    fn get_color_model(&self) -> Option<Box<&dyn ImageModel>>;

    ///
    /// Get tuple buffer for an image model.
    ///
    /// @return TupleBuffer, can be null if model is null
    ///
    fn get_tuple_buffer(&self, model : Box<&dyn ImageModel>) -> Box<&dyn TupleArray>;

    fn get_tuple(&self, coordinate: Box<&dyn Tuple>, model: Box<&dyn ImageModel>) -> Box<dyn Tuple>;

    Color getColor(Tuple coordinate);

    /**
     * Raw image data buffer.
     *
     * @return Buffer
     */
    Buffer getDataBuffer();
}
