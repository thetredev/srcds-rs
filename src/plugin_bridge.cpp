#include "plugin_bridge.h"

namespace plugin_bridge {

std::unique_ptr<PluginBridge> create_plugin_bridge(int client_command_index) {
    std::unique_ptr<PluginBridge> bridge = std::make_unique<PluginBridge>();
    bridge->command_index = client_command_index;
    return bridge;
}

} // namespace plugin_bridge
