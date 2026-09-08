
use std::any::Any;
use std::marker::PhantomData;

use crate::ConstId;
use crate::iter::IterImpl;
use crate::mapping::Mapper;
use crate::mapping::TypeKeyMapper;
use crate::type_map::TypeMap;

#[repr(transparent)]
pub struct TypeDictionary<M = TypeKeyMapper> {
    map: TypeMap<Box<dyn Any>>,
    mapper: PhantomData<M>,
}

#[inline(always)]
pub(crate) fn cast<T: 'static>(value: Box<dyn Any>) -> T { unsafe { *value.downcast().unwrap_unchecked() } }

#[inline(always)]
pub(crate) fn cast_ref<T: 'static>(value: &Box<dyn Any>) -> &T { unsafe { value.downcast_ref().unwrap_unchecked() } }

#[inline(always)]
pub(crate) fn cast_mut<T: 'static>(value: &mut Box<dyn Any>) -> &mut T { unsafe { value.downcast_mut().unwrap_unchecked() } }

impl<M> TypeDictionary<M> {
    #[inline(always)]
    pub fn new() -> Self { Self { map: TypeMap::new(), mapper: PhantomData } }
    
    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self { Self { map: TypeMap::with_capacity(capacity), mapper: PhantomData } }
    
    #[inline(always)]
    pub fn get<T: ConstId>(&self) -> Option<&M::Type> where M: Mapper<T> { self.map.get::<T>().map(cast_ref) }
    
    #[inline(always)]
    pub fn get_mut<T: ConstId>(&mut self) -> Option<&mut M::Type> where M: Mapper<T> { self.map.get_mut::<T>().map(cast_mut) }
    
    // remove element
    #[inline(always)]
    pub fn take<T: ConstId>(&mut self) -> Option<M::Type> where M: Mapper<T> { self.map.take::<T>().map(cast) }
    
    #[inline(always)]
    pub fn has<T: ConstId>(&self) -> bool { self.map.has::<T>() }
    
    // returns true if there is no element before
    #[inline(always)]
    pub fn add<T: ConstId>(&mut self, value: M::Type) -> bool where M: Mapper<T> { self.add_with::<T>(move || value) }
    
    #[inline(always)]
    pub fn add_with<T: ConstId>(&mut self, f: impl FnOnce() -> M::Type) -> bool where M: Mapper<T> { self.map.add_with::<T>(move || Box::new(f())) }
    
    #[inline(always)]
    pub fn add_default<T: ConstId>(&mut self) -> bool where M: Mapper<T>, M::Type: Default { self.add_with::<T>(Default::default) }
    
    // get element or insert
    #[inline(always)]
    pub fn get_or<T: ConstId>(&mut self, value: M::Type) -> &mut M::Type where M: Mapper<T> { cast_mut(self.map.get_or::<T>(Box::new(value))) }
    
    #[inline(always)]
    pub fn get_or_with<T: ConstId>(&mut self, f: impl FnOnce() -> M::Type) -> &mut M::Type where M: Mapper<T> { cast_mut(self.map.get_or_with::<T>(move || Box::new(f()))) }
    
    #[inline(always)]
    pub fn get_or_default<T: ConstId>(&mut self) -> &mut M::Type where M: Mapper<T>, M::Type: Default {
        self.add_default::<T>();
        unsafe { self.get_mut::<T>().unwrap_unchecked() }
    }
    
    // add or overwrite the element and returns the final element's mutable reference
    #[inline(always)]
    pub fn set<T: ConstId>(&mut self, value: M::Type) -> &mut M::Type where M: Mapper<T> { cast_mut(self.map.set::<T>(Box::new(value))) }
    
    // set the element but returns previous element
    #[inline(always)]
    pub fn overwrite<T: ConstId>(&mut self, value: M::Type) -> Option<M::Type> where M: Mapper<T> { self.map.overwrite::<T>(Box::new(value)).map(cast) }
    
    #[inline(always)] pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter { self.into_iter() }
    #[inline(always)] pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter { self.into_iter() }
}

impl<M> IterImpl for TypeDictionary<M> {
    type Inner = TypeMap<Box<dyn Any>>;
    
    #[inline(always)] fn get_inner(self) -> Self::Inner { self.map }
    #[inline(always)] fn get_inner_ref(&self) -> &Self::Inner { &self.map }
    #[inline(always)] fn get_inner_mut(&mut self) -> &mut Self::Inner { &mut self.map }
}
