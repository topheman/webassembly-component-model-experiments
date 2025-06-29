/** @module Interface repl:api/plugin **/
export function name(): string;
export function man(): string;
export function run(payload: string): PluginResponse;
export type PluginResponse = import("./repl-api-transport.js").PluginResponse;
