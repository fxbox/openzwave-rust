#[link(name = "openzwave-c", kind = "static")]
mod extern_manager {
    use libc::c_void;

    pub enum Manager {}
    pub enum Notification {} // TODO should likely be in its own mod

    extern {
        pub fn manager_create() -> *mut Manager;
        pub fn manager_get() -> *mut Manager;
        pub fn manager_destroy();
        pub fn manager_add_watcher(manager: *mut Manager,
                                   cb: extern fn(notification: *const Notification, ctx: *mut c_void),
                                   ctx: *mut c_void) -> bool;
        pub fn manager_remove_watcher(manager: *mut Manager,
                                      cb: extern fn(notification: *const Notification, ctx: *mut c_void),
                                      ctx: *mut c_void) -> bool;
    }
}

use utils::res_to_result;
use libc::c_void;

pub struct Manager {
    ptr: *mut extern_manager::Manager
}

pub fn create() -> Result<Manager, ()> {
    let external_manager = unsafe { extern_manager::manager_create() };
    if external_manager.is_null() {
        Err(())
    } else { 
        Ok(Manager { ptr: external_manager })
    }
}

pub fn get() -> Option<Manager> {
    let external_manager = unsafe { extern_manager::manager_get() };
    if external_manager.is_null() {
        None
    } else {
        Some(Manager { ptr: external_manager })
    }
}

pub fn destroy() {
    unsafe { extern_manager::manager_destroy() }
}

pub struct Notification {
    pub a: i32
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

extern "C" fn watcher_cb(notification: *const extern_manager::Notification, watcher: *mut c_void) {
    let watcher: &mut Watcher = unsafe { &mut *(watcher as *mut Watcher) };
    (watcher.cb)(Notification { a: 2 }); // TODO use thread synchronization
}

impl Manager {
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
}
