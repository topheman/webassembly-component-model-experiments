/** @module Interface repl:api/repl-logic **/
export function readline(line: string): ReadlineResponse;
export type ReadlineResponse =
  import("./repl-api-transport.js").ReadlineResponse;
