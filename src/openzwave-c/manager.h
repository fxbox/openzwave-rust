#include <openzwave/Manager.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef OpenZWave::Manager Manager;

Manager * manager_create();
Manager * manager_get();
void manager_destroy();
bool manager_add_watcher(Manager * manager, Manager::pfnOnNotification_t _watcher, void* _context);
bool manager_remove_watcher(Manager * manager, Manager::pfnOnNotification_t _watcher, void* _context);

#ifdef __cplusplus
}  // extern "C"
#endif
