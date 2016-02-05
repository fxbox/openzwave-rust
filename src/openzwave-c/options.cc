#include <string>
#include <openzwave/Options.h>
#include "options.h"

extern "C" {

OpenZWave::Options * options_create(char const *_configPath, char const *_userPath, char const *_commandLine) {
  std::string configPath(_configPath);
  std::string userPath(_userPath);
  std::string commandLine(_commandLine);

  return OpenZWave::Options::Create(configPath, userPath, commandLine);
}

}  // extern "C"
