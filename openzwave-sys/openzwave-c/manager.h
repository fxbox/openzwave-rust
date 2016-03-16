#include <openzwave/Manager.h>
#include <openzwave/Driver.h>
#include "utils.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef OpenZWave::Manager Manager;
typedef OpenZWave::Driver Driver;
typedef OpenZWave::ValueID ValueID;

Manager * manager_create();
Manager * manager_get();
void manager_destroy();
bool manager_add_watcher(Manager * manager, Manager::pfnOnNotification_t _watcher, void* _context);
bool manager_remove_watcher(Manager * manager, Manager::pfnOnNotification_t _watcher, void* _context);
bool manager_add_driver(Manager * manager, const char * _controllerPath, const Driver::ControllerInterface * _interface);
bool manager_remove_driver(Manager * manager, const char * _controllerPath);
uint8 manager_get_controller_node_id(Manager * manager, const uint32 home_id);
uint8 manager_get_suc_node_id(Manager * manager, const uint32 home_id);
bool manager_is_primary_controller(Manager * manager, const uint32 home_id);
bool manager_is_bridge_controller(Manager * manager, const uint32 home_id);
uint32 manager_get_send_queue_count(Manager * manager, const uint32 home_id);
void manager_log_driver_statistics(Manager * manager, const uint32 home_id);
Driver::ControllerInterface manager_get_controller_interface_type(Manager * manager, const uint32 home_id);
char * manager_get_library_version(Manager * manager, const uint32 home_id, const RustStringCreator stringCreator);

char * manager_get_library_type_name(Manager * manager, const uint32 home_id, const RustStringCreator stringCreator);
char * manager_get_controller_path(Manager * manager, const uint32 home_id, const RustStringCreator stringCreator);

int32 manager_get_poll_interval(Manager * manager);
void manager_set_poll_interval(Manager * manager, int32 interval, bool between_poll);
bool manager_enable_poll_with_intensity(Manager * manager, const ValueID * value, uint8 intensity);
bool manager_enable_poll(Manager * manager, const ValueID * value);
bool manager_disable_poll(Manager * manager, const ValueID * value);
bool manager_is_polled(Manager * manager, const ValueID * value);
void manager_set_poll_intensity(Manager * manager, const ValueID * value, uint8 intensity);
uint8 manager_get_poll_intensity(Manager * manager, const ValueID * value);

char * manager_get_value_label(Manager * manager, const ValueID * id, const RustStringCreator stringCreator);
void manager_set_value_label(Manager * manager, const ValueID * id, char const * str);
char * manager_get_value_units(Manager * manager, const ValueID * id, const RustStringCreator stringCreator);
void manager_set_value_units(Manager * manager, const ValueID * id, char const * str);
char * manager_get_value_help(Manager * manager, const ValueID * id, const RustStringCreator stringCreator);
void manager_set_value_help(Manager * manager, const ValueID * id, char const * str);
int32 manager_get_value_min(Manager * manager, const ValueID * id);
int32 manager_get_value_max(Manager * manager, const ValueID * id);
bool manager_is_value_read_only(Manager * manager, const ValueID * id);
bool manager_is_value_write_only(Manager * manager, const ValueID * id);
bool manager_is_value_set(Manager * manager, const ValueID * id);
bool manager_is_value_polled(Manager * manager, const ValueID * id);

#define GET_VALUE_FUNC(name, ...) \
  bool manager_get_value_ ## name (Manager * manager, const ValueID * id, __VA_ARGS__)

GET_VALUE_FUNC(as_bool, bool*);
GET_VALUE_FUNC(as_byte, uint8*);
GET_VALUE_FUNC(as_float, float*);
GET_VALUE_FUNC(float_precision, uint8*);
GET_VALUE_FUNC(as_int, int32*);
GET_VALUE_FUNC(as_short, int16*);
GET_VALUE_FUNC(as_string, char**, const RustStringCreator);
GET_VALUE_FUNC(as_raw, void ** value, RustU8VecCreator);
GET_VALUE_FUNC(list_selection_as_string, char**, const RustStringCreator);
GET_VALUE_FUNC(list_selection_as_int, int32*);
GET_VALUE_FUNC(list_items, void ** value, const RustStringVecCreator);
GET_VALUE_FUNC(list_values, void ** value, const RustI32VecCreator);

#define GET_NODE_FUNC(name, return_type, ...) \
  return_type manager_node_ ## name (Manager * manager, uint32 home_id, uint8 node_id, __VA_ARGS__)
#define GET_NODE_FUNC_NO_ARGS(name, return_type) \
  return_type manager_node_ ## name (Manager * manager, uint32 home_id, uint8 node_id)

GET_NODE_FUNC_NO_ARGS(is_listening_device, bool);
GET_NODE_FUNC_NO_ARGS(is_frequent_listening_device, bool);
GET_NODE_FUNC_NO_ARGS(is_beaming_device, bool);
GET_NODE_FUNC_NO_ARGS(is_routing_device, bool);
GET_NODE_FUNC_NO_ARGS(is_security_device, bool);
GET_NODE_FUNC_NO_ARGS(get_max_baud_rate, uint32);
GET_NODE_FUNC_NO_ARGS(get_version, uint8);
GET_NODE_FUNC_NO_ARGS(get_security, uint8);
GET_NODE_FUNC_NO_ARGS(is_zwave_plus, bool);
GET_NODE_FUNC_NO_ARGS(get_basic, uint8);
GET_NODE_FUNC_NO_ARGS(get_generic, uint8);
GET_NODE_FUNC_NO_ARGS(get_specific, uint8);
GET_NODE_FUNC_NO_ARGS(is_info_received, bool);
GET_NODE_FUNC_NO_ARGS(is_awake, bool);
GET_NODE_FUNC_NO_ARGS(is_failed, bool);
GET_NODE_FUNC_NO_ARGS(get_device_type, uint16);
GET_NODE_FUNC_NO_ARGS(get_role, uint8);
GET_NODE_FUNC_NO_ARGS(get_plus_type, uint8);

#define GET_NODE_STRING_FUNC(name) \
  GET_NODE_FUNC(name, char *, const RustStringCreator stringCreator)

GET_NODE_STRING_FUNC(get_type);
GET_NODE_STRING_FUNC(get_manufacturer_name);
GET_NODE_STRING_FUNC(get_product_name);
GET_NODE_STRING_FUNC(get_name);
GET_NODE_STRING_FUNC(get_location);
GET_NODE_STRING_FUNC(get_manufacturer_id);
GET_NODE_STRING_FUNC(get_product_type);
GET_NODE_STRING_FUNC(get_product_id);
GET_NODE_STRING_FUNC(get_query_stage);
GET_NODE_STRING_FUNC(get_device_type_string);
GET_NODE_STRING_FUNC(get_role_string);
GET_NODE_STRING_FUNC(get_plus_type_string);

GET_NODE_FUNC(get_neighbors, void *, const RustU8VecCreator);
GET_NODE_FUNC(
    get_class_information, bool,
    uint8 const command_class_id, char** class_name, uint8* class_version,
    const RustStringCreator);

//TODO node neighbors
#ifdef __cplusplus
}  // extern "C"
#endif
