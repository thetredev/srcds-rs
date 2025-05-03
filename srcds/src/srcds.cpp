#include <stdio.h>
#include "../include/srcds.h"

// Implementation of the Plugin class
Plugin::Plugin() {
    printf("Plugin constructor called\n");
}

void Plugin::hello() {
    std::lock_guard<std::mutex> lock(mutex);
    printf("Hello from C++ Plugin!\n");
}

// Global Plugin instance
Plugin plugin;

// C interface implementation
extern "C" {

// Return a pointer to the global Plugin instance
Plugin_C* get_plugin_instance() {
    return plugin_to_c(&plugin);
}

// Call hello() on a Plugin instance
void call_plugin_hello(Plugin_C* p) {
    Plugin* cpp_plugin = plugin_from_c(p);
    cpp_plugin->hello();
}

} // extern "C"
