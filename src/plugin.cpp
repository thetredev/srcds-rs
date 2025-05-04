// ========= INCLUDES =========
// HL2SDK
#include <eiface.h>
#include <tier0/icommandline.h>

// Plugin
#include "plugin.h"
#include "plugin_bridge.h"

// memdbgon must be the last include file in a .cpp file!!!
#include <tier0/memdbgon.h>

// ========= PLUGIN IMPLEMENTATION =========

// Constructors and destructor
Plugin::Plugin()
    : Plugin{0}
{}

Plugin::~Plugin(){}

Plugin::Plugin(int client_command_index)
    : m_bridge{plugin_bridge::create_plugin_bridge(client_command_index)}
{}

bool Plugin::Load(CreateInterfaceFn interface_factory, CreateInterfaceFn game_server_factory) {
    plugin_bridge::on_load(*m_bridge, &interface_factory, &game_server_factory);
}

void Plugin::Unload() {
    plugin_bridge::on_unload(*m_bridge);
    m_bridge = nullptr;
}

void Plugin::Pause() {
    plugin_bridge::on_pause(*m_bridge);
}

void Plugin::UnPause() {
    plugin_bridge::on_unpause(*m_bridge);
}

const char *Plugin::GetPluginDescription(void) {
    plugin_bridge::get_plugin_description(*m_bridge);
}

int Plugin::GetCommandIndex() {
    return m_bridge->get_command_index();
}

void Plugin::SetCommandClient(int index) {
    m_bridge->set_command_index(index);
}

PLUGIN_RESULT Plugin::ClientConnect(
    bool *allowed, edict_t *entity, const char *name,
    const char *address, char *reject, int maxlen
) {
    return static_cast<PLUGIN_RESULT>(
        plugin_bridge::on_client_connect(*m_bridge, allowed, entity, name, address, reject, maxlen)
    );
}

PLUGIN_RESULT Plugin::ClientCommand(edict_t *entity, const CCommand &args) {
    return static_cast<PLUGIN_RESULT>(
        plugin_bridge::on_client_command(*m_bridge, entity, &args)
    );
}

void Plugin::ClientActive(edict_t *entity) {
    plugin_bridge::on_client_activate(*m_bridge, entity);
}

void Plugin::ClientDisconnect(edict_t *entity) {
    plugin_bridge::on_client_disconnect(*m_bridge, entity);
}

void Plugin::ClientPutInServer(edict_t *entity, const char *name) {
    plugin_bridge::on_client_put_in_server(*m_bridge, entity, name);
}

void Plugin::ClientSettingsChanged(edict_t *entity) {
    plugin_bridge::on_client_settings_changed(*m_bridge, entity);
}

PLUGIN_RESULT Plugin::NetworkIDValidated(const char *name, const char *id) {
    plugin_bridge::on_network_id_validated(*m_bridge, name, id);
}

void Plugin::OnQueryCvarValueFinished(
    QueryCvarCookie_t cookie, edict_t *entity, EQueryCvarValueStatus status,
    const char *name, const char *value
) {
    plugin_bridge::on_query_cvar_value_finished(
        *m_bridge, cookie, entity, status, name, value
    );
}

void Plugin::LevelInit(const char *map_name) {
    plugin_bridge::on_level_init(*m_bridge, map_name);
}

void Plugin::LevelShutdown() {
    plugin_bridge::on_level_shutdown(*m_bridge);
}

void Plugin::ServerActivate(edict_t *edict_list, int edict_count, int client_max) {
    plugin_bridge::on_server_activate(*m_bridge, edict_list, edict_count, client_max);
}

void Plugin::GameFrame(bool simulating) {
    plugin_bridge::on_game_frame(*m_bridge, simulating);
}

void Plugin::OnEdictAllocated(edict_t *edict) {
    plugin_bridge::on_edict_allocated(*m_bridge, edict);
}

void Plugin::OnEdictFreed(const edict_t *edict) {
    plugin_bridge::on_edict_freed(*m_bridge, edict);
}

void Plugin::FireGameEvent(KeyValues *event) {
    plugin_bridge::on_fire_game_event(*m_bridge, event);
}


// Expose plugin interface singleton to SRCDS
Plugin plugin;
EXPOSE_SINGLE_INTERFACE_GLOBALVAR(
    Plugin, IServerPluginCallbacks, INTERFACEVERSION_ISERVERPLUGINCALLBACKS, plugin
);

