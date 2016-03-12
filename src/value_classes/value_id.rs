use ffi::value_classes::value_id as extern_value_id;
use ffi::manager as extern_manager;
use libc::{ c_char, c_void };
use std::ffi::{ CString, NulError };
use std::ptr;
use std::fmt;

pub use ffi::value_classes::value_id::{ValueGenre, ValueType};

use ffi::utils::{
    rust_string_creator, rust_vec_creator, rust_string_vec_creator,
    recover_string, recover_vec
};
use node::Node;
use controller::Controller;

pub struct ValueList<'a> {
    value_id: &'a ValueID
}

impl<'a> ValueList<'a> {
    pub fn selection_as_string(&self) -> Result<String, &str> {
        let manager_ptr = unsafe { extern_manager::get() };
        let mut raw_string: *mut c_char = ptr::null_mut();

        let res = unsafe {
            extern_manager::get_value_list_selection_as_string(manager_ptr, &self.value_id.as_ozw_vid(), &mut raw_string, rust_string_creator)
        };

        if res {
            Ok(recover_string(raw_string))
        } else {
            Err("Could not get the value")
        }
    }

    pub fn selection_as_int(&self) -> Result<i32, &str> {
        let manager_ptr = unsafe { extern_manager::get() };
        let mut val: i32 = 0;
        let res = unsafe { extern_manager::get_value_list_selection_as_int(manager_ptr, &self.value_id.as_ozw_vid(), &mut val) };
        if res { Ok(val) } else { Err("Could not get the value") }
    }

    pub fn items(&self) -> Result<Box<Vec<String>>, &str> {
        let manager_ptr = unsafe { extern_manager::get() };
        let mut c_items: *mut Vec<String> = ptr::null_mut();
        let c_items_void_ptr = &mut c_items as *mut *mut _ as *mut *mut c_void;
        let res = unsafe { extern_manager::get_value_list_items(manager_ptr, &self.value_id.as_ozw_vid(), c_items_void_ptr, rust_string_vec_creator) };
        if res {
            Ok(recover_vec(c_items))
        } else {
            Err("Could not get the value")
        }
    }

    pub fn values(&self) -> Result<Box<Vec<i32>>, &str> {
        let manager_ptr = unsafe { extern_manager::get() };
        let mut c_values: *mut Vec<i32> = ptr::null_mut();
        let c_values_void_ptr = &mut c_values as *mut *mut _ as *mut *mut c_void;
        let res = unsafe { extern_manager::get_value_list_values(manager_ptr, &self.value_id.as_ozw_vid(), c_values_void_ptr, rust_vec_creator::<i32>) };
        if res {
            Ok(recover_vec(c_values))
        } else {
            Err("Could not get the value")
        }
    }
}

impl<'a> fmt::Debug for ValueList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValueList {{ selection_as_string: {:?}, selection_as_int: {:?}, items: {:?}, values: {:?} }}",
               self.selection_as_string().ok(),
               self.selection_as_int().ok(),
               self.items().ok(),
               self.values().ok()
        )
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub struct ValueID {
    home_id: u32,
    id: u64
}

impl ValueID {
    pub fn from_packed_id(home_id: u32, id: u64) -> ValueID {
        ValueID { home_id: home_id, id: id }
    }

    pub fn as_ozw_vid(&self) -> extern_value_id::ValueID {
        unsafe { extern_value_id::value_id_from_packed_id(self.home_id, self.id) }
    }

    pub fn from_values(home_id: u32,
                       node_id: u8,
                       genre: ValueGenre,
                       command_class_id: u8,
                       instance: u8,
                       value_index: u8,
                       value_type: ValueType) -> ValueID {
        let ozw_vid = unsafe {
            extern_value_id::value_id_from_values(home_id,
                                                  node_id,
                                                  genre,
                                                  command_class_id,
                                                  instance,
                                                  value_index,
                                                  value_type)
        };
        ValueID {
            home_id: unsafe { extern_value_id::value_id_get_home_id(&ozw_vid) },
            id: unsafe { extern_value_id::value_id_get_id(&ozw_vid) }
        }
    }

    // instance methods
    pub fn get_controller(&self) -> Controller {
        Controller::new(self.home_id)
    }

    pub fn get_node(&self) -> Node {
        Node::from_id(self.home_id, self.get_node_id())
    }

    pub fn get_home_id(&self) -> u32 {
        self.home_id
    }

    pub fn get_node_id(&self) -> u8 {
        unsafe { extern_value_id::value_id_get_node_id(&self.as_ozw_vid()) }
    }

    pub fn get_genre(&self) -> ValueGenre {
        unsafe { extern_value_id::value_id_get_genre(&self.as_ozw_vid()) }
    }

    pub fn get_command_class_id(&self) -> u8 {
        unsafe { extern_value_id::value_id_get_command_class_id(&self.as_ozw_vid()) }
    }

    pub fn get_instance(&self) -> u8 {
        unsafe { extern_value_id::value_id_get_instance(&self.as_ozw_vid()) }
    }

    pub fn get_index(&self) -> u8 {
        unsafe { extern_value_id::value_id_get_index(&self.as_ozw_vid()) }
    }

    pub fn get_type(&self) -> ValueType {
        unsafe { extern_value_id::value_id_get_type(&self.as_ozw_vid()) }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn as_bool(&self) -> Result<bool, &str> {
        match self.get_type() {
            // The underlying library returns a value for both bool and button types.
            ValueType::ValueType_Bool | ValueType::ValueType_Button => {
                let manager_ptr = unsafe { extern_manager::get() };
                let mut val: bool = false;
                let res = unsafe { extern_manager::get_value_as_bool(manager_ptr, &self.as_ozw_vid(), &mut val) };
                if res { Ok(val) } else { Err("Could not get the value") }
            },
            _ => Err("Wrong type")
        }
    }

    pub fn as_byte(&self) -> Result<u8, &str> {
        if self.get_type() == ValueType::ValueType_Byte {
            let manager_ptr = unsafe { extern_manager::get() };
            let mut val: u8 = 0;
            let res = unsafe { extern_manager::get_value_as_byte(manager_ptr, &self.as_ozw_vid(), &mut val) };
            if res { Ok(val) } else { Err("Could not get the value") }
        } else {
            Err("Wrong type")
        }
    }

    pub fn as_float(&self) -> Result<f32, &str> {
        if self.get_type() == ValueType::ValueType_Decimal {
            let manager_ptr = unsafe { extern_manager::get() };
            let mut val: f32 = 0.;
            let res = unsafe { extern_manager::get_value_as_float(manager_ptr, &self.as_ozw_vid(), &mut val) };
            if res { Ok(val) } else { Err("Could not get the value") }
        } else {
            Err("Wrong type")
        }
    }

    pub fn get_float_precision(&self) -> Result<u8, &str> {
        if self.get_type() == ValueType::ValueType_Decimal {
            let manager_ptr = unsafe { extern_manager::get() };
            let mut val: u8 = 0;
            let res = unsafe { extern_manager::get_value_float_precision(manager_ptr, &self.as_ozw_vid(), &mut val) };
            if res { Ok(val) } else { Err("Could not get the value") }
        } else {
            Err("Wrong type")
        }
    }

    pub fn as_int(&self) -> Result<i32, &str> {
        if self.get_type() == ValueType::ValueType_Int {
            let manager_ptr = unsafe { extern_manager::get() };
            let mut val: i32 = 0;
            let res = unsafe { extern_manager::get_value_as_int(manager_ptr, &self.as_ozw_vid(), &mut val) };
            if res { Ok(val) } else { Err("Could not get the value") }
        } else {
            Err("Wrong type")
        }
    }

    pub fn as_short(&self) -> Result<i16, &str> {
        if self.get_type() == ValueType::ValueType_Short {
            let manager_ptr = unsafe { extern_manager::get() };
            let mut val: i16 = 0;
            let res = unsafe { extern_manager::get_value_as_short(manager_ptr, &self.as_ozw_vid(), &mut val) };
            if res { Ok(val) } else { Err("Could not get the value") }
        } else {
            Err("Wrong type")
        }
    }

    pub fn as_string(&self) -> Result<String, &str> {
        // The underlying C++ lib returns a value for any type.
        let manager_ptr = unsafe { extern_manager::get() };
        let mut raw_string: *mut c_char = ptr::null_mut();

        let res = unsafe {
            extern_manager::get_value_as_string(manager_ptr, &self.as_ozw_vid(), &mut raw_string, rust_string_creator)
        };

        if res {
            Ok(recover_string(raw_string))
        } else {
            Err("Could not get the value")
        }
    }

    pub fn as_raw(&self) -> Result<Box<Vec<u8>>, &str> {
        if self.get_type() == ValueType::ValueType_Raw {
            let mut raw_ptr: *mut Vec<u8> = ptr::null_mut();
            let raw_ptr_c_void = &mut raw_ptr as *mut *mut _ as *mut *mut c_void;

            let manager_ptr = unsafe { extern_manager::get() };
            let res = unsafe { extern_manager::get_value_as_raw(manager_ptr, &self.as_ozw_vid(), raw_ptr_c_void, rust_vec_creator::<u8>) };

            if res {
                Ok(recover_vec(raw_ptr))
            } else {
                Err("Could not get the value")
            }
        } else {
            Err("Wrong type")
        }
    }

    pub fn as_list(&self) -> Result<ValueList, &str> {
        if self.get_type() == ValueType::ValueType_List {
            Ok(ValueList { value_id: self })
        } else {
            Err("Wrong type")
        }
    }

    pub fn get_label(&self) -> String {
        recover_string(
            unsafe {
                let manager_ptr = extern_manager::get();
                extern_manager::get_value_label(manager_ptr, &self.as_ozw_vid(), rust_string_creator)
            }
        )
    }

    pub fn set_label(&self, str: &str) -> Result<(), NulError> {
        unsafe {
            let manager_ptr = extern_manager::get();
            let c_string = try!(CString::new(str)).as_ptr();
            extern_manager::set_value_label(manager_ptr, &self.as_ozw_vid(), c_string);
            Ok(())
        }
    }

    pub fn get_units(&self) -> String {
        recover_string(
            unsafe {
                let manager_ptr = extern_manager::get();
                extern_manager::get_value_units(manager_ptr, &self.as_ozw_vid(), rust_string_creator)
            }
        )
    }

    pub fn set_units(&self, str: &str) -> Result<(), NulError>  {
        unsafe {
            let manager_ptr = extern_manager::get();
            let c_string = try!(CString::new(str)).as_ptr();
            extern_manager::set_value_units(manager_ptr, &self.as_ozw_vid(), c_string);
            Ok(())
        }
    }

    pub fn get_help(&self) -> String {
        recover_string(
            unsafe {
                let manager_ptr = extern_manager::get();
                extern_manager::get_value_help(manager_ptr, &self.as_ozw_vid(), rust_string_creator)
            }
        )
    }

    pub fn set_help(&self, str: &str) -> Result<(), NulError> {
        unsafe {
            let manager_ptr = extern_manager::get();
            let c_string = try!(CString::new(str)).as_ptr();
            extern_manager::set_value_help(manager_ptr, &self.as_ozw_vid(), c_string);
            Ok(())
        }
    }

    pub fn get_min(&self) -> i32 {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::get_value_min(manager_ptr, &self.as_ozw_vid())
        }
    }

    pub fn get_max(&self) -> i32 {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::get_value_max(manager_ptr, &self.as_ozw_vid())
        }
    }

    pub fn is_read_only(&self) -> bool {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::is_value_read_only(manager_ptr, &self.as_ozw_vid())
        }
    }

    pub fn is_write_only(&self) -> bool {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::is_value_write_only(manager_ptr, &self.as_ozw_vid())
        }
    }

    pub fn is_set(&self) -> bool {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::is_value_set(manager_ptr, &self.as_ozw_vid())
        }
    }

    pub fn is_polled(&self) -> bool {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::is_value_polled(manager_ptr, &self.as_ozw_vid())
        }
    }
}

impl fmt::Display for ValueID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let node = Node::from_id(self.get_home_id(), self.get_node_id());
        f.pad(&format!("HomeId: {:08x} ID: {:016x} NodeId: {:3} {:20} CC: {:3} Type: {:8} Label: {:20} Value: {}",
                       self.get_home_id(),
                       self.get_id(),
                       self.get_node_id(),
                       node.get_name(),
                       self.get_command_class_id(),
                       self.get_type(),
                       self.get_label(),
                       self.as_string().unwrap_or(String::from("???"))))
    }
}

impl fmt::Debug for ValueID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValueID {{ home_id: {:?}, node_id: {:?}, genre: {:?}, command_class_id: {:?}, \
                   instance: {:?}, index: {:?}, type: {:?}, id: {:?}, \
                   label: {:?}, units: {:?}, help: {:?}, min: {:?}, max: {:?}, is_read_only: {:?}, \
                   is_write_only: {:?}, is_set: {:?}, is_polled: {:?}, \
                   as_bool: {:?}, as_byte: {:?}, \
                   as_float: {:?} (precision: {:?}), as_int: {:?}, as_short: {:?}, as_string: {:?}, as_raw: {:?}, \
                   list: {:?} \
                   }}",
               self.get_home_id(),
               self.get_node_id(),
               self.get_genre(),
               self.get_command_class_id(),
               self.get_instance(),
               self.get_index(),
               self.get_type(),
               self.get_id(),
               self.get_label(),
               self.get_units(),
               self.get_help(),
               self.get_min(),
               self.get_max(),
               self.is_read_only(),
               self.is_write_only(),
               self.is_set(),
               self.is_polled(),
               self.as_bool().ok(),
               self.as_byte().ok(),
               self.as_float().ok(),
               self.get_float_precision().ok(),
               self.as_int().ok(),
               self.as_short().ok(),
               self.as_string().ok(),
               self.as_raw().ok(),
               self.as_list().ok()
        )
    }
}

use std::cmp::{self, Ordering};

impl cmp::PartialOrd for ValueID {
    fn partial_cmp(&self, other: &ValueID) -> Option<Ordering> {
        let is_less_than = unsafe { extern_value_id::value_id_less_than(&self.as_ozw_vid(), &other.as_ozw_vid()) };
        if is_less_than {
            Some(Ordering::Less)
        } else if self.eq(other) {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl cmp::Ord for ValueID {
    fn cmp(&self, other: &ValueID) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
