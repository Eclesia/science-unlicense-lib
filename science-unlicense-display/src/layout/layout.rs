//
// Public Domain - unlicense.science
//
use crate::layout::Positionable;

///
/// Layouts are object in charge of placing Positionables in a defined space.
/// They can be used in widgets for user interfaces, to calculate graph node
/// positions or any system which require dynamic positioning based on constraints.
///
pub trait Layout {
    
    ///
    /// Update positionables transforms and size.
    /// The transforms will be relative to coordinate (0,0).
    ///
    fn update(positionables : Vec<Positionable>);

}

/* public interface Layout extends EventSource {

    /**
     * Property indicating when the layout view rectangle change.
     */
    public static final Chars PROPERTY_VIEW = Chars.constant("View");
    /**
     * Property indicating when the layout element placement have changed.
     */
    public static final Chars PROPERTY_DIRTY = Chars.constant("Dirty");

    /**
     * Get this layout constraint class.
     * Every layout may require different types of informations.
     *
     * @return expected positionable layout constraint class, can be null.
     */
    Class getLayoutConstraintClass();

    /**
     * Set layout working bbox.
     * The layout will try to place elements within this bbox.
     *
     * @param extent
     */
    void setView(BBox extent);

    /**
     * Working bbox of this layout.
     *
     * @return BBox never null
     */
    BBox getView();

    /**
     * Set collection of Positionable managed by the layout
     *
     * @param positionables
     */
    void setPositionables(Sequence positionables);

    /**
     * Get positionable collection.
     * 
     * @return
     */
    Sequence getPositionables();

    /**
     * Calculate layout min, best and max extents based on given constraint.
     * - Under minimum size the layout won't have a proper rendering.
     * - Best extent provide the most appropriate rendering.
     * - Over maximum size the layout will look over-stretched or have wasted space.
     *
     * The constraint can be null or have a fixed value on one or both axis.
     * The constraint may be mandatory for some layouts, like the BorderLayout.
     *
     * @param buffer
     * @param constraint can be null
     * @return
     */
    Extents getExtents(Extents buffer, Extent constraint);

    /**
     * Get an iterator of Pair (Positionable,AbsoluteConstraint) for elements which intersect the given extent
     * @param searchArea
     * @return
     */
    Iterator getPositionables(BBox searchArea);

} */