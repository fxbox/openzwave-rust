use std::{ fmt, ptr };

use ffi::manager as extern_manager;
use ffi::utils::{ rust_string_creator, rust_vec_creator, recover_string, recover_vec };
use libc::c_char;
use controller::Controller;
use manager::NodeBasic;
use itertools::free::join;

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Clone, Copy)]
pub struct Node {
    home_id: u32,
    node_id: u8
}

// implements simple node getters
macro_rules! node_getters {
    ( $($impl_name: ident as $name: ident -> $t: ty),+ ) => {
        $(pub fn $name(&self) -> $t {
            let manager_ptr = unsafe { extern_manager::get() };
            unsafe {
                extern_manager::$impl_name(manager_ptr, self.home_id, self.node_id)
            }
        })*
    }
}

macro_rules! node_string_getters {
    ( $($impl_name: ident as $name: ident),+ ) => {
        $(pub fn $name(&self) -> String {
            let manager_ptr = unsafe { extern_manager::get() };
            let result = unsafe {
                extern_manager::$impl_name(manager_ptr, self.home_id, self.node_id, rust_string_creator)
            };
            recover_string(result)
        })*
    }
}

impl Node {
    pub fn from_id(home_id: u32, node_id: u8) -> Node {
        Node { home_id: home_id, node_id: node_id }
    }

    node_getters! {
        is_node_listening_device as is_listening_device -> bool,
        is_node_frequent_listening_device as is_frequent_listening_device -> bool,
        is_node_beaming_device as is_beaming_device -> bool,
        is_node_routing_device as is_routing_device -> bool,
        is_node_security_device as is_security_device -> bool,
        get_node_max_baud_rate as get_max_baud_rate -> u32,
        get_node_version as get_version -> u8,
        get_node_security as get_security -> u8,
        is_node_zwave_plus as is_zwave_plus -> bool,
        get_node_generic as get_generic -> u8,
        get_node_specific as get_specific -> u8,
        get_node_device_type as get_device_type -> u16,
        get_node_role as get_role -> u8,
        get_node_plus_type as get_plus_type -> u8,
        is_node_info_received as is_info_received -> bool,
        is_node_awake as is_awake -> bool,
        is_node_failed as is_failed -> bool
    }

    node_string_getters! {
        get_node_type as get_type,
        get_node_manufacturer_name as get_manufacturer_name,
        get_node_product_name as get_product_name,
        get_node_name as get_name,
        get_node_location as get_location,
        get_node_manufacturer_id as get_manufacturer_id,
        get_node_product_type as get_product_type,
        get_node_product_id as get_product_id,
        get_node_query_stage as get_query_stage,
        get_node_device_type_string as get_device_type_string,
        get_node_role_string as get_role_string,
        get_node_plus_type_string as get_plus_type_string
    }

    pub fn get_controller(&self) -> Controller {
        Controller::new(self.home_id)
    }

    pub fn get_home_id(&self) -> u32 {
        self.home_id
    }

    pub fn get_id(&self) -> u8 {
        self.node_id
    }

    pub fn get_basic(&self) -> Option<NodeBasic> {
        let manager_ptr = unsafe { extern_manager::get() };
        NodeBasic::from_u8(unsafe { extern_manager::get_node_basic(manager_ptr, self.home_id, self.node_id) })
    }

    pub fn get_neighbors(&self) -> Option<Vec<Node>> {
        let manager_ptr = unsafe { extern_manager::get() };
        let result_ptr = unsafe {
            extern_manager::get_node_neighbors(manager_ptr, self.home_id, self.node_id, rust_vec_creator::<u8>)
        } as *mut Vec<u8>;

        if result_ptr.is_null() {
            return None;
        }

        let vec_neighbors_id = recover_vec(result_ptr);
        let vec_neighbors = vec_neighbors_id.into_iter()
            .map(|id| Node { home_id: self.home_id, node_id: id })
            .collect();
        Some(vec_neighbors)
    }

    pub fn get_class_information(&self, command_class_id: u8) -> Option<(String, u8)> {
        let manager_ptr = unsafe { extern_manager::get() };
        let mut class_name: *mut c_char = ptr::null_mut();
        let mut class_version: u8 = 0;

        let has_class = unsafe {
            extern_manager::get_node_class_information(
                manager_ptr, self.home_id, self.node_id,
                command_class_id, &mut class_name, &mut class_version,
                rust_string_creator
            )
        };

        if !has_class {
            return None;
        }

        let class_name = recover_string(class_name);

        Some((class_name, class_version))
    }

    pub fn simple_debug(&self) -> String {
        format!("Node {{ home_id: {}, node_id: {} }}", self.home_id, self.node_id)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("{:3} {:17} {:30} {:50} {:30} {:30}",
                      self.get_id(),
                      self.get_basic().map_or(String::from("unknown"), |basic| basic.to_string()),
                      self.get_type(),
                      self.get_product_name(),
                      self.get_name(),
                      self.get_location()
                      )
              )
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ home_id: {:?}, node_id: {:?}, \
                is_listening_device: {:?}, is_frequent_listening_device: {:?}, \
                is_beaming_device: {:?}, is_routing_device: {:?}, is_security_device: {:?}, \
                max_baud_rate: {:?}, version: {:?}, security: {:?}, is_zwave_plus: {:?}, \
                basic: {:?}, generic: {:?}, specific: {:?}, \
                type: {:?}, manufacturer_name: {:?}, product_name: {:?}, \
                name: {:?}, location: {:?}, manufacturer_id: {:?}, product_type: {:?} \
                product_id: {:?}, query_stage: {:?}, \
                is_info_received: {:?}, is_awake: {:?}, is_failed: {:?}, \
                device_type: {:?}, device_type_string: {:?}, \
                role: {:?}, role_string: {:?}, \
                plus_type: {:?}, plus_type_string: {:?}, \
                neighbors: [{}] }}",
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
               self.get_type(),
               self.get_manufacturer_name(),
               self.get_product_name(),
               self.get_name(),
               self.get_location(),
               self.get_manufacturer_id(),
               self.get_product_type(),
               self.get_product_id(),
               self.get_query_stage(),
               self.is_info_received(), self.is_awake(), self.is_failed(),
               self.get_device_type(), self.get_device_type_string(),
               self.get_role(), self.get_role_string(),
               self.get_plus_type(), self.get_plus_type_string(),
               join(self.get_neighbors()
                        .unwrap_or(Vec::new())
                        .iter()
                        .map(|node: &Node| node.simple_debug()),
                    ", ")
        )
    }
}
