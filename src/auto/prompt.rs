// This file was generated by gir (fb75f57) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct Prompt(Object<ffi::SecretPrompt>);

    match fn {
        get_type => || ffi::secret_prompt_get_type(),
    }
}

impl Prompt {
    //pub fn perform(&self, window_id: Option<&str>, return_type: /*Ignored*/&glib::VariantTy, cancellable: /*Ignored*/Option<&gio::Cancellable>, callback: /*Unknown conversion*//*Unimplemented*/AsyncReadyCallback, user_data: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::secret_prompt_perform() }
    //}

    //pub fn perform_finish<T: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &T) -> Result</*Ignored*/glib::Variant, Error> {
    //    unsafe { TODO: call ffi::secret_prompt_perform_finish() }
    //}

    //pub fn perform_sync(&self, window_id: Option<&str>, cancellable: /*Ignored*/Option<&gio::Cancellable>, return_type: /*Ignored*/&glib::VariantTy) -> Result</*Ignored*/glib::Variant, Error> {
    //    unsafe { TODO: call ffi::secret_prompt_perform_sync() }
    //}

    //pub fn run(&self, window_id: Option<&str>, cancellable: /*Ignored*/Option<&gio::Cancellable>, return_type: /*Ignored*/&glib::VariantTy) -> Result</*Ignored*/glib::Variant, Error> {
    //    unsafe { TODO: call ffi::secret_prompt_run() }
    //}
}
