//
// Public Domain - unlicense.science
//

///
///
///
pub trait Presentable {

    ///
    /// Get identifier.
    ///
    /// Should be unique.
    ///
    /// @return &str, never null
    ///
    fn get_id(&self) -> &str;

    ///
    /// Get human readable short title.
    ///
    /// @return &str, never null
    ///
    fn get_title(&self) -> &str;

    ///
    /// Get task description.
    ///
    /// @return &str, never null
    ///
    fn get_description(&self) -> &str;

}
