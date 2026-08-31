
pub trait Mapper<T> { type Type: 'static; }

pub trait TypeKey { type Type: 'static; }

#[repr(transparent)]
pub struct TypeKeyMapper;

impl<T: TypeKey> Mapper<T> for TypeKeyMapper { type Type = T::Type; }

#[repr(transparent)]
pub struct ReflexiveMapper;

impl<T: 'static> Mapper<T> for ReflexiveMapper { type Type = T; }
