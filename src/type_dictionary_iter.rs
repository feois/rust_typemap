
use std::any::Any;
use std::marker::PhantomData;

use crate::ConstId;
use crate::mapping::Mapper;
use crate::type_dictionary::*;
use crate::iter::*;
use crate::type_map_iter;

pub mod owned {
    use super::*;
    
    pub struct Entry<M> {
        pub(crate) entry: type_map_iter::owned::Entry<Box<dyn Any>>,
        pub(crate) mapper: PhantomData<M>,
    }
    
    impl<M> Entry<M> {
        #[inline(always)]
        pub fn as_ref(&self) -> reference::Entry<'_, M> { reference::Entry { entry: self.entry.as_ref(), mapper: PhantomData } }
        
        #[inline(always)]
        pub fn as_mut(&mut self) -> mutable::Entry<'_, M> { mutable::Entry { entry: self.entry.as_mut(), mapper: PhantomData } }
        
        #[inline(always)]
        pub fn is_key<T: ConstId>(&self) -> bool { self.as_ref().is_key::<T>() }
        
        #[inline(always)]
        pub fn match_key<T: ConstId>(self) -> Option<M::Type> where M: Mapper<T> { self.entry.match_key::<T>().map(cast) }
    }
}

pub mod reference {
    use super::*;
    
    #[repr(transparent)]
    pub struct Entry<'a, M> {
        pub(crate) entry: type_map_iter::reference::Entry<'a, Box<dyn Any>>,
        pub(crate) mapper: PhantomData<M>,
    }

    impl<'a, M> Entry<'a, M> {
        #[inline(always)]
        pub fn is_key<T: ConstId>(&self) -> bool { self.entry.is_key::<T>() }
        
        #[inline(always)]
        pub fn match_key<T: ConstId>(&self) -> Option<&'a M::Type> where M: Mapper<T> { self.entry.match_key::<T>().map(cast_ref) }
    }
}

pub mod mutable {
    use super::*;
    
    pub struct Entry<'a, M> {
        pub(crate) entry: type_map_iter::mutable::Entry<'a, Box<dyn Any>>,
        pub(crate) mapper: PhantomData<M>,
    }

    impl<'a, M> Entry<'a, M> {
        #[inline(always)]
        pub fn as_ref(&self) -> reference::Entry<'_, M> { reference::Entry { entry: self.entry.as_ref(), mapper: PhantomData } }
        
        #[inline(always)]
        pub fn is_key<T: ConstId>(&self) -> bool { self.as_ref().is_key::<T>() }
        
        #[inline(always)]
        pub fn match_key<T: ConstId>(&mut self) -> Option<&mut M::Type> where M: Mapper<T> { self.entry.match_key::<T>().map(cast_mut) }
    }
}

impl<M> IterImplMap for TypeDictionary<M> {
    type Item = owned::Entry<M>;
    
    #[inline(always)]
    fn iter_map_item(entry: <Self::Inner as IntoIterator>::Item) -> Self::Item { owned::Entry { entry, mapper: PhantomData } }
}

impl<'a, M> IterImplRef<'a> for TypeDictionary<M> {
    type Ref = reference::Entry<'a, M>;
    type Mut = mutable::Entry<'a, M>;
    
    #[inline(always)]
    fn iter_map_ref(entry: <&'a Self::Inner as IntoIterator>::Item) -> Self::Ref { reference::Entry { entry, mapper: PhantomData } }
    
    #[inline(always)]
    fn iter_map_mut(entry: <&'a mut Self::Inner as IntoIterator>::Item) -> Self::Mut { mutable::Entry { entry, mapper: PhantomData } }
}

crate::impl_iter!({M} TypeDictionary<M>);
