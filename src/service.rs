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

use std::mem::transmute;
use std::ptr::null_mut;

use ffi;
use gio_sys;
use glib::error;
use glib::translate::{FromGlib, ToGlib, ToGlibPtr, from_glib_full, from_glib_none};
use glib_ffi;
use gobject_ffi;
use libc::c_void;

use AsyncReadyCallback;
use Service;
use SERVICE_NONE;

impl Service {
    pub fn get<F: Fn(Result<Service, error::Error>) + 'static>(callback: F) {
        let trampoline: AsyncReadyCallback = unsafe { transmute(service_get_trampoline as usize) };
        type BoxedFn = Box<Fn(Result<Service, error::Error>) + 'static>;
        let f: Box<BoxedFn> = Box::new(Box::new(callback));
        let user_data: *mut c_void = Box::into_raw(f) as *mut _;
        unsafe { ffi::secret_service_get(SERVICE_NONE.to_glib(), null_mut(), trampoline, user_data) };
    }

    pub fn load_collections<F: FnOnce(Result<bool, error::Error>) + 'static>(&self, callback: F) {
        let trampoline: AsyncReadyCallback = unsafe { transmute(service_load_collections_trampoline as usize) };
        type BoxedFn = Box<FnOnce(Result<bool, error::Error>) + 'static>;
        let f: Box<BoxedFn> = Box::new(Box::new(callback));
        let user_data: *mut c_void = Box::into_raw(f) as *mut _;
        unsafe { ffi::secret_service_load_collections(self.to_glib_none().0, null_mut(), trampoline, user_data) };
    }
}

unsafe extern "C" fn service_get_trampoline(_this: *mut gobject_ffi::GObject, result: *mut gio_sys::GAsyncResult, f: glib_ffi::gpointer) {
    callback_guard!();
    let mut error = null_mut();
    let result = ffi::secret_service_get_finish(result, &mut error);
    let value =
        if !error.is_null() {
            Err(from_glib_full(error))
        }
        else {
            Ok(from_glib_none(result))
        };
    let f: &Box<Fn(Result<Service, error::Error>) + 'static> = &*(f as *const _);
    f(value)
}

unsafe extern "C" fn service_load_collections_trampoline(this: *mut gobject_ffi::GObject, result: *mut gio_sys::GAsyncResult, f: glib_ffi::gpointer) {
    callback_guard!();
    let mut error = null_mut();
    let result = ffi::secret_service_load_collections_finish(this as *mut _, result, &mut error);
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
