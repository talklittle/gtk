// This file was generated by gir (a4dcfea) from gir-files (0bcaef9)
// DO NOT EDIT

use Adjustment;
use CellRenderer;
use CellRendererText;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct CellRendererSpin(Object<ffi::GtkCellRendererSpin>): CellRendererText, CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_spin_get_type(),
    }
}

impl CellRendererSpin {
    pub fn new() -> CellRendererSpin {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_spin_new()).downcast_unchecked()
        }
    }
}

pub trait CellRendererSpinExt {
    fn get_property_adjustment(&self) -> Option<Adjustment>;

    fn set_property_adjustment(&self, adjustment: Option<&Adjustment>);

    fn get_property_climb_rate(&self) -> f64;

    fn set_property_climb_rate(&self, climb_rate: f64);

    fn get_property_digits(&self) -> u32;

    fn set_property_digits(&self, digits: u32);
}

impl<O: IsA<CellRendererSpin> + IsA<glib::object::Object>> CellRendererSpinExt for O {
    fn get_property_adjustment(&self) -> Option<Adjustment> {
        let mut value = Value::from(None::<&Adjustment>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "adjustment".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_adjustment(&self, adjustment: Option<&Adjustment>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "adjustment".to_glib_none().0, Value::from(adjustment).to_glib_none().0);
        }
    }

    fn get_property_climb_rate(&self) -> f64 {
        let mut value = Value::from(&0f64);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "climb-rate".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_climb_rate(&self, climb_rate: f64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "climb-rate".to_glib_none().0, Value::from(&climb_rate).to_glib_none().0);
        }
    }

    fn get_property_digits(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "digits".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_digits(&self, digits: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "digits".to_glib_none().0, Value::from(&digits).to_glib_none().0);
        }
    }
}
