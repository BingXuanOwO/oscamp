//!

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
#[doc(no_inline)]
pub use alloc::collections::*;


mod hashmap;

pub use self::hashmap::HashMap;