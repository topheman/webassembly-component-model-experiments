#include "plugin_api.h"
#include <string.h>
#include <stdlib.h>

/*
 * C implementation of the echo plugin
 *
 * IMPLEMENTATION SOURCE:
 * This implements the same interface as the Rust version in crates/plugin-echo/src/lib.rs
 * The function signatures are generated from the WIT interface by wit-bindgen:
 * - exports_repl_api_plugin_name() corresponds to fn name() -> String
 * - exports_repl_api_plugin_man() corresponds to fn man() -> String
 * - exports_repl_api_plugin_run() corresponds to fn run(payload: String) -> Result<PluginResponse, ()>
 *
 * MEMORY MANAGEMENT:
 * - Input parameters (like payload) are owned by the runtime - DO NOT free them
 * - Output parameters (like ret) are populated by us, freed by the runtime
 * - plugin_api_string_dup() allocates new memory for string copies
 * - The generated _free functions handle cleanup automatically
 * - No explicit free() calls needed in plugin code
 */

void exports_repl_api_plugin_name(plugin_api_string_t *ret)
{
    // Populate ret with "echo" as the plugin name
    // plugin_api_string_dup() allocates new memory and copies the string
    plugin_api_string_dup(ret, "echoc");
}

void exports_repl_api_plugin_man(plugin_api_string_t *ret)
{
    // Populate ret with the manual text for the echo command
    // plugin_api_string_dup() allocates new memory and copies the string
    const char *man_text =
        "\n"
        "NAME\n"
        "    echoc - Echo a message (built with C)\n"
        "\n"
        "USAGE\n"
        "    echoc <message>\n"
        "\n"
        "DESCRIPTION\n"
        "    Echo a message.\n"
        "\n"
        "        ";
    plugin_api_string_dup(ret, man_text);
}

bool exports_repl_api_plugin_run(plugin_api_string_t *payload, exports_repl_api_plugin_plugin_response_t *ret)
{
    // Set status to success (0 = success, 1 = error)
    ret->status = REPL_API_TRANSPORT_REPL_STATUS_SUCCESS;

    // Set stdout to contain the payload
    // is_some = true means the optional string has a value
    ret->stdout.is_some = true;
    // plugin_api_string_dup() creates a new copy of payload->ptr in ret->stdout.val
    // This allocates new memory and copies the string content
    plugin_api_string_dup(&ret->stdout.val, (const char *)payload->ptr);

    // Set stderr to none (no error output)
    ret->stderr.is_some = false;

    // Return true for success (false would indicate an error)
    // This corresponds to Ok(response) in the Rust Result<T, ()> pattern
    return true;
}
