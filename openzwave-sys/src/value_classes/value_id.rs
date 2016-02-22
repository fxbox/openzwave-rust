#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValueGenre {
    ValueGenre_Basic = 0,
    ValueGenre_User,
    ValueGenre_Config,
    ValueGenre_System,
    ValueGenre_Count
}

#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy)]
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

