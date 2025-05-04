use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;

#[cxx::bridge()]
pub mod ffi {
    unsafe extern "C++" {
        include!("plugin_bridge.h");

        // The PluginBridge struct
        #[namespace = "plugin_bridge"]
        type PluginBridge;

        type CreateInterfaceFn;
        type edict_t;
        type CCommand;
        type KeyValues;
        type QueryCvarCookie_t;

        // Factory functions
        #[namespace = "plugin_bridge"]
        fn create_plugin_bridge(client_command_index: i32) -> UniquePtr<PluginBridge>;

        // Already implemented functions
        #[namespace = "plugin_bridge"]
        fn get_command_index(self: &PluginBridge) -> i32;

        #[namespace = "plugin_bridge"]
        fn set_command_index(self: Pin<&mut PluginBridge>, index: i32);
    }

    // Rust implementations
    #[namespace = "plugin_bridge"]
    extern "Rust" {
        fn get_plugin_description(bridge: &PluginBridge) -> &'static str;

        unsafe fn on_load(
            bridge: &PluginBridge, interface_factory: *mut CreateInterfaceFn,
            game_server_factory: *mut CreateInterfaceFn
        ) -> bool;
        fn on_unload(bridge: &PluginBridge);
        fn on_pause(bridge: &PluginBridge);
        fn on_unpause(bridge: &PluginBridge);

        fn on_level_init(bridge: &PluginBridge, level_name: &str);
        fn on_level_shutdown(bridge: &PluginBridge);
        unsafe fn on_server_activate(
            bridge: &PluginBridge, edict_list: *mut edict_t,
            edict_count: i32, client_count_max: i32
        );
        fn on_game_frame(bridge: &PluginBridge, is_simulating: bool);

        unsafe fn on_client_connect(
            bridge: &PluginBridge, client_allowed: *mut bool,
            client_entity: *mut edict_t, client_name: &str,
            client_address: &str, reject: &str, max_reject_len: i32
        ) -> i32;
        unsafe fn on_client_activate(bridge: &PluginBridge, client_entity: *mut edict_t);
        unsafe fn on_client_disconnect(bridge: &PluginBridge, client_entity: *mut edict_t);
        unsafe fn on_client_put_in_server(bridge: &PluginBridge, client_entity: *mut edict_t, player_name: &str);
        unsafe fn on_client_command(bridge: &PluginBridge, client_entity: *mut edict_t, args: *const CCommand) -> i32;
        unsafe fn on_client_settings_changed(bridge: &PluginBridge, client_entity: *mut edict_t);

        fn on_network_id_validated(bridge: &PluginBridge, client_name: &str, network_id: &str) -> i32;

        unsafe fn on_query_cvar_value_finished(
            bridge: &PluginBridge, query_cvar_cookie: &QueryCvarCookie_t,
            client_entity: *mut edict_t, query_cvar_value_status: i32,
            cvar_name: &str, cvar_value: &str
        );

        unsafe fn on_edict_allocated(bridge: &PluginBridge, edict: *mut edict_t);
        unsafe fn on_edict_freed(bridge: &PluginBridge, edict: *const edict_t);

        unsafe fn on_fire_game_event(bridge: &PluginBridge, event_data: *mut KeyValues);
    }
}

// Implement all the Rust functions required by the bridge
fn get_plugin_description(bridge: &ffi::PluginBridge) -> &'static str {
    "Rust Plugin Implementation"
}

fn on_load(bridge: &ffi::PluginBridge, interface_factory: *mut ffi::CreateInterfaceFn,
          game_server_factory: *mut ffi::CreateInterfaceFn) -> bool {
    println!("Plugin loaded with command index: {}", bridge.get_command_index());
    true
}

fn on_unload(bridge: &ffi::PluginBridge) {
    println!("Plugin unloaded");
}

fn on_pause(bridge: &ffi::PluginBridge) {
    println!("Plugin paused");
}

fn on_unpause(bridge: &ffi::PluginBridge) {
    println!("Plugin unpaused");
}

fn on_level_init(bridge: &ffi::PluginBridge, level_name: &str) {
    println!("Level initialized: {}", level_name);
}

fn on_level_shutdown(bridge: &ffi::PluginBridge) {
    println!("Level shutdown");
}

fn on_server_activate(bridge: &ffi::PluginBridge, edict_list: *mut ffi::edict_t,
                     edict_count: i32, client_count_max: i32) {
    println!("Server activated with {} edicts, max clients: {}", edict_count, client_count_max);
}

fn on_game_frame(bridge: &ffi::PluginBridge, is_simulating: bool) {
    // Game frame update - typically don't log here as it would spam the console
}

fn on_client_connect(bridge: &ffi::PluginBridge, client_allowed: *mut bool,
                    client_entity: *mut ffi::edict_t, client_name: &str,
                    client_address: &str, reject: &str, max_reject_len: i32) -> i32 {
    println!("Client connecting: {} from {}", client_name, client_address);

    // Allow the client to connect
    if !client_allowed.is_null() {
        unsafe { *client_allowed = true; }
    }

    0 // Return 0 for success
}

fn on_client_activate(bridge: &ffi::PluginBridge, client_entity: *mut ffi::edict_t) {
    println!("Client activated");
}

fn on_client_disconnect(bridge: &ffi::PluginBridge, client_entity: *mut ffi::edict_t) {
    println!("Client disconnected");
}

fn on_client_put_in_server(bridge: &ffi::PluginBridge, client_entity: *mut ffi::edict_t, player_name: &str) {
    println!("Client put in server: {}", player_name);
}

fn on_client_command(bridge: &ffi::PluginBridge, client_entity: *mut ffi::edict_t, args: *const ffi::CCommand) -> i32 {
    // Handle client commands
    0 // Return 0 for success
}

fn on_client_settings_changed(bridge: &ffi::PluginBridge, client_entity: *mut ffi::edict_t) {
    println!("Client settings changed");
}

fn on_network_id_validated(bridge: &ffi::PluginBridge, client_name: &str, network_id: &str) -> i32 {
    println!("Network ID validated for {}: {}", client_name, network_id);
    0 // Return 0 for success
}

fn on_query_cvar_value_finished(bridge: &ffi::PluginBridge, query_cvar_cookie: &ffi::QueryCvarCookie_t,
                               client_entity: *mut ffi::edict_t, query_cvar_value_status: i32,
                               cvar_name: &str, cvar_value: &str) {
    println!("Query cvar value finished: {} = {}", cvar_name, cvar_value);
}

fn on_edict_allocated(bridge: &ffi::PluginBridge, edict: *mut ffi::edict_t) {
    // An edict was allocated
}

fn on_edict_freed(bridge: &ffi::PluginBridge, edict: *const ffi::edict_t) {
    // An edict was freed
}

fn on_fire_game_event(bridge: &ffi::PluginBridge, event_data: *mut ffi::KeyValues) {
    println!("Game event fired");
}
