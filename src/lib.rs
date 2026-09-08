
use std::any::TypeId;

pub mod mapping;
pub mod iter;
pub mod type_set;
pub mod type_set_iter;
pub mod type_map;
pub mod type_map_iter;
pub mod type_dictionary;
pub mod type_dictionary_iter;
pub mod any_set;

pub mod prelude {
    pub use crate::{
        type_map::TypeMap,
        type_set::TypeSet,
        type_dictionary::TypeDictionary,
        any_set::AnySet,
    };
}

#[cfg(test)] mod tests;

pub trait ConstId: 'static { const ID: TypeId = TypeId::of::<Self>(); }

impl<T: 'static> ConstId for T {}
