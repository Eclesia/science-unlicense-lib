//
// Public Domain - unlicense.science
//

use std::io::Error;
use crate::api::io::Readable;
use crate::api::store::Store;

///
/// A format define a group of bytes organize in a defined structure.
/// Formats are often associated to mime types and file extensions.
///
/// Formats produce stores which in turn give access to resources of various types :
/// - ImageResource : PNG,BMP,TGA,...
/// - Media : WAV,MKV,MP4,...
/// - SceneResource : Collada,OBJ,3DS,...
/// - Archive : ZIP,TAR,...
///
/// Formats are registered using https://github.com/dtolnay/inventory
///
///
pub trait Format {

    ///
    /// Identifier inside the library.
    /// Should be unique.
    ///
    /// @return Chars, never null
    ///
    fn get_identifier(&self) -> &str;

    ///
    /// Short name, most common abbreviation used for this format.
    ///
    /// @return Chars, never null
    ///
    fn get_short_name(&self) -> &str;

    ///
    /// Long name, full name of this format.
    ///
    /// @return Chars, never null
    ///
    fn get_long_name(&self) -> &str;

    ///
    /// Set of known mime-types.
    ///
    /// @return Set, never null, can be empty.
    ///
    fn get_mime_types(&self) -> &Vec<&str>;

    ///
    /// List of used file extensions.
    ///
    /// @return Chars array, never null, can be empty.
    ///
    fn get_extensions(&self) -> &Vec<&str>;

    ///
    /// Get a list of possible file signatures used by this format.
    /// A signature is a small number of bytes located at the begining of the file
    /// which allows to quickly identify the file format.
    /// Some format might have more then one signature.
    ///
    /// Size 0 if there are no specific signature for this format.
    ///
    /// @return collection of possible signatures, never null
    ///
    fn get_signature(&self) -> &Vec<&Vec<u8>>;

    ///
    /// Some formats extend existing formats, this is the case for formats based
    /// on all purpose encodings like geojson with json, or all xml schemas.
    /// PNG image format also has a subtype : APNG
    ///
    /// @return true if format is a subtype
    ///
    fn is_subset_format(&self) -> bool;

    ///
    /// Get format possible stored types.
    /// Formats which acts as containers like archives and Asset bundles can
    /// return an empty array to indicate anything may be encountered.
    ///
    /// @return collection of Resource classes expected to be found in this format.
    ///
    fn get_resource_types(&self) -> &Vec<&str>;

    ///
    /// Test to check if given input can be decoded.
    /// This method is expected to be fast, only a minimal decoding should be performed.
    /// Most format have a signature at the beginning of a file, it is
    /// enough to test only this signature.
    ///
    /// Note : subtype formats should implement a deeper test.
    ///
    /// @param input object or Document describing parameters.
    /// @return true if input can be decoded by this format.
    /// @throws science.unlicense.encoding.api.io.IOException
    ///
    fn can_decode(&self, input: &mut dyn Readable) -> Result<bool, Error>;

    ///
    /// Finding the expected end of the current format can have multiple use.
    ///
    /// If the file size is available near the beginning, often in the header then
    /// it should be returned whatever value has the fullscan flag.
    ///
    /// If the size can not be determinate then method should return -1 unless
    /// the fullscan flag is set to true it which case it should read the stream until it
    /// finds the end.
    ///
    /// Formats are allowed to return -1 in any case.
    ///
    fn search_end(&self, input: &mut dyn Readable, fullscan: bool) -> Result<u64, Error>;

    ///
    /// Get a description of parameters used to open a new store.
    ///
    /// @return DocumentType, never null
    ///
    fn get_parameters(&self);

    ///
    ///
    /// @param source input object or Document describing parameters.
    /// @return
    ///
    fn open(&self, source: dyn Readable) -> Result<Box<dyn Store>, Error>;

}
