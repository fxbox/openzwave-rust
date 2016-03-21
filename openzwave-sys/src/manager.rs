use libc::{ c_void, c_char, c_float };
use notification::Notification;
use value_classes::value_id::ValueID;
use utils::{ RustStringCreator, RustVecCreator };

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
    manager_get_controller_path as get_controller_path,
    manager_get_poll_interval as get_poll_interval,
    manager_set_poll_interval as set_poll_interval,
    manager_enable_poll_with_intensity as enable_poll_with_intensity,
    manager_enable_poll as enable_poll,
    manager_disable_poll as disable_poll,
    manager_is_polled as is_polled,
    manager_set_poll_intensity as set_poll_intensity,
    manager_get_poll_intensity as get_poll_intensity,
    manager_get_value_label as get_value_label,
    manager_set_value_label as set_value_label,
    manager_get_value_units as get_value_units,
    manager_set_value_units as set_value_units,
    manager_get_value_help as get_value_help,
    manager_set_value_help as set_value_help,
    manager_get_value_min as get_value_min,
    manager_get_value_max as get_value_max,
    manager_is_value_read_only as is_value_read_only,
    manager_is_value_write_only as is_value_write_only,
    manager_is_value_set as is_value_set,
    manager_is_value_polled as is_value_polled,
    manager_get_value_as_bool as get_value_as_bool,
    manager_get_value_as_byte as get_value_as_byte,
    manager_get_value_as_float as get_value_as_float,
    manager_get_value_float_precision as get_value_float_precision,
    manager_get_value_as_int as get_value_as_int,
    manager_get_value_as_short as get_value_as_short,
    manager_get_value_as_string as get_value_as_string,
    manager_get_value_as_raw as get_value_as_raw,
    manager_get_value_list_selection_as_string as get_value_list_selection_as_string,
    manager_get_value_list_selection_as_int as get_value_list_selection_as_int,
    manager_get_value_list_items as get_value_list_items,
    manager_get_value_list_values as get_value_list_values,

    manager_set_value_bool as set_value_bool,
    manager_set_value_byte as set_value_byte,
    manager_set_value_float as set_value_float,
    manager_set_value_int as set_value_int,
    manager_set_value_short as set_value_short,
    manager_set_value_string as set_value_string,
    manager_set_value_raw as set_value_raw,
    manager_set_value_list_selection_string as set_value_list_selection_string,

    manager_node_is_listening_device as is_node_listening_device,
    manager_node_is_frequent_listening_device as is_node_frequent_listening_device,
    manager_node_is_beaming_device as is_node_beaming_device,
    manager_node_is_routing_device as is_node_routing_device,
    manager_node_is_security_device as is_node_security_device,
    manager_node_get_max_baud_rate as get_node_max_baud_rate,
    manager_node_get_version as get_node_version,
    manager_node_get_security as get_node_security,
    manager_node_is_zwave_plus as is_node_zwave_plus,
    manager_node_get_basic as get_node_basic,
    manager_node_get_generic as get_node_generic,
    manager_node_get_specific as get_node_specific,

    manager_node_get_type as get_node_type,
    manager_node_get_manufacturer_name as get_node_manufacturer_name,
    manager_node_get_product_name as get_node_product_name,
    manager_node_get_name as get_node_name,
    manager_node_get_location as get_node_location,
    manager_node_get_manufacturer_id as get_node_manufacturer_id,
    manager_node_get_product_type as get_node_product_type,
    manager_node_get_product_id as get_node_product_id,
    manager_node_get_query_stage as get_node_query_stage,
    manager_node_get_device_type as get_node_device_type,
    manager_node_get_device_type_string as get_node_device_type_string,
    manager_node_get_role as get_node_role,
    manager_node_get_role_string as get_node_role_string,
    manager_node_get_plus_type as get_node_plus_type,
    manager_node_get_plus_type_string as get_node_plus_type_string,

    manager_node_is_info_received as is_node_info_received,
    manager_node_is_awake as is_node_awake,
    manager_node_is_failed as is_node_failed,

    manager_node_get_neighbors as get_node_neighbors,
    manager_node_get_class_information as get_node_class_information
};

extern {
    pub fn manager_create() -> *mut Manager;
    pub fn manager_get() -> *mut Manager;
    pub fn manager_destroy();
    pub fn manager_add_watcher(manager: *mut Manager,
                               cb: extern fn(notification: *const Notification, ctx: *const c_void),
                               ctx: *const c_void) -> bool;
    pub fn manager_remove_watcher(manager: *mut Manager,
                                  cb: extern fn(notification: *const Notification, ctx: *const c_void),
                                  ctx: *const c_void) -> bool;
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
    pub fn manager_get_library_version(manager: *mut Manager, home_id: u32, rust_string_creator: RustStringCreator) -> *mut c_char;
    pub fn manager_get_library_type_name(manager: *mut Manager, home_id: u32, rust_string_creator: RustStringCreator) -> *mut c_char;
    pub fn manager_get_controller_path(manager: *mut Manager, home_id: u32, rust_string_creator: RustStringCreator) -> *mut c_char;
    pub fn manager_get_poll_interval(manager: *mut Manager) -> i32;
    pub fn manager_set_poll_interval(manager: *mut Manager, interval_ms: i32, is_between_each_poll: bool);
    pub fn manager_enable_poll_with_intensity(manager: *mut Manager, value: *const ValueID, intensity: u8) -> bool;
    pub fn manager_enable_poll(manager: *mut Manager, value: *const ValueID) -> bool;
    pub fn manager_disable_poll(manager: *mut Manager, value: *const ValueID) -> bool;
    pub fn manager_is_polled(manager: *mut Manager, value: *const ValueID) -> bool;
    pub fn manager_set_poll_intensity(manager: *mut Manager, value: *const ValueID, intensity: u8);
    pub fn manager_get_poll_intensity(manager: *mut Manager, value: *const ValueID) -> u8;

    pub fn manager_get_value_label(manager: *mut Manager, id: *const ValueID, stringCreator: RustStringCreator) -> *mut c_char;
    pub fn manager_set_value_label(manager: *mut Manager, id: *const ValueID, str: *const c_char);
    pub fn manager_get_value_units(manager: *mut Manager, id: *const ValueID, stringCreator: RustStringCreator) -> *mut c_char;
    pub fn manager_set_value_units(manager: *mut Manager, id: *const ValueID, str: *const c_char);
    pub fn manager_get_value_help(manager: *mut Manager, id: *const ValueID, stringCreator: RustStringCreator) -> *mut c_char;
    pub fn manager_set_value_help(manager: *mut Manager, id: *const ValueID, str: *const c_char);
    pub fn manager_get_value_min(manager: *mut Manager, id: *const ValueID) -> i32;
    pub fn manager_get_value_max(manager: *mut Manager, id: *const ValueID) -> i32;
    pub fn manager_is_value_read_only(manager: *mut Manager, id: *const ValueID) -> bool;
    pub fn manager_is_value_write_only(manager: *mut Manager, id: *const ValueID) -> bool;
    pub fn manager_is_value_set(manager: *mut Manager, id: *const ValueID) -> bool;
    pub fn manager_is_value_polled(manager: *mut Manager, id: *const ValueID) -> bool;

    pub fn manager_get_value_as_bool(manager: *mut Manager, id: *const ValueID, result: *mut bool) -> bool;
    pub fn manager_get_value_as_byte(manager: *mut Manager, id: *const ValueID, result: *mut u8) -> bool;
    pub fn manager_get_value_as_float(manager: *mut Manager, id: *const ValueID, result: *mut c_float) -> bool;
    pub fn manager_get_value_float_precision(manager: *mut Manager, id: *const ValueID, result: *mut u8) -> bool;
    pub fn manager_get_value_as_int(manager: *mut Manager, id: *const ValueID, result: *mut i32) -> bool;
    pub fn manager_get_value_as_short(manager: *mut Manager, id: *const ValueID, result: *mut i16) -> bool;
    pub fn manager_get_value_as_string(manager: *mut Manager, id: *const ValueID, result: *mut *mut c_char, stringCreator: RustStringCreator) -> bool;
    pub fn manager_get_value_as_raw(manager: *mut Manager, id: *const ValueID, result: *mut *mut c_void, vecCreator: RustVecCreator<u8>) -> bool;
    pub fn manager_get_value_list_selection_as_string(manager: *mut Manager, id: *const ValueID, result: *mut *mut c_char, stringCreator: RustStringCreator) -> bool;
    pub fn manager_get_value_list_selection_as_int(manager: *mut Manager, id: *const ValueID, result: *mut i32) -> bool;
    pub fn manager_get_value_list_items(manager: *mut Manager, id: *const ValueID, result: *mut *mut c_void, vecCreator: RustVecCreator<*const c_char>) -> bool;
    pub fn manager_get_value_list_values(manager: *mut Manager, id: *const ValueID, result: *mut *mut c_void, vecCreator: RustVecCreator<i32>) -> bool;

    pub fn manager_set_value_bool(manager: *mut Manager, id: *const ValueID, value: bool) -> bool;
    pub fn manager_set_value_byte(manager: *mut Manager, id: *const ValueID, value: u8) -> bool;
    pub fn manager_set_value_float(manager: *mut Manager, id: *const ValueID, value: c_float) -> bool;
    pub fn manager_set_value_int(manager: *mut Manager, id: *const ValueID, value: i32) -> bool;
    pub fn manager_set_value_short(manager: *mut Manager, id: *const ValueID, value: i16) -> bool;
    pub fn manager_set_value_string(manager: *mut Manager, id: *const ValueID, value: *const c_char) -> bool;
    pub fn manager_set_value_raw(manager: *mut Manager, id: *const ValueID, value: *const u8, len: u8) -> bool;
    pub fn manager_set_value_list_selection_string(manager: *mut Manager, id: *const ValueID, value: *const c_char) -> bool;

    pub fn manager_node_is_listening_device(manager: *mut Manager, home_id: u32, node_id: u8) -> bool;
    pub fn manager_node_is_frequent_listening_device(manager: *mut Manager, home_id: u32, node_id: u8) -> bool;
    pub fn manager_node_is_beaming_device(manager: *mut Manager, home_id: u32, node_id: u8) -> bool;
    pub fn manager_node_is_routing_device(manager: *mut Manager, home_id: u32, node_id: u8) -> bool;
    pub fn manager_node_is_security_device(manager: *mut Manager, home_id: u32, node_id: u8) -> bool;
    pub fn manager_node_get_max_baud_rate(manager: *mut Manager, home_id: u32, node_id: u8) -> u32;
    pub fn manager_node_get_version(manager: *mut Manager, home_id: u32, node_id: u8) -> u8;
    pub fn manager_node_get_security(manager: *mut Manager, home_id: u32, node_id: u8) -> u8;
    pub fn manager_node_is_zwave_plus(manager: *mut Manager, home_id: u32, node_id: u8) -> bool;
    pub fn manager_node_get_basic(manager: *mut Manager, home_id: u32, node_id: u8) -> u8;
    pub fn manager_node_get_generic(manager: *mut Manager, home_id: u32, node_id: u8) -> u8;
    pub fn manager_node_get_specific(manager: *mut Manager, home_id: u32, node_id: u8) -> u8;

    pub fn manager_node_get_type(manager: *mut Manager, home_id: u32, node_id: u8, stringCreator: RustStringCreator) -> *mut c_char;
    pub fn manager_node_get_manufacturer_name(manager: *mut Manager, home_id: u32, node_id: u8, stringCreator: RustStringCreator) -> *mut c_char;
    pub fn manager_node_get_product_name(manager: *mut Manager, home_id: u32, node_id: u8, stringCreator: RustStringCreator) -> *mut c_char;
    pub fn manager_node_get_name(manager: *mut Manager, home_id: u32, node_id: u8, stringCreator: RustStringCreator) -> *mut c_char;
    pub fn manager_node_get_location(manager: *mut Manager, home_id: u32, node_id: u8, stringCreator: RustStringCreator) -> *mut c_char;
    pub fn manager_node_get_manufacturer_id(manager: *mut Manager, home_id: u32, node_id: u8, stringCreator: RustStringCreator) -> *mut c_char;
    pub fn manager_node_get_product_type(manager: *mut Manager, home_id: u32, node_id: u8, stringCreator: RustStringCreator) -> *mut c_char;
    pub fn manager_node_get_product_id(manager: *mut Manager, home_id: u32, node_id: u8, stringCreator: RustStringCreator) -> *mut c_char;
    pub fn manager_node_get_query_stage(manager: *mut Manager, home_id: u32, node_id: u8, stringCreator: RustStringCreator) -> *mut c_char;
    pub fn manager_node_get_device_type(manager: *mut Manager, home_id: u32, node_id: u8) -> u16;
    pub fn manager_node_get_device_type_string(manager: *mut Manager, home_id: u32, node_id: u8, stringCreator: RustStringCreator) -> *mut c_char;
    pub fn manager_node_get_role(manager: *mut Manager, home_id: u32, node_id: u8) -> u8;
    pub fn manager_node_get_role_string(manager: *mut Manager, home_id: u32, node_id: u8, stringCreator: RustStringCreator) -> *mut c_char;
    pub fn manager_node_get_plus_type(manager: *mut Manager, home_id: u32, node_id: u8) -> u8;
    pub fn manager_node_get_plus_type_string(manager: *mut Manager, home_id: u32, node_id: u8, stringCreator: RustStringCreator) -> *mut c_char;

    pub fn manager_node_is_info_received(manager: *mut Manager, home_id: u32, node_id: u8) -> bool;
    pub fn manager_node_is_awake(manager: *mut Manager, home_id: u32, node_id: u8) -> bool;
    pub fn manager_node_is_failed(manager: *mut Manager, home_id: u32, node_id: u8) -> bool;

    pub fn manager_node_get_neighbors(manager: *mut Manager, home_id: u32, node_id: u8, vecCreator: RustVecCreator<u8>) -> *mut c_void;
    pub fn manager_node_get_class_information(
        manager: *mut Manager, home_id: u32, node_id: u8,
        command_class_id: u8, class_name: *mut *mut c_char, class_version: *mut u8,
        stringCreator: RustStringCreator) -> bool;
}

