#include <openzwave/Manager.h>
#include <openzwave/Driver.h>

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

#ifdef __cplusplus
}  // extern "C"
#endif
