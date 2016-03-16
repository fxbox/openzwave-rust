#include <openzwave/Notification.h>
#include "notification.h"

extern "C" {

  NotificationType notification_get_type(const Notification * notification) {
    return notification->GetType();
  }

  uint32 notification_get_home_id(const Notification * notification) {
    return notification->GetHomeId();
  }

  uint8 notification_get_node_id(const Notification * notification) {
    return notification->GetNodeId();
  }

  ValueID notification_get_value_id(const Notification * notification) {
    return notification->GetValueID();
  }

  uint8 notification_get_group_idx(const Notification * notification) {
    return notification->GetGroupIdx();
  }

  uint8 notification_get_event(const Notification * notification) {
    return notification->GetEvent();
  }

  uint8 notification_get_button_id(const Notification * notification) {
    return notification->GetButtonId();
  }

  uint8 notification_get_scene_id(const Notification * notification) {
    return notification->GetSceneId();
  }

  uint8 notification_get_notification(const Notification * notification) {
    return notification->GetNotification();
  }

  uint8 notification_get_byte(const Notification * notification) {
    return notification->GetByte();
  }

  char const * notification_get_as_string(const Notification * notification, const RustStringCreator stringCreator) {
    // We can't just return c_str() because the underlying buffer for "string"
    // would be deallocated after the end of this function. Thats why we have a
    // complex dance with the Rust function stringCreator.
    return stringCreator(notification->GetAsString().c_str()); // stringCreator expects a NUL-ended string.
  }

}  // extern "C"
