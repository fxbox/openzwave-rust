use libc::{ c_void, c_char };
use notification::Notification;

pub enum Manager {}

#[repr(C)]
pub enum ControllerInterface {
    ControllerInterface_Unknown = 0,
    ControllerInterface_Serial,
    ControllerInterface_Hid
}

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
    pub fn manager_add_driver(manager: *mut Manager,
                                device: *const c_char,
                                interface: *const ControllerInterface) -> bool;
    pub fn manager_remove_driver(manager: *mut Manager,
                                 device: *const c_char) -> bool;
}
