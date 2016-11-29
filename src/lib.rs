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
