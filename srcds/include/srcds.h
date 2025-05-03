#ifndef SRCDS_H
#define SRCDS_H

#include <mutex>


#ifdef __cplusplus
extern "C" {
#endif

// Forward declaration for C interface
typedef struct Plugin_C Plugin_C;

// Get a pointer to the global Plugin instance
Plugin_C* get_plugin_instance();

// Function to call hello() on a Plugin instance
void call_plugin_hello(Plugin_C* plugin);

#ifdef __cplusplus
}

// C++ Plugin class definition
class Plugin {
public:
    Plugin();
    void hello();

private:
    std::mutex mutex;
};

// Cast between C and C++ types
inline Plugin* plugin_from_c(Plugin_C* p) {
    return reinterpret_cast<Plugin*>(p);
}

inline Plugin_C* plugin_to_c(Plugin* p) {
    return reinterpret_cast<Plugin_C*>(p);
}

#endif // __cplusplus

#endif // SRCDS_H
