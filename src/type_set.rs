
use crate::ConstId;
use crate::type_map::TypeMap;
use crate::iter::*;

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
