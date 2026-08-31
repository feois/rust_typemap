
pub trait IterImpl {
    type Inner;
    
    fn get_inner(self) -> Self::Inner;
    fn get_inner_ref(&self) -> &Self::Inner;
    fn get_inner_mut(&mut self) -> &mut Self::Inner;
}

pub trait IterImplMap: IterImpl where Self::Inner: IntoIterator {
    type Item;
    
    fn iter_map_item(value: <Self::Inner as IntoIterator>::Item) -> Self::Item;
}

pub trait IterImplRef<'a>: IterImpl where &'a Self::Inner: IntoIterator + 'a, &'a mut Self::Inner: IntoIterator + 'a {
    type Ref;
    type Mut;
    
    fn iter_map_ref(value: <&'a Self::Inner as IntoIterator>::Item) -> Self::Ref;
    fn iter_map_mut(value: <&'a mut Self::Inner as IntoIterator>::Item) -> Self::Mut;
}

#[macro_export]
macro_rules! impl_iter {
    ($({$($g:tt)*})? $t:ty) => {
        impl $(<$($g)*>)? ::std::iter::IntoIterator for $t {
            type IntoIter = ::std::iter::Map<<<$t as $crate::iter::IterImpl>::Inner as ::std::iter::IntoIterator>::IntoIter, fn(<<$t as $crate::iter::IterImpl>::Inner as ::std::iter::IntoIterator>::Item) -> Self::Item>;
            type Item = <$t as $crate::iter::IterImplMap>::Item;
            
            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter { $crate::iter::IterImpl::get_inner(self).into_iter().map(<$t as $crate::iter::IterImplMap>::iter_map_item) }
        }
        
        impl<'a, $($($g)*)?> ::std::iter::IntoIterator for &'a $t {
            type IntoIter = ::std::iter::Map<<&'a <$t as $crate::iter::IterImpl>::Inner as ::std::iter::IntoIterator>::IntoIter, fn(<&'a <$t as $crate::iter::IterImpl>::Inner as ::std::iter::IntoIterator>::Item) -> Self::Item>;
            type Item = <$t as $crate::iter::IterImplRef<'a>>::Ref;
            
            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter { $crate::iter::IterImpl::get_inner_ref(self).into_iter().map(<$t as $crate::iter::IterImplRef<'a>>::iter_map_ref) }
        }

        impl<'a, $($($g)*)?> ::std::iter::IntoIterator for &'a mut $t {
            type IntoIter = ::std::iter::Map<<&'a mut <$t as $crate::iter::IterImpl>::Inner as ::std::iter::IntoIterator>::IntoIter, fn(<&'a mut <$t as $crate::iter::IterImpl>::Inner as ::std::iter::IntoIterator>::Item) -> Self::Item>;
            type Item = <$t as $crate::iter::IterImplRef<'a>>::Mut;
            
            #[inline(always)]
            fn into_iter(self) -> Self::IntoIter { $crate::iter::IterImpl::get_inner_mut(self).into_iter().map(<$t as $crate::iter::IterImplRef<'a>>::iter_map_mut) }
        }
    };
}
