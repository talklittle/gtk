// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use TextBuffer;
use ffi;
use gdk;
use gdk_pixbuf;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct SelectionData(Boxed<ffi::GtkSelectionData>);

    match fn {
        copy => |ptr| ffi::gtk_selection_data_copy(mut_override(ptr)),
        free => |ptr| ffi::gtk_selection_data_free(ptr),
        get_type => || ffi::gtk_selection_data_get_type(),
    }
}

impl SelectionData {
    pub fn get_data_type(&self) -> gdk::Atom {
        unsafe {
            from_glib_none(ffi::gtk_selection_data_get_data_type(self.to_glib_none().0))
        }
    }

    pub fn get_data_with_length(&self) -> Vec<u8> {
        unsafe {
            let mut length = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_none_num(ffi::gtk_selection_data_get_data_with_length(self.to_glib_none().0, &mut length), length as usize);
            ret
        }
    }

    pub fn get_display(&self) -> Option<gdk::Display> {
        unsafe {
            from_glib_none(ffi::gtk_selection_data_get_display(self.to_glib_none().0))
        }
    }

    pub fn get_format(&self) -> i32 {
        unsafe {
            ffi::gtk_selection_data_get_format(self.to_glib_none().0)
        }
    }

    pub fn get_length(&self) -> i32 {
        unsafe {
            ffi::gtk_selection_data_get_length(self.to_glib_none().0)
        }
    }

    pub fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_full(ffi::gtk_selection_data_get_pixbuf(self.to_glib_none().0))
        }
    }

    pub fn get_selection(&self) -> gdk::Atom {
        unsafe {
            from_glib_none(ffi::gtk_selection_data_get_selection(self.to_glib_none().0))
        }
    }

    pub fn get_target(&self) -> gdk::Atom {
        unsafe {
            from_glib_none(ffi::gtk_selection_data_get_target(self.to_glib_none().0))
        }
    }

    pub fn get_targets(&self) -> Option<Vec<gdk::Atom>> {
        unsafe {
            let mut targets = ptr::null_mut();
            let mut n_atoms = mem::uninitialized();
            let ret = from_glib(ffi::gtk_selection_data_get_targets(self.to_glib_none().0, &mut targets, &mut n_atoms));
            if ret { Some(FromGlibContainer::from_glib_container_num(targets, n_atoms as usize)) } else { None }
        }
    }

    pub fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_selection_data_get_text(self.to_glib_none().0))
        }
    }

    pub fn get_uris(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_selection_data_get_uris(self.to_glib_none().0))
        }
    }

    pub fn set(&mut self, type_: &gdk::Atom, format: i32, data: &[u8]) {
        let length = data.len() as i32;
        unsafe {
            ffi::gtk_selection_data_set(self.to_glib_none_mut().0, type_.to_glib_none().0, format, data.to_glib_none().0, length);
        }
    }

    pub fn set_pixbuf(&self, pixbuf: &gdk_pixbuf::Pixbuf) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_data_set_pixbuf(mut_override(self.to_glib_none().0), pixbuf.to_glib_none().0))
        }
    }

    pub fn set_text(&self, str: &str) -> bool {
        let len = str.len() as i32;
        unsafe {
            from_glib(ffi::gtk_selection_data_set_text(mut_override(self.to_glib_none().0), str.to_glib_none().0, len))
        }
    }

    pub fn set_uris(&self, uris: &[&str]) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_data_set_uris(mut_override(self.to_glib_none().0), uris.to_glib_none().0))
        }
    }

    pub fn targets_include_image(&self, writable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_data_targets_include_image(self.to_glib_none().0, writable.to_glib()))
        }
    }

    pub fn targets_include_rich_text(&self, buffer: &TextBuffer) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_data_targets_include_rich_text(self.to_glib_none().0, buffer.to_glib_none().0))
        }
    }

    pub fn targets_include_text(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_data_targets_include_text(self.to_glib_none().0))
        }
    }

    pub fn targets_include_uri(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_selection_data_targets_include_uri(self.to_glib_none().0))
        }
    }
}
