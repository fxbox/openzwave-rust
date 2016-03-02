use ffi::value_classes::value_id as extern_value_id;
use ffi::manager as extern_manager;
use libc::{ c_char, c_void };
use std::ffi::{ CString, NulError };
use std::ptr;
use std::fmt;

pub use ffi::value_classes::value_id::{ValueGenre, ValueType, ValueID as ExternValueID};

use ffi::utils::{ rust_string_creator, rust_vec_creator, rust_string_vec_creator, recover_string };

pub struct ValueList<'a> {
    value_id: &'a ValueID
}

impl<'a> ValueList<'a> {
    pub fn selection_as_string(&self) -> Result<String, &str> {
        let manager_ptr = unsafe { extern_manager::get() };
        let mut raw_string: *mut c_char = ptr::null_mut();

        let res = unsafe {
            extern_manager::get_value_list_selection_as_string(manager_ptr, self.value_id.ptr, &mut raw_string, rust_string_creator)
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
        let res = unsafe { extern_manager::get_value_list_selection_as_int(manager_ptr, self.value_id.ptr, &mut val) };
        if res { Ok(val) } else { Err("Could not get the value") }
    }

    pub fn items(&self) -> Result<Box<Vec<String>>, &str> {
        let manager_ptr = unsafe { extern_manager::get() };
        let mut c_items: *mut Vec<String> = ptr::null_mut();
        let c_items_void_ptr = &mut c_items as *mut *mut _ as *mut *mut c_void;
        let res = unsafe { extern_manager::get_value_list_items(manager_ptr, self.value_id.ptr, c_items_void_ptr, rust_string_vec_creator) };
        if res {
            let vec_c_items = unsafe { Box::from_raw(c_items) };
            Ok(vec_c_items)
        } else {
            Err("Could not get the value")
        }
    }

    pub fn values(&self) -> Result<Box<Vec<i32>>, &str> {
        let manager_ptr = unsafe { extern_manager::get() };
        let mut c_values: *mut Vec<i32> = ptr::null_mut();
        let c_values_void_ptr = &mut c_values as *mut *mut _ as *mut *mut c_void;
        let res = unsafe { extern_manager::get_value_list_values(manager_ptr, self.value_id.ptr, c_values_void_ptr, rust_vec_creator::<i32>) };
        if res {
            let vec_c_values = unsafe { Box::from_raw(c_values) };
            Ok(vec_c_values)
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

pub struct ValueID {
    // public because it's used in some methods in manager.
    pub ptr: *const ExternValueID
}

impl ValueID {
    pub fn new(ptr: *const extern_value_id::ValueID) -> ValueID {
        ValueID {
            ptr: ptr
        }
    }

    pub fn from_packed_id(home_id: u32, id: u64) -> ValueID {
        ValueID {
            ptr: unsafe { extern_value_id::value_id_from_packed_id(home_id, id) }
        }
    }

    pub fn from_values(home_id: u32,
                       node_id: u8,
                       genre: ValueGenre,
                       command_class_id: u8,
                       instance: u8,
                       value_index: u8,
                       value_type: ValueType) -> ValueID {
        ValueID {
            ptr: unsafe {
                extern_value_id::value_id_from_values(home_id,
                                                      node_id,
                                                      genre,
                                                      command_class_id,
                                                      instance,
                                                      value_index,
                                                      value_type)
            }
        }
    }

    // instance methods
    pub fn get_home_id(&self) -> u32 {
        unsafe { extern_value_id::value_id_get_home_id(self.ptr) }
    }

    pub fn get_node_id(&self) -> u8 {
        unsafe { extern_value_id::value_id_get_node_id(self.ptr) }
    }

    pub fn get_genre(&self) -> ValueGenre {
        unsafe { extern_value_id::value_id_get_genre(self.ptr) }
    }

    pub fn get_command_class_id(&self) -> u8 {
        unsafe { extern_value_id::value_id_get_command_class_id(self.ptr) }
    }

    pub fn get_instance(&self) -> u8 {
        unsafe { extern_value_id::value_id_get_instance(self.ptr) }
    }

    pub fn get_index(&self) -> u8 {
        unsafe { extern_value_id::value_id_get_index(self.ptr) }
    }

    pub fn get_type(&self) -> ValueType {
        unsafe { extern_value_id::value_id_get_type(self.ptr) }
    }

    pub fn get_id(&self) -> u64 {
        unsafe { extern_value_id::value_id_get_id(self.ptr) }
    }

    pub fn as_bool(&self) -> Result<bool, &str> {
        match self.get_type() {
            // The underlying library returns a value for both bool and button types.
            ValueType::ValueType_Bool | ValueType::ValueType_Button => {
                let manager_ptr = unsafe { extern_manager::get() };
                let mut val: bool = false;
                let res = unsafe { extern_manager::get_value_as_bool(manager_ptr, self.ptr, &mut val) };
                if res { Ok(val) } else { Err("Could not get the value") }
            },
            _ => Err("Wrong type")
        }
    }

    pub fn as_byte(&self) -> Result<u8, &str> {
        if self.get_type() == ValueType::ValueType_Byte {
            let manager_ptr = unsafe { extern_manager::get() };
            let mut val: u8 = 0;
            let res = unsafe { extern_manager::get_value_as_byte(manager_ptr, self.ptr, &mut val) };
            if res { Ok(val) } else { Err("Could not get the value") }
        } else {
            Err("Wrong type")
        }
    }

    pub fn as_float(&self) -> Result<f32, &str> {
        if self.get_type() == ValueType::ValueType_Decimal {
            let manager_ptr = unsafe { extern_manager::get() };
            let mut val: f32 = 0.;
            let res = unsafe { extern_manager::get_value_as_float(manager_ptr, self.ptr, &mut val) };
            if res { Ok(val) } else { Err("Could not get the value") }
        } else {
            Err("Wrong type")
        }
    }

    pub fn get_float_precision(&self) -> Result<u8, &str> {
        if self.get_type() == ValueType::ValueType_Decimal {
            let manager_ptr = unsafe { extern_manager::get() };
            let mut val: u8 = 0;
            let res = unsafe { extern_manager::get_value_float_precision(manager_ptr, self.ptr, &mut val) };
            if res { Ok(val) } else { Err("Could not get the value") }
        } else {
            Err("Wrong type")
        }
    }

    pub fn as_int(&self) -> Result<i32, &str> {
        if self.get_type() == ValueType::ValueType_Int {
            let manager_ptr = unsafe { extern_manager::get() };
            let mut val: i32 = 0;
            let res = unsafe { extern_manager::get_value_as_int(manager_ptr, self.ptr, &mut val) };
            if res { Ok(val) } else { Err("Could not get the value") }
        } else {
            Err("Wrong type")
        }
    }

    pub fn as_short(&self) -> Result<i16, &str> {
        if self.get_type() == ValueType::ValueType_Short {
            let manager_ptr = unsafe { extern_manager::get() };
            let mut val: i16 = 0;
            let res = unsafe { extern_manager::get_value_as_short(manager_ptr, self.ptr, &mut val) };
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
            extern_manager::get_value_as_string(manager_ptr, self.ptr, &mut raw_string, rust_string_creator)
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
            let res = unsafe { extern_manager::get_value_as_raw(manager_ptr, self.ptr, raw_ptr_c_void, rust_vec_creator::<u8>) };

            if res {
                let val = unsafe { Box::from_raw(raw_ptr) };
                Ok(val)
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
                extern_manager::get_value_label(manager_ptr, self.ptr, rust_string_creator)
            }
        )
    }

    pub fn set_label(&self, str: &str) -> Result<(), NulError> {
        unsafe {
            let manager_ptr = extern_manager::get();
            let c_string = try!(CString::new(str)).as_ptr();
            extern_manager::set_value_label(manager_ptr, self.ptr, c_string);
            Ok(())
        }
    }

    pub fn get_units(&self) -> String {
        recover_string(
            unsafe {
                let manager_ptr = extern_manager::get();
                extern_manager::get_value_units(manager_ptr, self.ptr, rust_string_creator)
            }
        )
    }

    pub fn set_units(&self, str: &str) -> Result<(), NulError>  {
        unsafe {
            let manager_ptr = extern_manager::get();
            let c_string = try!(CString::new(str)).as_ptr();
            extern_manager::set_value_units(manager_ptr, self.ptr, c_string);
            Ok(())
        }
    }

    pub fn get_help(&self) -> String {
        recover_string(
            unsafe {
                let manager_ptr = extern_manager::get();
                extern_manager::get_value_help(manager_ptr, self.ptr, rust_string_creator)
            }
        )
    }

    pub fn set_help(&self, str: &str) -> Result<(), NulError> {
        unsafe {
            let manager_ptr = extern_manager::get();
            let c_string = try!(CString::new(str)).as_ptr();
            extern_manager::set_value_help(manager_ptr, self.ptr, c_string);
            Ok(())
        }
    }

    pub fn get_min(&self) -> i32 {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::get_value_min(manager_ptr, self.ptr)
        }
    }

    pub fn get_max(&self) -> i32 {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::get_value_max(manager_ptr, self.ptr)
        }
    }

    pub fn is_read_only(&self) -> bool {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::is_value_read_only(manager_ptr, self.ptr)
        }
    }

    pub fn is_write_only(&self) -> bool {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::is_value_write_only(manager_ptr, self.ptr)
        }
    }

    pub fn is_set(&self) -> bool {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::is_value_set(manager_ptr, self.ptr)
        }
    }

    pub fn is_polled(&self) -> bool {
        unsafe {
            let manager_ptr = extern_manager::get();
            extern_manager::is_value_polled(manager_ptr, self.ptr)
        }
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

impl cmp::PartialEq for ValueID {
    fn eq(&self, other: &ValueID) -> bool {
        unsafe { extern_value_id::value_id_eq(self.ptr, other.ptr) }
    }
}

impl cmp::Eq for ValueID {}

impl cmp::PartialOrd for ValueID {
    fn partial_cmp(&self, other: &ValueID) -> Option<Ordering> {
        let is_less_than = unsafe { extern_value_id::value_id_less_than(self.ptr, other.ptr) };
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
