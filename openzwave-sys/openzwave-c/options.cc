#include <string>
#include <openzwave/Options.h>
#include "options.h"

extern "C" {

Options * options_create(char const *_configPath, char const *_userPath, char const *_commandLine) {
  std::string configPath(_configPath);
  std::string userPath(_userPath);
  std::string commandLine(_commandLine);

  return Options::Create(configPath, userPath, commandLine);
}

Options * options_get() {
  return Options::Get();
}

bool options_add_option_bool(Options * options, const char *_name, bool _value) {
  std::string sName(_name);
  return options->AddOptionBool(sName, _value);
}

bool options_add_option_int(Options * options, const char *_name, int32 _value) {
  std::string sName(_name);
  return options->AddOptionInt(sName, _value);
}

bool options_add_option_string(Options * options, const char *_name, const char *_value, bool _append) {
  std::string sName(_name);
  std::string sValue(_value);
  return options->AddOptionString(sName, sValue, _append);
}

bool options_lock(Options * options) {
  return options->Lock();
}

bool options_destroy() {
  return Options::Destroy();
}

}  // extern "C"
