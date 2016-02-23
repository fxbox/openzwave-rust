use ffi::value_classes::value_id as extern_value_id;
use ffi::manager as extern_manager;
use libc::c_char;
use std::ffi::CString;
use std::ptr;

pub use ffi::value_classes::value_id::{ValueGenre, ValueType, ValueID as ExternValueID};

use utils::get_string_callback;

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
            extern_manager::get_value_as_string(manager_ptr, self.ptr, &mut raw_string, get_string_callback)
        };

        if res {
            let val = unsafe { CString::from_raw(raw_string) };
            Ok(val.into_string().unwrap())
        } else {
            Err("Could not get the value")
        }
    }

    pub fn as_raw(&self) -> Result<Vec<u8>, &str> {
        if self.get_type() == ValueType::ValueType_Raw {
            let mut length: u8 = 0;
            let mut raw_ptr: *mut u8 = ptr::null_mut();

            let manager_ptr = unsafe { extern_manager::get() };
            let res = unsafe { extern_manager::get_value_as_raw(manager_ptr, self.ptr, &mut raw_ptr, &mut length) };

            if res {
                let val = unsafe { Vec::from_raw_parts(raw_ptr, length as usize, length as usize) };
                Ok(val)
            } else {
                Err("Could not get the value")
            }
        } else {
            Err("Wrong type")
        }
    }
}

use std::fmt;
impl fmt::Debug for ValueID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValueID {{ home_id: {:?}, node_id: {:?}, genre: {:?}, command_class_id: {:?}, instance: {:?}, index: {:?}, type: {:?}, id: {:?}, as_bool: {:?}, as_byte: {:?}, as_float: {:?}, as_int: {:?}, as_short: {:?}, as_string: {:?}, as_raw: {:?} }}",
               self.get_home_id(),
               self.get_node_id(),
               self.get_genre(),
               self.get_command_class_id(),
               self.get_instance(),
               self.get_index(),
               self.get_type(),
               self.get_id(),
               self.as_bool().ok(),
               self.as_byte().ok(),
               self.as_float().ok(),
               self.as_int().ok(),
               self.as_short().ok(),
               self.as_string().ok(),
               self.as_raw().ok()
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
