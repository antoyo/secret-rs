/*
 * Copyright (c) 2016 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use std::collections::HashMap;
use std::ffi::CString;

use glib::translate::{ToGlib, from_glib_full};
use glib_ffi::{GHashTable, g_hash_table_insert, g_hash_table_new, g_int_equal, g_str_hash};
use libc::c_void;

use ffi;
use Schema;
use SchemaAttributeType;

impl Schema {
    pub fn new(name: &str, attribute_types: HashMap<String, SchemaAttributeType>) -> Self {
        let (_strings, hash_table) = to_glib_hash_map(&attribute_types);
        let name = CString::new(name).unwrap();
        let schema = unsafe { ffi::secret_schema_newv(name.as_ptr(), ffi::SECRET_SCHEMA_NONE, hash_table) };
        unsafe { from_glib_full(schema) }
    }
}

fn to_glib_hash_map(hash_map: &HashMap<String, SchemaAttributeType>) -> (Vec<CString>, *mut GHashTable) {
    let result = unsafe { g_hash_table_new(Some(g_str_hash), Some(g_int_equal)) };
    let mut strings = vec![];
    for (key, value) in hash_map {
        let key = CString::new(key.clone()).unwrap();
        let value = value.to_glib() as i64;
        unsafe { g_hash_table_insert(result, key.as_ptr() as *mut _, value as *mut c_void) };
        strings.push(key);
    }
    (strings, result)
}
