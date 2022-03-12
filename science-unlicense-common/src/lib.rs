// Public Domain - unlicense.science

pub mod api {
    pub mod model {
        pub mod doc {
            mod documenttype; pub use documenttype::DocumentType;
            mod fieldtype; pub use fieldtype::FieldType;
        }
        mod multiplicitytype; pub use multiplicitytype::MultiplicityType;
        mod presentable; pub use presentable::Presentable;
        mod valuetype; pub use valuetype::ValueType;
    }
    pub mod number {
        pub const TYPE_FLOAT32 : Float32 = Float32{};
        pub const TYPE_FLOAT64 : Float64 = Float64{};

        mod arithmetic; pub use arithmetic::Arithmetic;
        mod operand; pub use operand::Operand;
        mod arithmetictype; pub use arithmetictype::ArithmeticType;
        mod float32; pub use float32::Float32;
        mod float64; pub use float64::Float64;
        pub mod primitive;
    }
    pub mod reflection {
        ///
        /// Cast given Any to a concrete type.
        ///
        pub fn cast_struct<T: std::any::Any>(x : &dyn std::any::Any) -> Option<&T> {
            return x.downcast_ref::<T>();
        }

        ///
        /// Tests whether the given value is castable to some trait object. This will
        /// always return `false` if the implementation of the target trait, for the
        /// concrete type of x, has not been registered via `traitcast!`.
        ///
        pub fn implements_trait<From, To>(x: &From) -> bool
        where
            From: traitcast::TraitcastFrom + ?Sized,
            To: ?Sized + 'static,
        {
            return traitcast::implements_trait::<From, To>(x);
        }

        ///
        /// Tries to cast the given pointer to a dynamic trait object. This will always
        /// return Err if the implementation of the target trait, for the concrete type
        /// of x, has not been registered via `traitcast!`.
        ///
        pub fn cast_box<From, To>(x: Box<From>) -> Result<Box<To>, Box<dyn std::any::Any>>
        where
            From: traitcast::TraitcastFrom + ?Sized,
            To: ?Sized + 'static,
        {
            return traitcast::cast_box::<From, To>(x);
        }

        ///
        /// Tries to cast the given mutable reference to a dynamic trait object. This
        /// will always return None if the implementation of the target trait, for the
        /// concrete type of x, has not been registered via `traitcast!`.
        ///
        pub fn cast_mut<'a, From, To>(x: &'a mut From) -> Option<&'a mut To>
        where
            From: traitcast::TraitcastFrom + ?Sized,
            To: ?Sized + 'static,
        {
            return traitcast::cast_mut::<From, To>(x);
        }

        ///
        /// Tries to cast the given reference to a dynamic trait object. This will
        /// always return None if the implementation of the target trait, for the
        /// concrete type of x, has not been registered via `traitcast!`.
        ///
        pub fn cast_ref<'a, From, To>(x: &'a From) -> Option<&'a To>
        where
            From: traitcast::TraitcastFrom + ?Sized,
            To: ?Sized + 'static,
        {
            return traitcast::cast_ref::<From, To>(x);
        }

    }
    mod logger; pub use logger::Logger;
    mod polymorph; pub use polymorph::Polymorph;
    mod module; pub use module::Module;
    mod messageerror; pub use messageerror::MessageError;
    pub mod logging;
    pub mod registry;
    pub mod asserts;
    
}

pub mod event {
    mod event; pub use event::Event;
    mod eventlistener; pub use eventlistener::EventListener;
    mod eventmessage; pub use eventmessage::EventMessage;
    mod eventsource; pub use eventsource::EventSource;
}

pub mod predicate {
    mod expression; pub use expression::Expression;
    mod predicate; pub use predicate::Predicate;
}

///
/// Crate initialisation
///
pub fn init() {
    //does nothing for now
}

#[cfg(test)]
mod tests {

}
