use ffi::manager as extern_manager;
use libc::c_char;
use ffi::utils::{ rust_string_creator, recover_string };

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Controller {
    home_id: u32
}

macro_rules! network_impl {
    ( $( $name: ident -> $type_: ty ),+ ) => {
        $(
            pub fn $name(&self) -> $type_ {
                let manager_ptr = unsafe { extern_manager::get() };
                unsafe { extern_manager::$name(manager_ptr, self.home_id) }
            }
        )*
    };
}

macro_rules! network_impl_string {
    ( $( $name: ident ),+ ) => {
        $(
            pub fn $name(&self) -> String {
                let manager_ptr = unsafe { extern_manager::get() };
                recover_string(
                    unsafe {
                        extern_manager::$name(manager_ptr, self.home_id, rust_string_creator)
                    } as *mut c_char
                )
            }
         )*
    };
}

impl Controller {
    pub fn new(home_id: u32) -> Controller {
        Controller {
            home_id: home_id
        }
    }

    pub fn get_home_id(&self) -> u32 {
        self.home_id
    }

    network_impl! {
        get_controller_node_id -> u8,
        get_suc_node_id -> u8,
        is_primary_controller -> bool,
        is_bridge_controller -> bool,
        get_send_queue_count -> i32,
        log_driver_statistics -> (),
        write_config -> (),
        get_controller_interface_type -> extern_manager::ControllerInterface
    }

    network_impl_string! {
        get_library_version,
        get_library_type_name,
        get_controller_path
    }
}

use std::fmt::{ self, Debug, Display, Formatter };

impl Display for Controller {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad(&format!("{}: HomeId: {:08x} Node: {} Path: {}",
                       self.get_library_type_name(),
                       self.get_home_id(),
                       self.get_controller_node_id(),
                       self.get_controller_path()))
    }
}

impl Debug for Controller {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Controller {{ home_id: {:?}, controller_node_id: {:?}, suc_node_id: {:?}, is_primary_controller: {:?}, is_bridge_controller: {:?}, send_queue_count: {:?}, controller_interface_type: {:?}, controller_path: {:?}, library_type_name: {:?}, library_version: {:?} }}",
               self.home_id,
               self.get_controller_node_id(),
               self.get_suc_node_id(),
               self.is_primary_controller(),
               self.is_bridge_controller(),
               self.get_send_queue_count(),
               self.get_controller_interface_type(),
               self.get_controller_path(),
               self.get_library_type_name(),
               self.get_library_version()
              )
    }
}
