//
// Public Domain - unlicense.science
//

use crate::event::Event;

///
/// Object listening to events must implement this interface.
/// Events will be catch in the receiveEvent method.
///
/// @author Johann Sorel
///
pub trait EventListener {

    ///
    /// Called by the EventSource when a new event is fired.
    ///
    /// @param event event object, never null
    ///
    fn receive_event(&self, event: Event);
}