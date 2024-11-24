
use bevy::prelude::*;

#[derive(Component, Eq, Debug)]
pub enum Toggled<T> {
    On,
    Off,
    Marker(std::marker::PhantomData<T>),
}

impl<T> Toggled<T> {
    pub fn into<K>(self) -> Toggled<K> {
        match self {
            Toggled::On => Toggled::On,
            Toggled::Off => Toggled::Off,
            Toggled::Marker(_) => Toggled::Marker(std::marker::PhantomData)
        }
    }
}

impl<T> Copy for Toggled<T> {}
impl<T> Clone for Toggled<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> PartialEq for Toggled<T> {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (Self::On, Self::On) => true,
            (Self::Off, Self::Off) => true,
            (Self::Marker(_), Self::Marker(_)) => true,
            _ => false,
        }
    }
}
