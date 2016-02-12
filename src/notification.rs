#[link(name = "openzwave-c", kind = "static")]
mod extern_notification {
    use value_classes::value_id::ExternValueID;
    use libc::c_char;

    #[derive(Debug)]
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
        pub fn notification_get_value_id(notification: *const Notification) -> *const ExternValueID;
        pub fn notification_get_group_idx(notification: *const Notification) -> u8;
        pub fn notification_get_event(notification: *const Notification) -> u8;
        pub fn notification_get_button_id(notification: *const Notification) -> u8;
        pub fn notification_get_scene_id(notification: *const Notification) -> u8;
        pub fn notification_get_notification(notification: *const Notification) -> u8;
        pub fn notification_get_byte(notification: *const Notification) -> u8;
        pub fn notification_get_as_string(notification: *const Notification,
                                          rust_string_creator: extern fn(*const c_char) -> *const c_char) -> *const c_char;
    }
}

pub use self::extern_notification::{NotificationType, NotificationCode, Notification as ExternNotification};
use value_classes::value_id::ValueID;
use std::ffi::{CStr, CString};
use libc::c_char;

pub struct Notification {
    ptr: *const extern_notification::Notification
}

// This is used in Notification::get_as_string.
// The argument `data` is assumed to have a final \0, it's a valid C string.
// This function needs to allocate a Rust-owned memory space to copy this string too. Here we use
// to_string_lossy ant into_owned to ensure this. Then we use CString::into_raw to get a char*
// we'll pass back to the C function, that will return it. Then in our get_as_string wrapper we'll
// transform this back into a CString using from_raw.
extern "C" fn get_as_string_callback(data: *const c_char) -> *const c_char {
    let str = unsafe { CStr::from_ptr(data) }.to_string_lossy().into_owned();

    return CString::new(str).unwrap().into_raw();
}

impl Notification {
    pub fn new(ptr: *const extern_notification::Notification) -> Notification {
        Notification {
            ptr: ptr
        }
    }

    pub fn get_type(&self) -> NotificationType {
        unsafe { extern_notification::notification_get_type(self.ptr) }
    }

    pub fn get_home_id(&self) -> u32 {
        unsafe { extern_notification::notification_get_home_id(self.ptr) }
    }

    pub fn get_node_id(&self) -> u8 {
        unsafe { extern_notification::notification_get_node_id(self.ptr) }
    }

    pub fn get_value_id(&self) -> ValueID {
        ValueID::new(unsafe { extern_notification::notification_get_value_id(self.ptr) })
    }

    pub fn get_group_idx(&self) -> Option<u8> {
        match self.get_type() {
            NotificationType::Type_Group =>
                Some(unsafe { extern_notification::notification_get_group_idx(self.ptr) }),
            _ => None
        }
    }

    pub fn get_event(&self) -> Option<u8> {
        match self.get_type() {
            NotificationType::Type_NodeEvent | NotificationType::Type_ControllerCommand =>
                Some(unsafe { extern_notification::notification_get_event(self.ptr) }),
            _ => None
        }
    }

    pub fn get_button_id(&self) -> Option<u8> {
        match self.get_type() {
            NotificationType::Type_CreateButton | NotificationType::Type_DeleteButton |
            NotificationType::Type_ButtonOn | NotificationType::Type_ButtonOff =>
                Some(unsafe { extern_notification::notification_get_button_id(self.ptr) }),
            _ => None
        }
    }

    pub fn get_scene_id(&self) -> Option<u8> {
        match self.get_type() {
            NotificationType::Type_SceneEvent =>
                Some(unsafe { extern_notification::notification_get_scene_id(self.ptr) }),
            _ => None
        }
    }

    pub fn get_notification(&self) -> Option<u8> {
        match self.get_type() {
            NotificationType::Type_Notification | NotificationType::Type_ControllerCommand =>
                Some(unsafe { extern_notification:: notification_get_notification(self.ptr) }),
            _ => None
        }
    }

    pub fn get_byte(&self) -> u8 {
        unsafe { extern_notification::notification_get_byte(self.ptr) }
    }

    pub fn get_as_string(&self) -> String {
        unsafe {
            CString::from_raw(extern_notification::notification_get_as_string(self.ptr, get_as_string_callback) as *mut c_char)
        }.into_string().unwrap()
    }
}

use std::fmt;
impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_as_string())
    }
}

impl fmt::Debug for Notification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Notification {{ type: {:?}, home_id: {:?}, value_id: {:?}, group_idx: {:?}, event: {:?}, button_id: {:?}, scene_id: {:?}, notification: {:?}, byte: {:?}, as_string: {:?} }}",
               self.get_type(),
               self.get_home_id(),
               self.get_value_id(),
               self.get_group_idx(),
               self.get_event(),
               self.get_button_id(),
               self.get_scene_id(),
               self.get_notification(),
               self.get_byte(),
               self.get_as_string()
        )
    }
}
