use ffi::manager;
use libc::c_char;
use utils::get_string_callback;
use std::ffi::CString;

pub struct Controller {
    home_id: u32,
    ptr: *mut manager::Manager
}

macro_rules! network_impl {
    ( $( $name: ident -> $type_: ty ),+ ) => {
        $(
            pub fn $name(&self) -> $type_ {
                unsafe { manager::$name(self.ptr, self.home_id) }
            }
        )*
    };
}

macro_rules! network_impl_string {
    ( $( $name: ident ),+ ) => {
        $(
            pub fn $name(&self) -> String {
                unsafe {
                    CString::from_raw(manager::$name(self.ptr, self.home_id, get_string_callback) as *mut c_char)
                }.into_string().unwrap()
            }
         )*
    };
}

impl Controller {
    pub fn new(home_id: u32) -> Option<Controller> {
        let manager_ptr = unsafe { manager::get() };
        if manager_ptr.is_null() {
            None
        } else {
            Some(Controller {
                ptr: manager_ptr,
                home_id: home_id
            })
        }
    }

    network_impl! {
        get_controller_node_id -> u8,
        get_suc_node_id -> u8,
        is_primary_controller -> bool,
        is_bridge_controller -> bool,
        get_send_queue_count -> i32,
        log_driver_statistics -> (),
        get_controller_interface_type -> manager::ControllerInterface
    }

    network_impl_string! {
        get_library_version,
        get_library_type_name,
        get_controller_path
    }
}
