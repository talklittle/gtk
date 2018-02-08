use std::ptr;

use ffi;
use glib::translate::*;
use {TargetEntry, TargetList};

impl TargetList {
    pub fn new(targets: &[TargetEntry]) -> Self {
        skip_assert_initialized!();
        let stashes: Vec<_> = targets.iter().map(|e| e.to_glib_none()).collect();
        let t: Vec<_> = stashes.iter().map(|stash| unsafe { *stash.0 }).collect();
        let t_ptr: *mut ffi::GtkTargetEntry = if !t.is_empty() {
            t.as_ptr() as *mut _
        } else {
            ptr::null_mut()
        };
        unsafe {
            from_glib_full(ffi::gtk_target_list_new(t_ptr, t.len() as u32))
        }
    }
}
