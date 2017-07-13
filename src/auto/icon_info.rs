// This file was generated by gir (4b09025) from gir-files (71d73f0)
// DO NOT EDIT

use Error;
use IconTheme;
use StyleContext;
#[cfg(feature = "v3_10")]
use cairo;
use ffi;
use gdk;
use gdk_pixbuf;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use std;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct IconInfo(Object<ffi::GtkIconInfo>);

    match fn {
        get_type => || ffi::gtk_icon_info_get_type(),
    }
}

impl IconInfo {
    pub fn new_for_pixbuf(icon_theme: &IconTheme, pixbuf: &gdk_pixbuf::Pixbuf) -> IconInfo {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_icon_info_new_for_pixbuf(icon_theme.to_glib_none().0, pixbuf.to_glib_none().0))
        }
    }
}

pub trait IconInfoExt {
    fn copy(&self) -> Option<IconInfo>;

    //fn get_attach_points(&self, points: /*Unimplemented*/Vec<gdk::Point>) -> Option<i32>;

    #[cfg(feature = "v3_10")]
    fn get_base_scale(&self) -> i32;

    fn get_base_size(&self) -> i32;

    fn get_builtin_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf>;

    fn get_display_name(&self) -> Option<String>;

    fn get_embedded_rect(&self) -> Option<gdk::Rectangle>;

    fn get_filename(&self) -> Option<std::path::PathBuf>;

    #[cfg(feature = "v3_12")]
    fn is_symbolic(&self) -> bool;

    fn load_icon(&self) -> Result<gdk_pixbuf::Pixbuf, Error>;

    //#[cfg(feature = "v3_8")]
    //fn load_icon_async<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R);

    //#[cfg(feature = "v3_8")]
    //fn load_icon_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, res: &P) -> Result<gdk_pixbuf::Pixbuf, Error>;

    #[cfg(feature = "v3_10")]
    fn load_surface<'a, P: Into<Option<&'a gdk::Window>>>(&self, for_window: P) -> Result<cairo::Surface, Error>;

    fn load_symbolic<'a, 'b, 'c, P: Into<Option<&'a gdk::RGBA>>, Q: Into<Option<&'b gdk::RGBA>>, R: Into<Option<&'c gdk::RGBA>>>(&self, fg: &gdk::RGBA, success_color: P, warning_color: Q, error_color: R) -> Result<(gdk_pixbuf::Pixbuf, bool), Error>;

    //#[cfg(feature = "v3_8")]
    //fn load_symbolic_async<'a, 'b, 'c, 'd, 'e, P: Into<Option<&'a gdk::RGBA>>, Q: Into<Option<&'b gdk::RGBA>>, R: Into<Option<&'c gdk::RGBA>>, S: Into<Option<&'d /*Ignored*/gio::Cancellable>>, T: Into<Option<&'e /*Ignored*/gio::AsyncReadyCallback>>, U: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, fg: &gdk::RGBA, success_color: P, warning_color: Q, error_color: R, cancellable: S, callback: T, user_data: U);

    //#[cfg(feature = "v3_8")]
    //fn load_symbolic_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, res: &P) -> Result<(gdk_pixbuf::Pixbuf, bool), Error>;

    fn load_symbolic_for_context(&self, context: &StyleContext) -> Result<(gdk_pixbuf::Pixbuf, bool), Error>;

    //#[cfg(feature = "v3_8")]
    //fn load_symbolic_for_context_async<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, context: &StyleContext, cancellable: P, callback: Q, user_data: R);

    //#[cfg(feature = "v3_8")]
    //fn load_symbolic_for_context_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, res: &P) -> Result<(gdk_pixbuf::Pixbuf, bool), Error>;

    fn set_raw_coordinates(&self, raw_coordinates: bool);
}

impl<O: IsA<IconInfo>> IconInfoExt for O {
    fn copy(&self) -> Option<IconInfo> {
        unsafe {
            from_glib_full(ffi::gtk_icon_info_copy(self.to_glib_none().0))
        }
    }

    //fn get_attach_points(&self, points: /*Unimplemented*/Vec<gdk::Point>) -> Option<i32> {
    //    unsafe { TODO: call ffi::gtk_icon_info_get_attach_points() }
    //}

    #[cfg(feature = "v3_10")]
    fn get_base_scale(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_info_get_base_scale(self.to_glib_none().0)
        }
    }

    fn get_base_size(&self) -> i32 {
        unsafe {
            ffi::gtk_icon_info_get_base_size(self.to_glib_none().0)
        }
    }

    fn get_builtin_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_icon_info_get_builtin_pixbuf(self.to_glib_none().0))
        }
    }

    fn get_display_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_icon_info_get_display_name(self.to_glib_none().0))
        }
    }

    fn get_embedded_rect(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rectangle = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_icon_info_get_embedded_rect(self.to_glib_none().0, rectangle.to_glib_none_mut().0));
            if ret { Some(rectangle) } else { None }
        }
    }

    fn get_filename(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::gtk_icon_info_get_filename(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    fn is_symbolic(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_icon_info_is_symbolic(self.to_glib_none().0))
        }
    }

    fn load_icon(&self) -> Result<gdk_pixbuf::Pixbuf, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_info_load_icon(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //#[cfg(feature = "v3_8")]
    //fn load_icon_async<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::gtk_icon_info_load_icon_async() }
    //}

    //#[cfg(feature = "v3_8")]
    //fn load_icon_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, res: &P) -> Result<gdk_pixbuf::Pixbuf, Error> {
    //    unsafe { TODO: call ffi::gtk_icon_info_load_icon_finish() }
    //}

    #[cfg(feature = "v3_10")]
    fn load_surface<'a, P: Into<Option<&'a gdk::Window>>>(&self, for_window: P) -> Result<cairo::Surface, Error> {
        let for_window = for_window.into();
        let for_window = for_window.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_info_load_surface(self.to_glib_none().0, for_window.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn load_symbolic<'a, 'b, 'c, P: Into<Option<&'a gdk::RGBA>>, Q: Into<Option<&'b gdk::RGBA>>, R: Into<Option<&'c gdk::RGBA>>>(&self, fg: &gdk::RGBA, success_color: P, warning_color: Q, error_color: R) -> Result<(gdk_pixbuf::Pixbuf, bool), Error> {
        let success_color = success_color.into();
        let success_color = success_color.to_glib_none();
        let warning_color = warning_color.into();
        let warning_color = warning_color.to_glib_none();
        let error_color = error_color.into();
        let error_color = error_color.to_glib_none();
        unsafe {
            let mut was_symbolic = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_info_load_symbolic(self.to_glib_none().0, fg.to_glib_none().0, success_color.0, warning_color.0, error_color.0, &mut was_symbolic, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), from_glib(was_symbolic))) } else { Err(from_glib_full(error)) }
        }
    }

    //#[cfg(feature = "v3_8")]
    //fn load_symbolic_async<'a, 'b, 'c, 'd, 'e, P: Into<Option<&'a gdk::RGBA>>, Q: Into<Option<&'b gdk::RGBA>>, R: Into<Option<&'c gdk::RGBA>>, S: Into<Option<&'d /*Ignored*/gio::Cancellable>>, T: Into<Option<&'e /*Ignored*/gio::AsyncReadyCallback>>, U: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, fg: &gdk::RGBA, success_color: P, warning_color: Q, error_color: R, cancellable: S, callback: T, user_data: U) {
    //    unsafe { TODO: call ffi::gtk_icon_info_load_symbolic_async() }
    //}

    //#[cfg(feature = "v3_8")]
    //fn load_symbolic_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, res: &P) -> Result<(gdk_pixbuf::Pixbuf, bool), Error> {
    //    unsafe { TODO: call ffi::gtk_icon_info_load_symbolic_finish() }
    //}

    fn load_symbolic_for_context(&self, context: &StyleContext) -> Result<(gdk_pixbuf::Pixbuf, bool), Error> {
        unsafe {
            let mut was_symbolic = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_icon_info_load_symbolic_for_context(self.to_glib_none().0, context.to_glib_none().0, &mut was_symbolic, &mut error);
            if error.is_null() { Ok((from_glib_full(ret), from_glib(was_symbolic))) } else { Err(from_glib_full(error)) }
        }
    }

    //#[cfg(feature = "v3_8")]
    //fn load_symbolic_for_context_async<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, context: &StyleContext, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::gtk_icon_info_load_symbolic_for_context_async() }
    //}

    //#[cfg(feature = "v3_8")]
    //fn load_symbolic_for_context_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, res: &P) -> Result<(gdk_pixbuf::Pixbuf, bool), Error> {
    //    unsafe { TODO: call ffi::gtk_icon_info_load_symbolic_for_context_finish() }
    //}

    fn set_raw_coordinates(&self, raw_coordinates: bool) {
        unsafe {
            ffi::gtk_icon_info_set_raw_coordinates(self.to_glib_none().0, raw_coordinates.to_glib());
        }
    }
}