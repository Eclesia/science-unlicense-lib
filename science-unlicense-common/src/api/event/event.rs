//
// Public Domain - unlicense.science
//
use crate::api::event::EventMessage;
use crate::api::event::EventSource;

///
/// An event is a container for informations.
/// Whenever an object needs to send notifications to others it should implement
/// the EventSource class, and then send Event objects.
///
/// Events can be used for example as :
/// - mouse move event
/// - button click
/// - data update notification
///
/// Event objects should not be subclasses, event object should only hold
/// informations of the source, message and trigger. Since new events can
/// be created with different sources, the only property which will be preserved
/// is the EventMessage. It is recommended to subclass EventMessage and not Event.
///
pub struct Event {
    source: Box<dyn EventSource>,
    message: Box<dyn EventMessage>,
    trigger: Option<Box<Event>>
}
 
impl Event {

    pub fn new(source: Box<dyn EventSource>, message: Box<dyn EventMessage>) -> Self {
        return Event {
            source : source,
            message : message,
            trigger : None
        }
    }

    pub fn new_with_trigger(source: Box<dyn EventSource>, message: Box<dyn EventMessage>, trigger: Box<Event>) -> Self {
        return Event {
            source : source,
            message : message,
            trigger : Some(trigger)
        }
    }

    ///
    /// @return EventSource who raised the event, never null.
    ///
    pub fn get_source(&self) -> &Box<dyn EventSource> {
        return &self.source;
    }

    ///
    /// @return EventMessage the event informations, never null.
    ///
    pub fn get_message(&self) -> &Box<dyn EventMessage> {
        return &self.message;
    }

    ///
    /// An event might have been trigger by another.
    /// This method returns the original trigger event.
    ///
    /// @return Event, can be null.
    ///
    pub fn get_trigger(&self) -> &Option<Box<Event>>{
        return &self.trigger;
    }

}