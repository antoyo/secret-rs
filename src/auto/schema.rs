// This file was generated by gir (fb75f57) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct Schema(Shared<ffi::SecretSchema>);

    match fn {
        ref => |ptr| ffi::secret_schema_ref(ptr),
        unref => |ptr| ffi::secret_schema_unref(ptr),
    }
}

impl Schema {

    //pub fn new(name: &str, flags: SchemaFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Schema {
    //    unsafe { TODO: call ffi::secret_schema_new() }
    //}

    //pub fn newv(name: &str, flags: SchemaFlags, attribute_names_and_types: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 1, id: 20 }) -> Schema {
    //    unsafe { TODO: call ffi::secret_schema_newv() }
    //}
}
