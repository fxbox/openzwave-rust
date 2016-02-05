#include <openzwave/Options.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Options Options;

OpenZWave::Options * options_create(char const *configPath, char const *userPath, char const *commandLine);
OpenZWave::Options * options_get();
bool options_lock(OpenZWave::Options * option);
bool options_destroy();

#ifdef __cplusplus
}  // extern "C"
#endif
