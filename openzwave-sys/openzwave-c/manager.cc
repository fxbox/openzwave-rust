#include <string>
#include <memory>
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

char * manager_get_library_version(Manager * manager, const uint32 home_id, const RustStringCreator stringCreator) {
  // We can't just return c_str() because the underlying buffer for "string"
  // would be deallocated after the end of this function. Thats why we have a
  // complex dance with the Rust function stringCreator.
  return stringCreator(manager->GetLibraryVersion(home_id).c_str()); // stringCreator expects a NUL-ended string.
}

char * manager_get_library_type_name(Manager * manager, const uint32 home_id, const RustStringCreator stringCreator) {
  return stringCreator(manager->GetLibraryTypeName(home_id).c_str());
}

char * manager_get_controller_path(Manager * manager, const uint32 home_id, const RustStringCreator stringCreator) {
  return stringCreator(manager->GetControllerPath(home_id).c_str());
}

int32 manager_get_poll_interval(Manager * manager) {
  return manager->GetPollInterval();
}

void manager_set_poll_interval(Manager * manager, int32 interval, bool between_poll) {
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

char * manager_get_value_label(Manager * manager, const ValueID * id, const RustStringCreator stringCreator) {
  return stringCreator(manager->GetValueLabel(*id).c_str());
}

void manager_set_value_label(Manager * manager, const ValueID * id, char const * str) {
  const std::string string(str);
  manager->SetValueLabel(*id, string);
}

char * manager_get_value_units(Manager * manager, const ValueID * id, const RustStringCreator stringCreator) {
  return stringCreator(manager->GetValueUnits(*id).c_str());
}

void manager_set_value_units(Manager * manager, const ValueID * id, char const * str) {
  const std::string string(str);
  manager->SetValueUnits(*id, string);
}

char * manager_get_value_help(Manager * manager, const ValueID * id, const RustStringCreator stringCreator) {
  return stringCreator(manager->GetValueHelp(*id).c_str());
}

void manager_set_value_help(Manager * manager, const ValueID * id, char const * str) {
  const std::string string(str);
  manager->SetValueHelp(*id, string);
}

int32 manager_get_value_min(Manager * manager, const ValueID * id) {
  return manager->GetValueMin(*id);
}

int32 manager_get_value_max(Manager * manager, const ValueID * id) {
  return manager->GetValueMax(*id);
}

bool manager_is_value_read_only(Manager * manager, const ValueID * id) {
  return manager->IsValueReadOnly(*id);
}

bool manager_is_value_write_only(Manager * manager, const ValueID * id) {
  return manager->IsValueWriteOnly(*id);
}

bool manager_is_value_set(Manager * manager, const ValueID * id) {
  return manager->IsValueSet(*id);
}

bool manager_is_value_polled(Manager * manager, const ValueID * id) {
  return manager->IsValuePolled(*id);
}

GET_VALUE_FUNC(as_bool, bool* value) {
  return manager->GetValueAsBool(*id, value);
}

GET_VALUE_FUNC(as_byte, uint8* value) {
  return manager->GetValueAsByte(*id, value);
}

GET_VALUE_FUNC(as_float, float* value) {
  return manager->GetValueAsFloat(*id, value);
}

GET_VALUE_FUNC(float_precision, uint8* value) {
  return manager->GetValueFloatPrecision(*id, value);
}

GET_VALUE_FUNC(as_int, int32* value) {
  return manager->GetValueAsInt(*id, value);
}

GET_VALUE_FUNC(as_short, int16* value) {
  return manager->GetValueAsShort(*id, value);
}

GET_VALUE_FUNC(as_string, char** value, const RustStringCreator stringCreator) {
  std::string result;
  bool res =  manager->GetValueAsString(*id, &result);
  if (res) {
    *value = stringCreator(result.c_str());
  }
  return res;
}

GET_VALUE_FUNC(as_raw, void ** rust_value, const RustU8VecCreator vecCreator) {
  uint8* value;
  uint8 length; // strangely GetValueAsRaw wants uint8
  bool res = manager->GetValueAsRaw(*id, &value, &length);
  if (res) {
    *rust_value = vecCreator(value, length);
  }
  return res;
}

GET_VALUE_FUNC(list_selection_as_string, char** value, const RustStringCreator stringCreator) {
  std::string result;
  bool res = manager->GetValueListSelection(*id, &result);
  if (res) {
    *value = stringCreator(result.c_str());
  }
  return res;
}

GET_VALUE_FUNC(list_selection_as_int, int32* value) {
  return manager->GetValueListSelection(*id, value);
}

GET_VALUE_FUNC(list_items, void ** rust_value, const RustStringVecCreator vecCreator) {
  std::vector<std::string> vec;

  bool res = manager->GetValueListItems(*id, &vec);

  if (res) {
    size_t length = vec.size();
    const std::unique_ptr<const char* []> value(new const char*[length]);
    size_t count = 0;
    for (const std::string &str : vec) {
      value[count++] = str.c_str();
    }
    *rust_value = vecCreator(value.get(), length);
  }

  return res;
}

GET_VALUE_FUNC(list_values, void ** rust_value, const RustI32VecCreator vecCreator) {
  std::vector<int32> vec;
  bool res = manager->GetValueListValues(*id, &vec);
  if (res) {
    *rust_value = vecCreator(vec.data(), vec.size());
  }
  return res;
}

#define GET_NODE_FUNC_IMPL(name, name_impl, type) \
  GET_NODE_FUNC(name, type) { \
    return manager->name_impl(home_id, node_id); \
  }

GET_NODE_FUNC_IMPL(is_listening_device, IsNodeListeningDevice, bool)
GET_NODE_FUNC_IMPL(is_frequent_listening_device, IsNodeFrequentListeningDevice, bool)
GET_NODE_FUNC_IMPL(is_beaming_device, IsNodeBeamingDevice, bool)
GET_NODE_FUNC_IMPL(is_routing_device, IsNodeRoutingDevice, bool)
GET_NODE_FUNC_IMPL(is_security_device, IsNodeSecurityDevice, bool)
GET_NODE_FUNC_IMPL(get_max_baud_rate, GetNodeMaxBaudRate, uint32)
GET_NODE_FUNC_IMPL(get_version, GetNodeVersion, uint8)
GET_NODE_FUNC_IMPL(get_security, GetNodeSecurity, uint8)
GET_NODE_FUNC_IMPL(is_zwave_plus, IsNodeZWavePlus, bool)
GET_NODE_FUNC_IMPL(get_basic, GetNodeBasic, uint8)
GET_NODE_FUNC_IMPL(get_generic, GetNodeGeneric, uint8)
GET_NODE_FUNC_IMPL(get_specific, GetNodeSpecific, uint8)

#define GET_NODE_STRING_FUNC_IMPL(name, name_impl) \
  GET_NODE_STRING_FUNC(name) { \
    return stringCreator(manager->name_impl(home_id, node_id).c_str()); \
  }

GET_NODE_STRING_FUNC_IMPL(get_manufacturer_name, GetNodeManufacturerName)

} /* extern "C" */
