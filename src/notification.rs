pub use ffi::notification::{NotificationType, NotificationCode, Notification as ExternNotification};
use ffi::notification as extern_notification;
use value_classes::value_id::ValueID;
use std::ffi::CString;
use libc::c_char;
use ffi::utils::rust_string_creator;

pub struct Notification {
    ptr: *const ExternNotification
}

impl Notification {
    pub fn new(ptr: *const ExternNotification) -> Notification {
        // Because the Notification object is not mutable, we might as well just fetch all
        // information right away and store it in a normal rust struct ?
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

    pub fn get_notification(&self) -> Option<NotificationCode> {
        let result = match self.get_type() {
            NotificationType::Type_Notification | NotificationType::Type_ControllerCommand =>
                Some(unsafe { extern_notification:: notification_get_notification(self.ptr) }),
            _ => None
        };

        result.and_then(|code| {
            match code {
                0 => Some(NotificationCode::Code_MsgComplete),
                1 => Some(NotificationCode::Code_Timeout),
                2 => Some(NotificationCode::Code_NoOperation),
                3 => Some(NotificationCode::Code_Awake),
                4 => Some(NotificationCode::Code_Sleep),
                5 => Some(NotificationCode::Code_Dead),
                6 => Some(NotificationCode::Code_Alive),
                _ => None
            }
        })
    }

    pub fn get_byte(&self) -> u8 {
        unsafe { extern_notification::notification_get_byte(self.ptr) }
    }

    pub fn get_as_string(&self) -> String {
        unsafe {
            CString::from_raw(extern_notification::notification_get_as_string(self.ptr, rust_string_creator) as *mut c_char)
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
        write!(f, "Notification {{ type: {:?}, home_id: {:?}, group_idx: {:?}, event: {:?}, button_id: {:?}, scene_id: {:?}, notification: {:?}, byte: {:?}, as_string: {:?}, \
                   value_id: {:?} }}",
               self.get_type(),
               self.get_home_id(),
               self.get_group_idx(),
               self.get_event(),
               self.get_button_id(),
               self.get_scene_id(),
               self.get_notification(),
               self.get_byte(),
               self.get_as_string(),
               self.get_value_id()
        )
    }
}
