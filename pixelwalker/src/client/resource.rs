use std::any::{Any, TypeId};
use std::collections::HashMap;

/// A type-erased container of values owned by the client. Handlers request
/// them by type via `Res<T>` in their signature.
///
/// Values are keyed by `TypeId`, so **at most one value of each concrete
/// type** may be registered. This mirrors Rocket's managed state.
///
/// # Example
///
/// ```no_run,no_test
/// let mut resources = Resources::new();
/// resources.insert(PlayerManager::default());
/// resources.insert(42u32);
/// ```
#[derive(Default)]
pub struct Resources {
    map: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers `value`, replacing any previous value of the same type.
    pub fn insert<T: Send + Sync + 'static>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(value));
    }

    /// Removes and returns the value of type `T`, if any.
    pub fn remove<T: Send + Sync + 'static>(&mut self) -> Option<T> {
        self.map
            .remove(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast::<T>().ok())
            .map(|b| *b)
    }

    /// Borrows the value of type `T`, if any.
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    /// Mutably borrows the value of type `T`, if any.
    pub fn get_mut<T: Send + Sync + 'static>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }

    /// Returns `true` if a value of type `T` is registered.
    pub fn contains<T: Send + Sync + 'static>(&self) -> bool {
        self.map.contains_key(&TypeId::of::<T>())
    }
}

/// A shared reference to a resource of type `T`. Handlers request resources
/// by writing `Res<T>` in their signature.
pub struct Res<'a, T>(pub &'a T);

impl<'a, T> std::ops::Deref for Res<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0
    }
}

/// Extracts a value from a [`Resources`] container. Implemented for `Res<T>`
/// out of the box; user types can implement this to request multiple related
/// resources in one parameter.
pub trait FromResources<'a>: Sized {
    fn from_resources(resources: &'a Resources) -> Option<Self>;
}

impl<'a, T: Send + Sync + 'static> FromResources<'a> for Res<'a, T> {
    fn from_resources(resources: &'a Resources) -> Option<Self> {
        resources.get::<T>().map(Res)
    }
}