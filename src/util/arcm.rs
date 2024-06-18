use std::{ops::{Deref, DerefMut}, sync::{Arc, Mutex}};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Arcm<T>(Arc<Mutex<T>>);
impl<T> Deref for Arcm<T> {
    type Target = Arc<Mutex<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Arcm<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
       &mut self.0 
    }
}
impl<T> Arcm<T> {
    pub fn new(val:T) -> Arcm<T>{
        Arcm(Arc::new(Mutex::new(val)))
    }
}

impl<'de ,T: Deserialize<'de> + Default> Deserialize<'de> for Arcm<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(Arcm::new(T::deserialize(deserializer).unwrap_or_default()))

    }
}
impl<'de ,T: Serialize> Serialize for Arcm<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        self.lock().unwrap().serialize(serializer)
    }
}
