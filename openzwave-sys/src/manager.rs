use libc::{ c_void, c_char };
use notification::Notification;

pub enum Manager {}

#[repr(C)]
#[derive(Debug)]
pub enum ControllerInterface {
    ControllerInterface_Unknown = 0,
    ControllerInterface_Serial,
    ControllerInterface_Hid
}

pub use self::{
    manager_create as create,
    manager_get as get,
    manager_destroy as destroy,
    manager_add_watcher as add_watcher,
    manager_remove_watcher as remove_watcher,
    manager_add_driver as add_driver,
    manager_remove_driver as remove_driver,
    manager_get_controller_node_id as get_controller_node_id,
    manager_get_suc_node_id as get_suc_node_id,
    manager_is_primary_controller as is_primary_controller,
    manager_is_bridge_controller as is_bridge_controller,
    manager_get_send_queue_count as get_send_queue_count,
    manager_log_driver_statistics as log_driver_statistics,
    manager_get_controller_interface_type as get_controller_interface_type,
    manager_get_library_version as get_library_version,
    manager_get_library_type_name as get_library_type_name,
    manager_get_controller_path as get_controller_path
};

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
    pub fn manager_get_controller_node_id(manager: *mut Manager,
                                          home_id: u32) -> u8;
    pub fn manager_get_suc_node_id(manager: *mut Manager, home_id: u32) -> u8;
    pub fn manager_is_primary_controller(manager: *mut Manager, home_id: u32) -> bool;
    pub fn manager_is_bridge_controller(manager: *mut Manager, home_id: u32) -> bool;
    pub fn manager_get_send_queue_count(manager: *mut Manager, home_id: u32) -> i32;
    pub fn manager_log_driver_statistics(manager: *mut Manager, home_id: u32);
    pub fn manager_get_controller_interface_type(manager: *mut Manager, home_id: u32) -> ControllerInterface;
    pub fn manager_get_library_version(manager: *mut Manager, home_id: u32,
                                       rust_string_creator: extern fn(*const c_char) -> *const c_char) -> *const c_char;
    pub fn manager_get_library_type_name(manager: *mut Manager, home_id: u32,
                                         rust_string_creator: extern fn(*const c_char) -> *const c_char) -> *const c_char;
    pub fn manager_get_controller_path(manager: *mut Manager, home_id: u32,
                                       rust_string_creator: extern fn(*const c_char) -> *const c_char) -> *const c_char;
}
