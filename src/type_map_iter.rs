
use crate::iter::*;
use crate::type_map::TypeMap;

pub mod owned {
    use crate::*;
    use super::*;
    
    pub struct Entry<T> {
        pub(crate) id: TypeId,
        pub(crate) value: T,
    }
    
    impl<T> Entry<T> {
        #[inline(always)]
        pub fn as_ref(&self) -> reference::Entry<'_, T> { reference::Entry { id: self.id, value: &self.value } }
        
        #[inline(always)]
        pub fn as_mut(&mut self) -> mutable::Entry<'_, T> { mutable::Entry { id: self.id, value: &mut self.value } }
        
        #[inline(always)]
        pub fn is_key<U: ConstId>(&self) -> bool { self.as_ref().is_key::<U>() }
        
        #[inline(always)]
        pub fn match_key<U: ConstId>(self) -> Option<T> { self.is_key::<U>().then_some(self.value) }
    }
}

pub mod reference {
    use crate::*;
    
    pub struct Entry<'a, T> {
        pub(crate) id: TypeId,
        pub(crate) value: &'a T,
    }

    impl<'a, T> Entry<'a, T> {
        #[inline(always)]
        pub fn is_key<U: ConstId>(&self) -> bool { self.id == U::ID }
        
        #[inline(always)]
        pub fn match_key<U: ConstId>(&self) -> Option<&'a T> { self.is_key::<U>().then_some(self.value) }
    }
}

pub mod mutable {
    use crate::*;
    use super::*;
    
    pub struct Entry<'a, T> {
        pub(crate) id: TypeId,
        pub(crate) value: &'a mut T,
    }

    impl<'a, T> Entry<'a, T> {
        #[inline(always)]
        pub fn as_ref(&self) -> reference::Entry<'_, T> { reference::Entry { id: self.id, value: &*self.value } }
        
        #[inline(always)]
        pub fn is_key<U: ConstId>(&self) -> bool { self.as_ref().is_key::<U>() }
        
        #[inline(always)]
        pub fn match_key<U: ConstId>(&mut self) -> Option<&mut T> { self.is_key::<U>().then_some(self.value) }
    }
}

impl<T> IterImplMap for TypeMap<T> {
    type Item = owned::Entry<T>;
    
    #[inline(always)]
    fn iter_map_item((id, value): <Self::Inner as IntoIterator>::Item) -> Self::Item { owned::Entry { id, value } }
}

impl<'a, T: 'a> IterImplRef<'a> for TypeMap<T> {
    type Ref = reference::Entry<'a, T>;
    type Mut = mutable::Entry<'a, T>;
    
    #[inline(always)]
    fn iter_map_ref((&id, value): <&'a Self::Inner as IntoIterator>::Item) -> Self::Ref { reference::Entry { id, value } }
    
    #[inline(always)]
    fn iter_map_mut((&id, value): <&'a mut Self::Inner as IntoIterator>::Item) -> Self::Mut { mutable::Entry { id, value } }
}

crate::impl_iter!({T} TypeMap<T>);
