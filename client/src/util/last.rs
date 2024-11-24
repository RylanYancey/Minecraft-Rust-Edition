use bevy::prelude::*;

/// Used for storing what the last value was for a component.
#[derive(Component, PartialEq, Eq, Deref, DerefMut, Debug)]
pub struct Last<T>(pub T);

impl<T: PartialEq> Last<T> {
    pub fn set(&mut self, value: T) {
        self.0 = value;
    }

    pub fn is(&self, value: T) -> bool {
        self.0 == value
    }
}
