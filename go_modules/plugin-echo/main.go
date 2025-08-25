package main

import (
	"webassembly-repl/plugin-echo/internal/repl/api/plugin"
	"webassembly-repl/plugin-echo/internal/repl/api/transport"

	"go.bytecodealliance.org/cm"
)

func init() {
	// Export the plugin name function
	plugin.Exports.Name = func() string {
		return "echogo"
	}

	// Export the manual function
	plugin.Exports.Man = func() string {
		return `NAME
    echogo - Echo a message (built with Go)

USAGE
    echogo <message>

DESCRIPTION
    Echo a message.`
	}

	// Export the run function
	plugin.Exports.Run = func(payload string) cm.Result[plugin.PluginResponse, plugin.PluginResponse, struct{}] {
		response := plugin.PluginResponse{
			Status: transport.ReplStatusSuccess,
			Stdout: cm.Some(payload),
			Stderr: cm.None[string](),
		}
		return cm.OK[cm.Result[plugin.PluginResponse, plugin.PluginResponse, struct{}]](response)
	}
}

// main is required for the wasip2 target
func main() {}
