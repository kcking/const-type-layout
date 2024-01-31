use crate::{
    typeset::{tset, ComputeTypeSet, ExpandTypeSet},
    Field, MaybeUninhabited, TypeLayout, TypeLayoutInfo, TypeStructure,
};

unsafe impl<T: TypeLayout + core::ops::Deref> TypeLayout for core::pin::Pin<T> {
    type Inhabited = T::Inhabited;

    const TYPE_LAYOUT: TypeLayoutInfo<'static> = TypeLayoutInfo {
        ty: crate::TypeRef::of::<Self>(),
        size: ::core::mem::size_of::<Self>(),
        alignment: ::core::mem::align_of::<Self>(),
        structure: TypeStructure::Struct {
            repr: "transparent",
            fields: &[Field {
                name: "pointer",
                offset: MaybeUninhabited::new::<T>(0),
                ty: crate::TypeRef::of::<T>(),
            }],
        },
    };
}

unsafe impl<T: ComputeTypeSet + core::ops::Deref> ComputeTypeSet for core::pin::Pin<T> {
    type Output<R: ExpandTypeSet> = tset![T, .. @ R];
}
