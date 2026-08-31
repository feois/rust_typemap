
use std::any::TypeId;

pub mod mapping;
pub mod map_static;
pub mod map_dynamic;
pub mod iter;
pub mod iter_static;
pub mod iter_dynamic;

pub mod prelude {
    pub use crate::{
        map_static::{TypeMap, TypeSet},
        map_dynamic::{DynamicTypeMap, DynamicTypeSet},
    };
}

#[cfg(test)] mod tests;

pub trait ConstId: 'static { const ID: TypeId = TypeId::of::<Self>(); }

impl<T: 'static> ConstId for T {}
