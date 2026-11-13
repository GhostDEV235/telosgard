// The dashboard's only bridge: same-origin JSON calls to the Worker, and the clipboard.

export async function call(method, path, body, code) {
  try {
    const headers = body === undefined || body === null ? {} : { "content-type": "application/json" };
    // Step-up for sensitive actions (ADR 0007): the current authenticator code.
    if (code) headers["x-telosgard-code"] = code;
    const res = await fetch(path, {
      method,
      credentials: "same-origin",
      headers,
      body: body === undefined || body === null ? undefined : body,
    });
    return JSON.stringify({ status: res.status, body: await res.text() });
  } catch (e) {
    return JSON.stringify({ status: 0, body: "" });
  }
}

export async function copyText(text) {
  try {
    await navigator.clipboard.writeText(text);
    return true;
  } catch (e) {
    return false;
  }
}
