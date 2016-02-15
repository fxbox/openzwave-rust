use utils::res_to_result;
use libc::c_void;
use std::ffi::CString;
use notification::{Notification, ExternNotification};
use options::Options;
use ffi::manager as extern_manager;

pub struct Manager {
    ptr: *mut extern_manager::Manager,
    options: Options
}

pub struct Watcher {
    cb: Box<FnMut(Notification) -> ()>
}

impl Watcher {
    pub fn new<F: 'static>(callback: F) -> Watcher
    where F: FnMut(Notification) -> () {
        Watcher {
            cb: Box::new(callback)
        }
    }
}

extern "C" fn watcher_cb(notification: *const ExternNotification, watcher: *mut c_void) {
    let watcher: &mut Watcher = unsafe { &mut *(watcher as *mut Watcher) };
    (watcher.cb)(Notification::new(notification)); // TODO use thread synchronization
}

impl Manager {
    pub fn create(mut options: Options) -> Result<Manager, ()> {
        try!(options.lock());
        let external_manager = unsafe { extern_manager::manager_create() };
        if external_manager.is_null() {
            Err(())
        } else { 
            Ok(Manager { ptr: external_manager, options: options })
        }
    }

    pub fn get() -> Option<Manager> {
        let external_manager = unsafe { extern_manager::manager_get() };
        if external_manager.is_null() {
            None
        } else {
            Some(Manager { ptr: external_manager, options: Options::get().unwrap() })
        }
    }

    pub fn add_watcher(&mut self, watcher: &mut Watcher) -> Result<(), ()> {
        let watcher_ptr: *mut c_void = watcher as *mut _ as *mut c_void;
        res_to_result(unsafe {
            extern_manager::manager_add_watcher(self.ptr, watcher_cb, watcher_ptr)
        })
    }

    pub fn remove_watcher(&mut self, watcher: &mut Watcher) -> Result<(), ()> {
        let watcher_ptr: *mut c_void = watcher as *mut _ as *mut c_void;
        res_to_result(unsafe {
            extern_manager::manager_remove_watcher(self.ptr, watcher_cb, watcher_ptr)
        })
    }

    pub fn add_driver(&mut self, device: &str) -> Result<(), ()> {
        let device = CString::new(device).unwrap();
        res_to_result(unsafe {
            extern_manager::manager_add_driver(self.ptr, device.as_ptr(), &extern_manager::ControllerInterface::ControllerInterface_Serial)
        })
    }

    pub fn add_usb_driver(&mut self) -> Result<(), ()> {
        let device = CString::new("HID Controller").unwrap();
        res_to_result(unsafe {
            extern_manager::manager_add_driver(self.ptr, device.as_ptr(), &extern_manager::ControllerInterface::ControllerInterface_Hid)
        })
    }

    pub fn remove_driver(&mut self, device: &str) -> Result<(), ()> {
        let device = CString::new(device).unwrap();
        res_to_result(unsafe {
            extern_manager::manager_remove_driver(self.ptr, device.as_ptr())
        })
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        unsafe { extern_manager::manager_destroy() }
    }
}

