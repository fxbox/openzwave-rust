#include <openzwave/Notification.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef OpenZWave::Notification Notification;
typedef OpenZWave::ValueID ValueID;
typedef Notification::NotificationType NotificationType;
typedef Notification::NotificationCode NotificationCode;
typedef char const * (*RustStringCreator) (char const *);

NotificationType notification_get_type(const Notification *);

uint32 notification_get_home_id(const Notification *);

uint8 notification_get_node_id(const Notification *);

ValueID const * notification_get_value_id(const Notification *);

uint8 notification_get_group_idx(const Notification *);

uint8 notification_get_event(const Notification *);

uint8 notification_get_button_id(const Notification *);

uint8 notification_get_scene_id(const Notification *);

uint8 notification_get_notification(const Notification *);

uint8 notification_get_byte(const Notification *);

char const * notification_get_as_string(const Notification *, const RustStringCreator);

#ifdef __cplusplus
}  // extern "C"
#endif
