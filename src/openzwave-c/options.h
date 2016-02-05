#include <openzwave/Options.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct Options Options;

OpenZWave::Options * options_create(char const *configPath, char const *userPath, char const *commandLine);

#ifdef __cplusplus
}  // extern "C"
#endif
