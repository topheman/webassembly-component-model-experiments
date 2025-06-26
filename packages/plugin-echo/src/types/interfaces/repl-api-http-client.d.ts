/** @module Interface repl:api/http-client **/
export function get(url: string, headers: Array<HttpHeader>): HttpResponse;
export function post(
  url: string,
  headers: Array<HttpHeader>,
  body: string,
): HttpResponse;
export interface HttpHeader {
  name: string;
  value: string;
}
export interface HttpResponse {
  status: number;
  headers: Array<HttpHeader>;
  body: string;
}
