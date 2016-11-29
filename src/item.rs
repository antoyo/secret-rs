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
use std::ffi::CStr;
use std::mem::{transmute, uninitialized};
use std::ptr::null_mut;

use ffi;
use gio_sys;
use glib::error;
use glib::translate::{ToGlibPtr, from_glib_full};
use glib_ffi::{self, GHashTableIter, g_hash_table_iter_init, g_hash_table_iter_next};
use gobject_ffi;
use libc::c_void;

use AsyncReadyCallback;
use Item;

impl Item {
    pub fn delete<F: Fn(Result<bool, error::Error>) + 'static>(&self, callback: F) {
        let trampoline: AsyncReadyCallback = unsafe { transmute(item_delete_trampoline as usize) };
        type BoxedFn = Box<Fn(Result<bool, error::Error>) + 'static>;
        let f: Box<BoxedFn> = Box::new(Box::new(callback));
        let user_data: *mut c_void = Box::into_raw(f) as *mut _;
        unsafe {
            ffi::secret_item_delete(
                self.to_glib_none().0, null_mut(), trampoline, user_data
            );
        }
    }

    pub fn get_attributes(&self) -> HashMap<String, String> {
        let mut attributes = HashMap::new();
        let hash_table = unsafe { ::ffi::secret_item_get_attributes(self.to_glib_none().0) };
        let mut key = null_mut();
        let mut value = null_mut();
        let mut iter: GHashTableIter = unsafe { uninitialized() };
        unsafe { g_hash_table_iter_init(&mut iter, hash_table) };
        while unsafe { g_hash_table_iter_next(&mut iter, &mut key, &mut value) } != 0 {
            let key = unsafe { CStr::from_ptr(key as *const _) };
            let value = unsafe { CStr::from_ptr(value as *const _) };
            attributes.insert(key.to_str().unwrap().to_string(), value.to_str().unwrap().to_string());
        }
        attributes
    }
}

unsafe extern "C" fn item_delete_trampoline(this: *mut gobject_ffi::GObject, result: *mut gio_sys::GAsyncResult, f: glib_ffi::gpointer) {
    callback_guard!();
    let mut error = null_mut();
    let result = ffi::secret_item_delete_finish(this as *mut _, result, &mut error);
    let value =
        if result != 0 {
            Ok(true)
        }
        else {
            Err(from_glib_full(error))
        };
    let f: &Box<Fn(Result<bool, error::Error>) + 'static> = &*(f as *const _);
    f(value)
}
