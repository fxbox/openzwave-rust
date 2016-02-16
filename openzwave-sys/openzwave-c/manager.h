#include <openzwave/Manager.h>
#include <openzwave/Driver.h>
#include "utils.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef OpenZWave::Manager Manager;
typedef OpenZWave::Driver Driver;

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
char const * manager_get_library_version(Manager * manager, const uint32 home_id, const RustStringCreator stringCreator);

char const * manager_get_library_type_name(Manager * manager, const uint32 home_id, const RustStringCreator stringCreator);
char const * manager_get_controller_path(Manager * manager, const uint32 home_id, const RustStringCreator stringCreator);

#ifdef __cplusplus
}  // extern "C"
#endif
