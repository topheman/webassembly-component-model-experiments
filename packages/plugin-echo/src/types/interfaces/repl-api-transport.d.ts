/** @module Interface repl:api/transport **/
/**
 * # Variants
 *
 * ## `"success"`
 *
 * ## `"error"`
 */
export type ReplStatus = "success" | "error";
export interface PluginResponse {
  status: ReplStatus;
  stdout?: string;
  stderr?: string;
}
export interface ParsedLine {
  command: string;
  payload: string;
}
export type ReadlineResponse = ReadlineResponseToRun | ReadlineResponseReady;
export interface ReadlineResponseToRun {
  tag: "to-run";
  val: ParsedLine;
}
export interface ReadlineResponseReady {
  tag: "ready";
  val: PluginResponse;
}
export interface ReplVar {
  key: string;
  value: string;
}
