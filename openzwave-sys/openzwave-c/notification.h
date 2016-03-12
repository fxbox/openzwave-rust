#include <openzwave/Notification.h>
#include "value_classes/value_id.h"
#include "utils.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef OpenZWave::Notification Notification;
typedef OpenZWave::ValueID ValueID;
typedef Notification::NotificationType NotificationType;
typedef Notification::NotificationCode NotificationCode;

NotificationType notification_get_type(const Notification *);

uint32 notification_get_home_id(const Notification *);

uint8 notification_get_node_id(const Notification *);

ValueID notification_get_value_id(const Notification *);

uint8 notification_get_group_idx(const Notification *);

uint8 notification_get_event(const Notification *);

uint8 notification_get_button_id(const Notification *);

uint8 notification_get_scene_id(const Notification *);

uint8 notification_get_notification(const Notification *);

uint8 notification_get_byte(const Notification *);

// RustStringCreator will be passed a NUL-ended C String so that it can
// allocates a Rust-owned buffer. This is made necessary by OpenZWave's
// Notification::GetAsString that returns a std::string by value and thus that
// will be deallocated after the end of the function. Another option would be to
// parse std::string directly in Rust, but this would be
// C++-implementation-dependent.
char const * notification_get_as_string(const Notification *, const RustStringCreator);

#ifdef __cplusplus
}  // extern "C"
#endif
