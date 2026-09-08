
use crate::iter::*;
use crate::type_set::TypeSet;

impl IterImplMap for TypeSet {
    type Item = <Self::Inner as IterImplMap>::Item;
    
    #[inline(always)] fn iter_map_item(value: <Self::Inner as IntoIterator>::Item) -> Self::Item { value }
}

impl<'a> IterImplRef<'a> for TypeSet {
    type Ref = <Self::Inner as IterImplRef<'a>>::Ref;
    type Mut = <Self::Inner as IterImplRef<'a>>::Mut;
    
    #[inline(always)] fn iter_map_ref(value: <&'a Self::Inner as IntoIterator>::Item) -> Self::Ref { value }
    #[inline(always)] fn iter_map_mut(value: <&'a mut Self::Inner as IntoIterator>::Item) -> Self::Mut { value }
}

crate::impl_iter!(TypeSet);
