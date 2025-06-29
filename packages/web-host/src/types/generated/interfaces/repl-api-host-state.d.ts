/** @module Interface repl:api/host-state **/
export function getPluginsNames(): Array<string>;
export function setReplVars(vars: Array<ReplVar>): void;
export function getReplVars(): Array<ReplVar>;
export function setReplVar(var_: ReplVar): void;
export type ReadlineResponse =
  import("./repl-api-transport.js").ReadlineResponse;
export type ReplVar = import("./repl-api-transport.js").ReplVar;
