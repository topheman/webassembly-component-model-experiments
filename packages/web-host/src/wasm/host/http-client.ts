import type {
  get as HttpGetHost,
  HttpHeader,
  HttpResponse,
} from "../../types/generated/interfaces/repl-api-http-client";

// biome-ignore lint/correctness/noUnusedVariables: commented code
// @ts-ignore
type AsyncHttpGetHost = (
  ...args: Parameters<typeof HttpGetHost>
) => Promise<ReturnType<typeof HttpGetHost>>;

type SyncHttpGetHost = typeof HttpGetHost;

// export const get: AsyncHttpGetHost = async (
//   url: string,
//   headers: HttpHeader[] = [],
// ): Promise<HttpResponse> => {
//   console.log("http-client: get", url, headers);
//   await new Promise((resolve) => setTimeout(resolve, 5000));
//   const response = await fetch(url, {
//     headers: headers.map((header) => [header.name, header.value]),
//   });
//   console.log("http-client: get response", response);
//   const result = {
//     status: response.status,
//     headers: Array.from(response.headers.entries()).map(([name, value]) => ({
//       name,
//       value,
//     })),
//     body: response.body ? await response.text() : "",
//     ok: response.ok,
//   };
//   console.log("http-client: get result", result);
//   return result;
// };

/**
 * For the moment, the http client exposed in synchronous, hence the use of XMLHttpRequest.
 * This is not ideal, but async calls are not supported yet with jco.
 */
export const get: SyncHttpGetHost = (
  url: string,
  headers: HttpHeader[],
): HttpResponse => {
  const request = new XMLHttpRequest();
  request.open("GET", url, false);
  for (const header of headers) {
    console.log("[Host][http-client] set header ⬆️", header);
    request.setRequestHeader(header.name, header.value);
  }
  request.send(null);

  const status = request.status;
  const responseText = request.responseText;
  const responseHeaders = request
    .getAllResponseHeaders()
    .split("\n")
    .map((line) => {
      const [name, value] = line.split(": ");
      return {
        name,
        value,
      };
    })
    .filter((header) => header.name !== "" && header.value !== undefined);
  console.log("[Host][http-client] get request ⬇️", request);

  if (status >= 200 && status < 300) {
    const result = {
      status,
      ok: true,
      headers: responseHeaders,
      body: responseText,
    };
    console.log("[Host][http-client] get result ✅", result);
    return result;
  }
  const result = {
    status: request.status,
    ok: false,
    headers: [],
    body: request.responseText,
  };
  console.log("[Host][http-client] get result ❌", result);
  return result;
};
