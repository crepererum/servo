/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use eutil::fptr_is_null;
use libc::{c_int};
use std::collections::TreeMap;
use std::mem;
use std::slice;
use std::str;
use std::string::String;
use string::{cef_string_userfree_utf8_alloc,cef_string_userfree_utf8_free,cef_string_utf8_set};
use types::{cef_string_map_t,cef_string_t};

//cef_string_map

#[no_mangle]
pub extern "C" fn cef_string_map_alloc() -> *mut cef_string_map_t {
    unsafe {
         let sm: Box<TreeMap<String, *mut cef_string_t>> = box TreeMap::new();
         mem::transmute(sm)
    }
}
