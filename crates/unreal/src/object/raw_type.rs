pub trait RawType {
    type Type;

    unsafe fn from_raw(raw: *mut Self::Type) -> Self;
    fn as_raw(&self) -> *mut Self::Type;
}
