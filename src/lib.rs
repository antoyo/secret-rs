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

#[macro_use]
extern crate bitflags;
extern crate gio_sys;
#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gtk;
extern crate libc;

extern crate secret_sys as ffi;

macro_rules! assert_initialized_main_thread {
    () => (
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            }
            else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    )
}

macro_rules! skip_assert_initialized {
    () => ()
}

#[macro_export]
macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

#[macro_use]
mod macros;
mod auto;
mod item;
mod schema;
mod password;

pub use auto::*;
pub use password::*;
