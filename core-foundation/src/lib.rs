// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![allow(non_snake_case)]

extern crate core_foundation_sys;
extern crate libc;

#[macro_export]
macro_rules! impl_TCFType {
    ($ty:ident, $raw:ident, $ty_id:ident) => {
        impl $crate::base::TCFType<$raw> for $ty {
            #[inline]
            fn as_concrete_TypeRef(&self) -> $raw {
                self.0
            }

            #[inline]
            unsafe fn wrap_under_get_rule(reference: $raw) -> $ty {
                let reference = mem::transmute(::core_foundation_sys::base::CFRetain(mem::transmute(reference)));
                $crate::base::TCFType::wrap_under_create_rule(reference)
            }

            #[inline]
            fn as_CFTypeRef(&self) -> ::core_foundation_sys::base::CFTypeRef {
                unsafe {
                    mem::transmute(self.as_concrete_TypeRef())
                }
            }

            #[inline]
            unsafe fn wrap_under_create_rule(obj: $raw) -> $ty {
                $ty(obj)
            }

            #[inline]
            fn type_id() -> ::core_foundation_sys::base::CFTypeID {
                unsafe {
                    $ty_id()
                }
            }
        }
    }
}

pub mod array;
pub mod base;
pub mod boolean;
pub mod data;
pub use core_foundation_sys::date; // back compat
pub mod dictionary;
pub mod number;
pub mod set;
pub mod string;
pub mod url;
pub mod bundle;
pub mod runloop;

#[cfg(test)]
pub mod test {
    #[test]
    fn test_stuff() {
        use base::TCFType;
        use boolean::CFBoolean;
        use number::number;
        use dictionary::CFDictionary;
        use string::CFString;

        /*let n = CFNumber::new_number(42 as i32);
        io::println(format!("%d", (&n).retain_count() as int));
        (&n).show();*/

        let bar = CFString::from_static_string("Bar");
        let baz = CFString::from_static_string("Baz");
        let boo = CFString::from_static_string("Boo");
        let foo = CFString::from_static_string("Foo");
        let tru = CFBoolean::true_value();
        let n42 = number(42);

        let _d = CFDictionary::from_CFType_pairs(&[
            (bar.as_CFType(), boo.as_CFType()),
            (baz.as_CFType(), tru.as_CFType()),
            (foo.as_CFType(), n42.as_CFType()),
        ]);
    }
}
