#ifndef SRCDS_PLUGIN_BRIDGE_H_
#define SRCDS_PLUGIN_BRIDGE_H_

#include <memory>

#include <eiface.h>
#include <igameevents.h>
#include <engine/iserverplugin.h>

#include "rust/cxx.h"

namespace plugin_bridge {

struct PluginBridge {
    int command_index;

    // Already implemented functions
    int get_command_index() const {
        return command_index;
    }

    void set_command_index(int index) {
        command_index = index;
    }
};

// Factory function
std::unique_ptr<PluginBridge> create_plugin_bridge(int client_command_index);

} // namespace plugin_bridge

// Include the cxx generated header
#include "srcds/src/lib.rs.h"

#endif // SRCDS_PLUGIN_BRIDGE_H_
