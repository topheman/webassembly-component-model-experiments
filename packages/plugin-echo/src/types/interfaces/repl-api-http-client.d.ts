/** @module Interface repl:api/http-client **/
export function get(url: string, headers: Array<HttpHeader>): HttpResponse;
export interface HttpHeader {
  name: string;
  value: string;
}
export interface HttpResponse {
  status: number;
  ok: boolean;
  headers: Array<HttpHeader>;
  body: string;
}
