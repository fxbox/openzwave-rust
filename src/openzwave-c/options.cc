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

bool options_lock(Options * options) {
  return options->Lock();
}

bool options_destroy() {
  return Options::Destroy();
}

}  // extern "C"
