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
use std::mem::transmute;
use std::ptr::null_mut;

use gio_sys;
use glib::error;
use glib::translate::{FromGlib, ToGlib, ToGlibPtr, from_glib_full, from_glib_none};
use glib_ffi;
use gobject_ffi;
use libc::c_void;

use AsyncReadyCallback;
use ffi;
use Collection;
use Item;
use ITEM_CREATE_REPLACE;
use Schema;
use SEARCH_ALL;
use SEARCH_LOAD_SECRETS;
use SEARCH_UNLOCK;
use to_glib_string_hash_map;
use COLLECTION_CREATE_NONE;

impl Collection {
    pub fn create<F: Fn(Result<Collection, error::Error>) + 'static>(label: &str, callback: F) {
        let label = CString::new(label).unwrap();
        let trampoline: AsyncReadyCallback = unsafe { transmute(collection_create_trampoline as usize) };
        type BoxedFn = Box<Fn(Result<Collection, error::Error>) + 'static>;
        let f: Box<BoxedFn> = Box::new(Box::new(callback));
        let user_data: *mut c_void = Box::into_raw(f) as *mut _;
        unsafe { ffi::secret_collection_create(null_mut(), label.as_ptr(), null_mut(), COLLECTION_CREATE_NONE.to_glib(), null_mut(), trampoline, user_data) };
    }

    pub fn delete<F: Fn(Result<bool, error::Error>) + 'static>(&self, callback: F) {
        let trampoline: AsyncReadyCallback = unsafe { transmute(collection_delete_trampoline as usize) };
        type BoxedFn = Box<Fn(Result<bool, error::Error>) + 'static>;
        let f: Box<BoxedFn> = Box::new(Box::new(callback));
        let user_data: *mut c_void = Box::into_raw(f) as *mut _;
        unsafe { ffi::secret_collection_delete(self.to_glib_none().0, null_mut(), trampoline, user_data) };
    }

    pub fn item_create<F: Fn(Result<Item, error::Error>) + 'static>(&self, schema: &Schema, label: &str, password: &str, attributes: &HashMap<String, String>, callback: F) {
        let (_strings, hash_table) = unsafe { to_glib_string_hash_map(schema.to_glib_none().0, attributes) };
        let trampoline: AsyncReadyCallback = unsafe { transmute(item_create_trampoline as usize) };
        type BoxedFn = Box<Fn(Result<Item, error::Error>) + 'static>;
        let f: Box<BoxedFn> = Box::new(Box::new(callback));
        let user_data: *mut c_void = Box::into_raw(f) as *mut _;
        let label = CString::new(label).unwrap();
        let password = CString::new(password).unwrap();
        let content_type = CString::new("text/plain").unwrap();
        let value = unsafe { ffi::secret_value_new(password.as_ptr(), -1, content_type.as_ptr()) };
        unsafe {
            ffi::secret_item_create(
                self.to_glib_none().0, schema.to_glib_none().0, hash_table,
                label.as_ptr(), value, ITEM_CREATE_REPLACE.to_glib(), ::std::ptr::null_mut(), trampoline, user_data,
            );
        }
    }

    pub fn search<F: Fn(Result<Vec<Item>, error::Error>) + 'static>(&self, schema: &Schema, attributes: &HashMap<String, String>, callback: F) {
        let (_strings, hash_table) = unsafe { to_glib_string_hash_map(schema.to_glib_none().0, attributes) };
        let trampoline: AsyncReadyCallback = unsafe { transmute(collection_search_trampoline as usize) };
        type BoxedFn = Box<Fn(Result<Vec<Item>, ::glib::error::Error>) + 'static>;
        let f: Box<BoxedFn> = Box::new(Box::new(callback));
        let user_data: *mut c_void = Box::into_raw(f) as *mut _;

        unsafe {
            ffi::secret_collection_search(self.to_glib_none().0, schema.to_glib_none().0,
                hash_table, (SEARCH_ALL | SEARCH_LOAD_SECRETS | SEARCH_UNLOCK).to_glib(), null_mut(), trampoline, user_data)
        }
    }
}

unsafe extern "C" fn collection_create_trampoline(_this: *mut gobject_ffi::GObject, result: *mut gio_sys::GAsyncResult, f: glib_ffi::gpointer) {
    callback_guard!();
    let mut error = null_mut();
    let result = ffi::secret_collection_create_finish(result, &mut error);
    let value =
        if !error.is_null() {
            Err(from_glib_full(error))
        }
        else {
            Ok(from_glib_none(result))
        };
    let f: &Box<Fn(Result<Collection, error::Error>) + 'static> = &*(f as *const _);
    f(value)
}

unsafe extern "C" fn collection_delete_trampoline(this: *mut gobject_ffi::GObject, result: *mut gio_sys::GAsyncResult, f: glib_ffi::gpointer) {
    callback_guard!();
    let mut error = null_mut();
    let result = ffi::secret_collection_delete_finish(this as *mut _, result, &mut error);
    let value =
        if !error.is_null() {
            Err(from_glib_full(error))
        }
        else {
            Ok(FromGlib::from_glib(result))
        };
    let f: &Box<Fn(Result<bool, error::Error>) + 'static> = &*(f as *const _);
    f(value)
}

unsafe extern "C" fn collection_search_trampoline(this: *mut gobject_ffi::GObject, result: *mut gio_sys::GAsyncResult, f: glib_ffi::gpointer) {
    callback_guard!();
    let mut error = ::std::ptr::null_mut();
    let result = ffi::secret_collection_search_finish(this as *mut _, result, &mut error);
    let value =
        if !error.is_null() {
            Err(from_glib_full(error))
        }
        else if !result.is_null() {
            let mut items = vec![];
            let mut list = result;
            while !list.is_null() {
                items.push(from_glib_none((*list).data as *mut _));
                list = (*list).next;
            }
            Ok(items)
        }
        else {
            Ok(vec![])
        };
    let f: &Box<Fn(Result<Vec<Item>, ::glib::error::Error>) + 'static> = &*(f as *const _);
    f(value)
}

unsafe extern "C" fn item_create_trampoline(_this: *mut gobject_ffi::GObject, result: *mut gio_sys::GAsyncResult, f: glib_ffi::gpointer) {
    callback_guard!();
    let mut error = null_mut();
    let result = ffi::secret_item_create_finish(result, &mut error);
    let value =
        if !error.is_null() {
            Err(from_glib_full(error))
        }
        else {
            Ok(from_glib_none(result))
        };
    let f: &Box<Fn(Result<Item, error::Error>) + 'static> = &*(f as *const _);
    f(value)
}
