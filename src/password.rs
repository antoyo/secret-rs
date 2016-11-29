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

use gio_sys;
use gobject_ffi;
use libc::c_void;

use Schema;

#[macro_export]
macro_rules! clear {
    ($passwords:expr, $callback:expr, { $($name:ident: $value:expr,)* }) => {{
        use glib::translate::ToGlibPtr;

        unsafe extern "C" fn password_clear_trampoline(_this: *mut gobject_sys::GObject, result: *mut gio_sys::GAsyncResult, f: glib_sys::gpointer) {
            callback_guard!();
            let mut error = ::std::ptr::null_mut();
            let result = ::secret_sys::secret_password_clear_finish(result, &mut error);
            let value =
                if result != 0 {
                    Ok(true)
                }
                else {
                    Err(::glib::translate::from_glib_full(error))
                };
            let f: &Box<Fn(Result<bool, ::glib::error::Error>) + 'static> = &*(f as *const _);
            f(value)
        }

        let trampoline: ::secret::AsyncReadyCallback = unsafe { ::std::mem::transmute(password_clear_trampoline as usize) };
        let f: Box<Box<Fn(Result<bool, ::glib::error::Error>) + 'static>> = Box::new(Box::new($callback));
        let user_data: *mut libc::c_void = Box::into_raw(f) as *mut _;
        unsafe {
            ::secret_sys::secret_password_clear(
                $passwords.schema.to_glib_none().0, ::std::ptr::null_mut(),
                trampoline, user_data,
                $(c_stringify!($name), $value as i64,)*
                ::std::ptr::null_mut::<::libc::c_void>()
            );
        }
    }}
}

#[macro_export]
macro_rules! lookup {
    ($passwords:expr, $callback:expr, { $($name:ident: $value:expr,)* }) => {{
        use glib::translate::ToGlibPtr;

        unsafe extern "C" fn password_lookup_trampoline(_this: *mut gobject_sys::GObject, result: *mut gio_sys::GAsyncResult, f: glib_sys::gpointer) {
            callback_guard!();
            let mut error = ::std::ptr::null_mut();
            let result = ::secret_sys::secret_password_lookup_finish(result, &mut error);
            let value =
                if result.is_null() {
                    Err(::glib::translate::from_glib_full(error))
                }
                else {
                    Ok(::glib::translate::from_glib_none(result))
                };
            let f: &Box<Fn(Result<String, ::glib::error::Error>) + 'static> = &*(f as *const _);
            f(value)
        }

        let trampoline: ::secret::AsyncReadyCallback = unsafe { ::std::mem::transmute(password_lookup_trampoline as usize) };
        let f: Box<Box<Fn(Result<String, ::glib::error::Error>) + 'static>> = Box::new(Box::new($callback));
        let user_data: *mut libc::c_void = Box::into_raw(f) as *mut _;
        unsafe {
            ::secret_sys::secret_password_lookup(
                $passwords.schema.to_glib_none().0, ::std::ptr::null_mut(),
                trampoline, user_data,
                $(c_stringify!($name), $value as i64,)*
                ::std::ptr::null_mut::<::libc::c_void>()
            );
        }
    }}
}

#[macro_export]
macro_rules! store {
    ($passwords:expr, $label:expr, $password:expr, $callback:expr, { $($name:ident: $value:expr,)* }) => {{
        use glib::translate::ToGlibPtr;

        unsafe extern "C" fn password_store_trampoline(_this: *mut gobject_sys::GObject, result: *mut gio_sys::GAsyncResult, f: glib_sys::gpointer) {
            callback_guard!();
            let mut error = ::std::ptr::null_mut();
            let result = ::secret_sys::secret_password_store_finish(result, &mut error);
            let value =
                if result != 0 {
                    Ok(true)
                }
                else {
                    Err(::glib::translate::from_glib_full(error))
                };
            let f: &Box<Fn(Result<bool, ::glib::error::Error>) + 'static> = &*(f as *const _);
            f(value)
        }

        let trampoline: ::secret::AsyncReadyCallback = unsafe { ::std::mem::transmute(password_store_trampoline as usize) };
        let f: Box<Box<Fn(Result<bool, ::glib::error::Error>) + 'static>> = Box::new(Box::new($callback));
        let user_data: *mut libc::c_void = Box::into_raw(f) as *mut _;
        let label = ::std::ffi::CString::new($label).unwrap();
        let password = ::std::ffi::CString::new($password).unwrap();
        unsafe {
            ::secret_sys::secret_password_store(
                $passwords.schema.to_glib_none().0, $passwords.collection.to_glib_none().0,
                label.as_ptr(), password.as_ptr(), ::std::ptr::null_mut(), trampoline, user_data,
                $(c_stringify!($name), $value as i64,)*
                ::std::ptr::null_mut::<::libc::c_void>()
            );
        }
    }}
}

pub type AsyncReadyCallback = Option<unsafe extern "C" fn(*mut gobject_ffi::GObject, *mut gio_sys::GAsyncResult, *mut c_void)>;

pub struct Passwords {
    pub collection: Option<String>,
    pub schema: Schema,
}

impl Passwords {
    pub fn new(schema: Schema) -> Self {
        Passwords {
            collection: None,
            schema: schema,
        }
    }
}
