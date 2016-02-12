#[link(name = "openzwave-c", kind = "static")]
mod extern_value_id {
    #[repr(C)]
    #[derive(Debug)]
    pub enum ValueGenre {
        ValueGenre_Basic = 0,
        ValueGenre_User,
        ValueGenre_Config,
        ValueGenre_System,
        ValueGenre_Count
    }

    #[repr(C)]
    #[derive(Debug)]
    pub enum ValueType {
        ValueType_Bool = 0,
        ValueType_Byte,
        ValueType_Decimal,
        ValueType_Int,
        ValueType_List,
        ValueType_Schedule,
        ValueType_Short,
        ValueType_String,
        ValueType_Button,
        ValueType_Raw,
        //ValueType_Max = ValueType_Raw // likely useless in Rust wrapper
    }

    pub enum ValueID {}

    extern {
        pub fn value_id_from_packed_id(home_id: u32, id: u64) -> *const ValueID;
        pub fn value_id_from_values(home_id: u32,
                                    node_id: u8,
                                    genre: ValueGenre,
                                    command_class_id: u8,
                                    instance: u8,
                                    value_index: u8,
                                    value_type: ValueType) -> *const ValueID;

        pub fn value_id_get_home_id(value: *const ValueID) -> u32;
        pub fn value_id_get_node_id(value: *const ValueID) -> u8;
        pub fn value_id_get_genre(value: *const ValueID) -> ValueGenre;
        pub fn value_id_get_command_class_id(value: *const ValueID) -> u8;
        pub fn value_id_get_instance(value: *const ValueID) -> u8;
        pub fn value_id_get_index(value: *const ValueID) -> u8;
        pub fn value_id_get_type(value: *const ValueID) -> ValueType;
        pub fn value_id_get_id(value: *const ValueID) -> u64;

        // Comparison Operators
        pub fn value_id_eq(myself: *const ValueID, other: *const ValueID) -> bool;
        pub fn value_id_less_than(myself: *const ValueID, other: *const ValueID) -> bool;
    }
}

pub use self::extern_value_id::{ValueGenre, ValueType, ValueID as ExternValueID};

pub struct ValueID {
    ptr: *const ExternValueID
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
}

use std::fmt;
impl fmt::Debug for ValueID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ValueID {{ home_id: {:?}, node_id: {:?}, genre: {:?}, command_class_id: {:?}, instance: {:?}, index: {:?}, type: {:?}, id: {:?} }}",
               self.get_home_id(),
               self.get_node_id(),
               self.get_genre(),
               self.get_command_class_id(),
               self.get_instance(),
               self.get_index(),
               self.get_type(),
               self.get_id()
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
