
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MessageError {
    message: String,
}

impl MessageError {

    pub fn new(t: String) -> Self {
        return MessageError { message: t };
    }
}