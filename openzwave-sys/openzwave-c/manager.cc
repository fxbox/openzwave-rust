#include <string>
#include "manager.h"

extern "C" {

Manager * manager_create() {
  return Manager::Create();
}

Manager * manager_get() {
  return Manager::Get();
}

void manager_destroy() {
  Manager::Destroy();
}

bool manager_add_watcher(Manager * manager, Manager::pfnOnNotification_t _watcher, void* _context) {
  return manager->AddWatcher(_watcher, _context);
}

bool manager_remove_watcher(Manager * manager, Manager::pfnOnNotification_t _watcher, void* _context) {
  return manager->RemoveWatcher(_watcher, _context);
}

bool manager_add_driver(Manager * manager, const char * _controllerPath, const Driver::ControllerInterface * _interface) {
  const std::string controllerPath(_controllerPath);
  return manager->AddDriver(controllerPath, *_interface);
}

bool manager_remove_driver(Manager * manager, const char * _controllerPath) {
  const std::string controllerPath(_controllerPath);
  return manager->RemoveDriver(controllerPath);
}

uint8 manager_get_controller_node_id(Manager * manager, const uint32 home_id) {
  return manager->GetControllerNodeId(home_id);
}

uint8 manager_get_suc_node_id(Manager * manager, const uint32 home_id) {
  return manager->GetSUCNodeId(home_id);
}

bool manager_is_primary_controller(Manager * manager, const uint32 home_id) {
  return manager->IsPrimaryController(home_id);
}

bool manager_is_bridge_controller(Manager * manager, const uint32 home_id) {
  return manager->IsBridgeController(home_id);
}

uint32 manager_get_send_queue_count(Manager * manager, const uint32 home_id) {
  return manager->GetSendQueueCount(home_id);
}

void manager_log_driver_statistics(Manager * manager, const uint32 home_id) {
  manager->LogDriverStatistics(home_id);
}

Driver::ControllerInterface manager_get_controller_interface_type(Manager * manager, const uint32 home_id) {
  return manager->GetControllerInterfaceType(home_id);
}

char const * manager_get_library_version(Manager * manager, const uint32 home_id, const RustStringCreator stringCreator) {
  // We can't just return c_str() because the underlying buffer for "string"
  // would be deallocated after the end of this function. Thats why we have a
  // complex dance with the Rust function stringCreator.
  return stringCreator(manager->GetLibraryVersion(home_id).c_str()); // stringCreator expects a NUL-ended string.
}

char const * manager_get_library_type_name(Manager * manager, const uint32 home_id, const RustStringCreator stringCreator) {
  return stringCreator(manager->GetLibraryTypeName(home_id).c_str());
}

char const * manager_get_controller_path(Manager * manager, const uint32 home_id, const RustStringCreator stringCreator) {
  return stringCreator(manager->GetControllerPath(home_id).c_str());
}

uint32 manager_get_poll_interval(Manager * manager) {
  return manager->GetPollInterval();
}

void manager_set_poll_interval(Manager * manager, uint32 interval, bool between_poll) {
  manager->SetPollInterval(interval, between_poll);
}

bool manager_enable_poll_with_intensity(Manager * manager, const ValueID * value, uint8 intensity) {
  return manager->EnablePoll(*value, intensity);
}

bool manager_enable_poll(Manager * manager, const ValueID * value) {
  return manager->EnablePoll(*value);
}

bool manager_disable_poll(Manager * manager, const ValueID * value) {
  return manager->DisablePoll(*value);
}

bool manager_is_polled(Manager * manager, const ValueID * value) {
  return manager->isPolled(*value);
}

void manager_set_poll_intensity(Manager * manager, const ValueID * value, uint8 intensity) {
  manager->SetPollIntensity(*value, intensity);
}

uint8 manager_get_poll_intensity(Manager * manager, const ValueID * value) {
  return manager->GetPollIntensity(*value);
}

} /* extern "C" */
