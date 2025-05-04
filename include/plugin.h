#ifndef SRCDS_PLUGIN_H_
#define SRCDS_PLUGIN_H_

#include "plugin_bridge.h"


// Plugin class declaration
class Plugin :
    public IServerPluginCallbacks,
    public IGameEventListener {
public:
    Plugin();
    ~Plugin();

private:
    Plugin(int client_command_index);

public:
    // Required interface methods
    virtual bool Load(CreateInterfaceFn interface_factory, CreateInterfaceFn game_server_factory);
    virtual void Unload(void);
    virtual void Pause(void);
    virtual void UnPause(void);
    virtual const char *GetPluginDescription(void);
    virtual int GetCommandIndex();
    virtual void SetCommandClient(int index);

    // Game events
    virtual void LevelInit(const char *level_name);
    virtual void LevelShutdown(void);
    virtual void ServerActivate(edict_t *edict_list, int edict_count, int client_count_max);
    virtual void GameFrame(bool simulating);

    // Client events
    virtual PLUGIN_RESULT ClientConnect(
        bool *client_allowed, edict_t *client_entity,
        const char *client_name, const char *client_address,
        char *reject, int max_reject_len
    );
    virtual void ClientActive(edict_t *client_entity);
    virtual void ClientDisconnect(edict_t *client_entity);
    virtual void ClientPutInServer(edict_t *client_entity, const char *player_name);
    virtual PLUGIN_RESULT ClientCommand(edict_t *client_entity, const CCommand &args);
    virtual void ClientSettingsChanged(edict_t *client_entity);
    virtual PLUGIN_RESULT NetworkIDValidated(const char *client_name, const char *network_id);

    // Query handling
    virtual void OnQueryCvarValueFinished(
        QueryCvarCookie_t query_cvar_cookie,
        edict_t *client_entity, EQueryCvarValueStatus query_cvar_value_status,
        const char *cvar_name, const char *cvar_value
    );

    // Edict management
    virtual void OnEdictAllocated(edict_t *edict);
    virtual void OnEdictFreed(const edict_t *edict);

    // Game events
    virtual void FireGameEvent(KeyValues *event_data);

private:
    std::unique_ptr<plugin_bridge::PluginBridge> m_bridge;
};

#endif // SRCDS_PLUGIN_H_
