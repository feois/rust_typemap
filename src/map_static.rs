use std::{any::TypeId, collections::HashMap};

use crate::{ConstId, iter::IterImpl};

#[repr(transparent)]
pub struct TypeMap<T> { pub(crate) map: HashMap<TypeId, T> }

impl<T> TypeMap<T> {
    #[inline(always)]
    pub fn new() -> Self { Self { map: HashMap::new() } }
    
    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self { Self { map: HashMap::with_capacity(capacity) } }
    
    #[inline(always)]
    pub fn get<U: ConstId>(&self) -> Option<&T> { self.map.get(&U::ID) }
    
    #[inline(always)]
    pub fn get_mut<U: ConstId>(&mut self) -> Option<&mut T> { self.map.get_mut(&U::ID) }
    
    // remove element
    #[inline(always)]
    pub fn take<U: ConstId>(&mut self) -> Option<T> { self.map.remove(&U::ID) }
    
    #[inline(always)]
    pub fn has<U: ConstId>(&self) -> bool { self.map.contains_key(&U::ID) }
    
    // returns true if there is no element before
    #[inline(always)]
    pub fn add<U: ConstId>(&mut self, value: T) -> bool { self.add_with::<U>(move || value) }
    
    #[inline(always)]
    pub fn add_with<U: ConstId>(&mut self, f: impl FnOnce() -> T) -> bool {
        if self.has::<U>() { false }
        else {
            self.map.insert(U::ID, f());
            true
        }
    }
    
    #[inline(always)]
    pub fn add_default<U: ConstId>(&mut self) -> bool where T: Default { self.add_with::<U>(Default::default) }
    
    // get element or insert
    #[inline(always)]
    pub fn get_or<U: ConstId>(&mut self, value: T) -> &mut T {
        self.add::<U>(value);
        unsafe { self.get_mut::<U>().unwrap_unchecked() }
    }
    
    #[inline(always)]
    pub fn get_or_with<U: ConstId>(&mut self, f: impl FnOnce() -> T) -> &mut T {
        self.add_with::<U>(f);
        unsafe { self.get_mut::<U>().unwrap_unchecked() }
    }
    
    #[inline(always)]
    pub fn get_or_default<U: ConstId>(&mut self) -> &mut T where T: Default {
        self.add_default::<U>();
        unsafe { self.get_mut::<U>().unwrap_unchecked() }
    }
    
    // add or overwrite the element and returns the final element's mutable reference
    #[inline(always)]
    pub fn set<U: ConstId>(&mut self, value: T) -> &mut T {
        self.map.insert(U::ID, value);
        unsafe { self.get_mut::<U>().unwrap_unchecked() }
    }
    
    // set the element but returns previous element
    #[inline(always)]
    pub fn overwrite<U: ConstId>(&mut self, value: T) -> Option<T> { self.map.insert(U::ID, value) }
    
    #[inline(always)] pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter { self.into_iter() }
    #[inline(always)] pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter { self.into_iter() }
}

impl<T> IterImpl for TypeMap<T> {
    type Inner = HashMap<TypeId, T>;
    
    #[inline(always)] fn get_inner(self) -> Self::Inner { self.map }
    #[inline(always)] fn get_inner_ref(&self) -> &Self::Inner { &self.map }
    #[inline(always)] fn get_inner_mut(&mut self) -> &mut Self::Inner { &mut self.map }
}

#[repr(transparent)]
pub struct TypeSet { map: TypeMap<()> }

impl TypeSet {
    #[inline(always)]
    pub fn new() -> Self { Self { map: TypeMap::new() } }
    
    #[inline(always)]
    pub fn with_capacity(capacity: usize) -> Self { Self { map: TypeMap::with_capacity(capacity) } }
    
    #[inline(always)]
    pub fn has<T: ConstId>(&self) -> bool { self.map.has::<T>() }
    
    // returns true if there is no element before
    #[inline(always)]
    pub fn add<T: ConstId>(&mut self) -> bool { self.map.add::<T>(()) }
    
    pub fn remove<T: ConstId>(&mut self) -> bool { self.map.take::<T>().is_some() }
    
    #[inline(always)] pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter { self.into_iter() }
    #[inline(always)] pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter { self.into_iter() }
}

impl IterImpl for TypeSet {
    type Inner = TypeMap<()>;
    
    #[inline(always)] fn get_inner(self) -> Self::Inner { self.map }
    #[inline(always)] fn get_inner_ref(&self) -> &Self::Inner { &self.map }
    #[inline(always)] fn get_inner_mut(&mut self) -> &mut Self::Inner { &mut self.map }
}
