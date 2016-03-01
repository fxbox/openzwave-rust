use std::fmt;
use ffi::manager as extern_manager;
use ffi::utils::rust_string_creator;
use std::ffi::CString;

#[derive(Clone, Copy)]
pub struct Node {
    home_id: u32,
    node_id: u8
}

// implements simple node getters
macro_rules! node_getters {
    ( $($name: ident, $impl_name: ident -> $t: ty;)+ ) => {
        $(pub fn $name(&self) -> $t {
            let manager_ptr = unsafe { extern_manager::get() };
            unsafe {
                extern_manager::$impl_name(manager_ptr, self.home_id, self.node_id)
            }
        })*
    }
}

macro_rules! node_string_getters {
    ( $($name: ident, $impl_name: ident;)+ ) => {
        $(pub fn $name(&self) -> String {
            let manager_ptr = unsafe { extern_manager::get() };
            let result = unsafe {
                CString::from_raw(
                    extern_manager::$impl_name(manager_ptr, self.home_id, self.node_id, rust_string_creator)
                )
            };
            result.into_string().unwrap()
        })*
    }
}

impl Node {
    pub fn from_id(home_id: u32, node_id: u8) -> Node {
        Node { home_id: home_id, node_id: node_id }
    }

    node_getters! {
        is_listening_device, is_node_listening_device -> bool;
        is_frequent_listening_device, is_node_frequent_listening_device -> bool;
        is_beaming_device, is_node_beaming_device -> bool;
        is_routing_device, is_node_routing_device -> bool;
        is_security_device, is_node_security_device -> bool;
        get_max_baud_rate, get_node_max_baud_rate -> u32;
        get_version, get_node_version -> u8;
        get_security, get_node_security -> u8;
        is_zwave_plus, is_node_zwave_plus -> bool;
        get_basic, get_node_basic -> u8;
        get_generic, get_node_generic -> u8;
        get_specific, get_node_specific -> u8;
    }

    node_string_getters! {
        get_manufacturer_name, get_node_manufacturer_name;
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ home_id: {:?}, node_id: {:?}, \
                is_listening_device: {:?}, is_frequent_listening_device: {:?}, \
                is_beaming_device: {:?}, is_routing_device: {:?}, is_security_device: {:?}, \
                max_baud_rate: {:?}, version: {:?}, security: {:?}, is_zwave_plus: {:?}, \
                basic: {:?}, generic: {:?}, specific: {:?}, \
                manufacturer_name: {:?} }}",
               self.home_id,
               self.node_id,
               self.is_listening_device(),
               self.is_frequent_listening_device(),
               self.is_beaming_device(),
               self.is_routing_device(),
               self.is_security_device(),
               self.get_max_baud_rate(),
               self.get_version(),
               self.get_security(),
               self.is_zwave_plus(),
               self.get_basic(),
               self.get_generic(),
               self.get_specific(),
               self.get_manufacturer_name()
        )
    }
}
