// The dashboard's only bridge: same-origin JSON calls to the Worker, and the clipboard.

export async function call(method, path, body) {
  try {
    const res = await fetch(path, {
      method,
      credentials: "same-origin",
      headers: body === undefined || body === null ? {} : { "content-type": "application/json" },
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
