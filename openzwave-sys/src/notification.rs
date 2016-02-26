use value_classes::value_id::ValueID;
use libc::c_char;
use utils::RustStringCreator;

#[derive(Debug)]
#[repr(C)]
pub enum NotificationType {
    Type_ValueAdded = 0,
    Type_ValueRemoved,
    Type_ValueChanged,
    Type_ValueRefreshed,
    Type_Group,
    Type_NodeNew,
    Type_NodeAdded,
    Type_NodeRemoved,
    Type_NodeProtocolInfo,
    Type_NodeNaming,
    Type_NodeEvent,
    Type_PollingDisabled,
    Type_PollingEnabled,
    Type_SceneEvent,
    Type_CreateButton,
    Type_DeleteButton,
    Type_ButtonOn,
    Type_ButtonOff,
    Type_DriverReady,
    Type_DriverFailed,
    Type_DriverReset,
    Type_EssentialNodeQueriesComplete,
    Type_NodeQueriesComplete,
    Type_AwakeNodesQueried,
    Type_AllNodesQueriedSomeDead,
    Type_AllNodesQueried,
    Type_Notification,
    Type_DriverRemoved,
    Type_ControllerCommand,
    Type_NodeReset
}

#[derive(Debug)]
#[repr(C)]
pub enum NotificationCode {
    Code_MsgComplete = 0,
    Code_Timeout,
    Code_NoOperation,
    Code_Awake,
    Code_Sleep,
    Code_Dead,
    Code_Alive
}

pub enum Notification {}
extern {
    pub fn notification_get_type(notification: *const Notification) -> NotificationType;
    pub fn notification_get_home_id(notification: *const Notification) -> u32;
    pub fn notification_get_node_id(notification: *const Notification) -> u8;
    pub fn notification_get_value_id(notification: *const Notification) -> *const ValueID;
    pub fn notification_get_group_idx(notification: *const Notification) -> u8;
    pub fn notification_get_event(notification: *const Notification) -> u8;
    pub fn notification_get_button_id(notification: *const Notification) -> u8;
    pub fn notification_get_scene_id(notification: *const Notification) -> u8;
    pub fn notification_get_notification(notification: *const Notification) -> u8;
    pub fn notification_get_byte(notification: *const Notification) -> u8;
    pub fn notification_get_as_string(notification: *const Notification, rust_string_creator: RustStringCreator) -> *const c_char;
}

