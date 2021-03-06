// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Error;
use ffi;
use gdk_pixbuf;
use gio;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct RecentInfo(Shared<ffi::GtkRecentInfo>);

    match fn {
        ref => |ptr| ffi::gtk_recent_info_ref(ptr),
        unref => |ptr| ffi::gtk_recent_info_unref(ptr),
        get_type => || ffi::gtk_recent_info_get_type(),
    }
}

impl RecentInfo {
    pub fn create_app_info<'a, P: Into<Option<&'a str>>>(&self, app_name: P) -> Result<Option<gio::AppInfo>, Error> {
        let app_name = app_name.into();
        let app_name = app_name.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_recent_info_create_app_info(self.to_glib_none().0, app_name.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn exists(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_exists(self.to_glib_none().0))
        }
    }

    pub fn get_added(&self) -> libc::c_long {
        unsafe {
            ffi::gtk_recent_info_get_added(self.to_glib_none().0)
        }
    }

    pub fn get_age(&self) -> i32 {
        unsafe {
            ffi::gtk_recent_info_get_age(self.to_glib_none().0)
        }
    }

    pub fn get_application_info(&self, app_name: &str) -> Option<(String, u32, libc::c_long)> {
        unsafe {
            let mut app_exec = ptr::null();
            let mut count = mem::uninitialized();
            let mut time_ = mem::uninitialized();
            let ret = from_glib(ffi::gtk_recent_info_get_application_info(self.to_glib_none().0, app_name.to_glib_none().0, &mut app_exec, &mut count, &mut time_));
            if ret { Some((from_glib_none(app_exec), count, time_)) } else { None }
        }
    }

    pub fn get_applications(&self) -> Vec<String> {
        unsafe {
            let mut length = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_full_num(ffi::gtk_recent_info_get_applications(self.to_glib_none().0, &mut length), length as usize);
            ret
        }
    }

    pub fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_recent_info_get_description(self.to_glib_none().0))
        }
    }

    pub fn get_display_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_recent_info_get_display_name(self.to_glib_none().0))
        }
    }

    pub fn get_gicon(&self) -> Option<gio::Icon> {
        unsafe {
            from_glib_full(ffi::gtk_recent_info_get_gicon(self.to_glib_none().0))
        }
    }

    pub fn get_groups(&self) -> Vec<String> {
        unsafe {
            let mut length = mem::uninitialized();
            let ret = FromGlibContainer::from_glib_full_num(ffi::gtk_recent_info_get_groups(self.to_glib_none().0, &mut length), length as usize);
            ret
        }
    }

    pub fn get_icon(&self, size: i32) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_full(ffi::gtk_recent_info_get_icon(self.to_glib_none().0, size))
        }
    }

    pub fn get_mime_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_recent_info_get_mime_type(self.to_glib_none().0))
        }
    }

    pub fn get_modified(&self) -> libc::c_long {
        unsafe {
            ffi::gtk_recent_info_get_modified(self.to_glib_none().0)
        }
    }

    pub fn get_private_hint(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_get_private_hint(self.to_glib_none().0))
        }
    }

    pub fn get_short_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_recent_info_get_short_name(self.to_glib_none().0))
        }
    }

    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_recent_info_get_uri(self.to_glib_none().0))
        }
    }

    pub fn get_uri_display(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_recent_info_get_uri_display(self.to_glib_none().0))
        }
    }

    pub fn get_visited(&self) -> libc::c_long {
        unsafe {
            ffi::gtk_recent_info_get_visited(self.to_glib_none().0)
        }
    }

    pub fn has_application(&self, app_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_has_application(self.to_glib_none().0, app_name.to_glib_none().0))
        }
    }

    pub fn has_group(&self, group_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_has_group(self.to_glib_none().0, group_name.to_glib_none().0))
        }
    }

    pub fn is_local(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_is_local(self.to_glib_none().0))
        }
    }

    pub fn last_application(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_recent_info_last_application(self.to_glib_none().0))
        }
    }

    pub fn match_(&self, info_b: &RecentInfo) -> bool {
        unsafe {
            from_glib(ffi::gtk_recent_info_match(self.to_glib_none().0, info_b.to_glib_none().0))
        }
    }
}
