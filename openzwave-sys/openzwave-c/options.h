#include <openzwave/Options.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef OpenZWave::Options Options;

Options * options_create(char const *configPath, char const *userPath, char const *commandLine);
Options * options_get();
bool options_lock(Options * option);
bool options_add_option_string(Options * Options, const char *_name, const char *_default, bool _append);
bool options_destroy();

#ifdef __cplusplus
}  // extern "C"
#endif
