// This file was generated by gir (https://github.com/gtk-rs/gir @ ea993ed)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct TreeIter(Boxed<ffi::GtkTreeIter>);

    match fn {
        copy => |ptr| ffi::gtk_tree_iter_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_tree_iter_free(ptr),
        get_type => || ffi::gtk_tree_iter_get_type(),
    }
}
