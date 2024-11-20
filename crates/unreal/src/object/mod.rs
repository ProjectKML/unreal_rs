mod class;
mod object;
mod raw_type;

pub use class::*;
pub use object::*;
pub use raw_type::*;

use crate::ffi;

pub struct Object(*mut ffi::UObject);