// Public Domain - unlicense.science

pub mod layout {
    mod layout; pub use layout::Layout;
    mod layoutconstraint; pub use layoutconstraint::LayoutConstraint;
    mod positionable; pub use positionable::Positionable;
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
