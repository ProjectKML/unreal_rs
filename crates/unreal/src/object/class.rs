use crate::{ffi, object::RawType};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Class(pub(crate) *mut ffi::UClass);

unsafe impl Send for Class {}
unsafe impl Sync for Class {}

impl RawType for Class {
    type Type = ffi::UClass;

    #[inline]
    unsafe fn from_raw(raw: *mut Self::Type) -> Self {
        Self(raw)
    }

    #[inline]
    fn as_raw(&self) -> *mut Self::Type {
        self.0
    }
}

pub trait HasClass: RawType {
    fn get_class() -> Class;
}