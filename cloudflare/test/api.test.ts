import { env } from "cloudflare:test";
import { describe, expect, it } from "vitest";
import worker, { type Env } from "../src/index";


/** Calls the Worker as `user` (via DEV_USER on localhost), or signed out when user is null. */
async function call(user: string | null, method: string, path: string, body?: BodyInit, headers: Record<string, string> = {}, host = "http://localhost") {
  const e = { ...env, DEV_USER: user ?? undefined };
  const res = await worker.fetch(new Request(host + path, { method, body, headers }), e as unknown as Env);
  const text = await res.text();
  let data: unknown = text;
  try {
    data = JSON.parse(text);
  } catch {}
  return { status: res.status, data };
}

const doc = (kind: string, extra: Record<string, unknown> = {}) => JSON.stringify({ kind, ...extra });

describe("identity", () => {
  it("refuses everything except /api/me when signed out", async () => {
    expect((await call(null, "GET", "/api/me")).data).toEqual({ signedIn: false });
    const r = await call(null, "GET", "/api/docs/profile");
    expect(r.status).toBe(401);
    expect(r.data).toMatchObject({ code: "not_granted" });
  });

  it("only honours DEV_USER on localhost", async () => {
    expect((await call("alice", "GET", "/api/me")).data).toMatchObject({ signedIn: true, userId: "alice" });
    const remote = await call("alice", "GET", "/api/docs/profile", undefined, {}, "https://telosgard.example.workers.dev");
    expect(remote.status).toBe(401);
  });
});

describe("documents", () => {
  it("round-trips a document and lists it by kind", async () => {
    expect((await call("alice", "PUT", "/api/docs/lesson-01", doc("progress", { quizBest: 100 }))).status).toBe(200);
    expect((await call("alice", "GET", "/api/docs/lesson-01")).data).toEqual({ kind: "progress", quizBest: 100 });
    await call("alice", "PUT", "/api/docs/qrec-01", doc("qrec", { lesson: 1 }));
    const list = await call("alice", "GET", "/api/docs?kind=progress");
    expect(list.data).toEqual([{ id: "lesson-01", data: { kind: "progress", quizBest: 100 } }]);
    expect((await call("alice", "GET", "/api/docs/nope")).data).toBeNull();
  });

  it("keeps one user's documents away from another", async () => {
    await call("alice", "PUT", "/api/docs/plan-v1", doc("plan", { version: 1 }));
    expect((await call("bob", "GET", "/api/docs/plan-v1")).data).toBeNull();
    expect((await call("bob", "GET", "/api/docs?kind=plan")).data).toEqual([]);
    await call("bob", "DELETE", "/api/docs/plan-v1");
    expect((await call("alice", "GET", "/api/docs/plan-v1")).data).toEqual({ kind: "plan", version: 1 });
  });

  it("overwrites and deletes", async () => {
    await call("alice", "PUT", "/api/docs/profile", doc("profile", { theme: "dark" }));
    await call("alice", "PUT", "/api/docs/profile", doc("profile", { theme: "light" }));
    expect((await call("alice", "GET", "/api/docs/profile")).data).toEqual({ kind: "profile", theme: "light" });
    await call("alice", "DELETE", "/api/docs/profile");
    expect((await call("alice", "GET", "/api/docs/profile")).data).toBeNull();
  });

  it("rejects bad ids, missing kinds and non-JSON", async () => {
    expect((await call("alice", "PUT", "/api/docs/bad%20id", doc("x"))).status).toBe(400);
    expect((await call("alice", "PUT", "/api/docs/ok", JSON.stringify({ no: "kind" }))).status).toBe(400);
    expect((await call("alice", "PUT", "/api/docs/ok", "not json")).status).toBe(400);
    expect((await call("alice", "PUT", "/api/docs/ok", "[1,2]")).status).toBe(400);
    expect((await call("alice", "GET", "/api/docs?kind=")).status).toBe(400);
  });
});

describe("files", () => {
  const png = new Uint8Array([0x89, 0x50, 0x4e, 0x47, 1, 2, 3]);
  it("stores an image for its owner only", async () => {
    const up = await call("alice", "POST", "/api/files", png, { "content-type": "image/png" });
    expect(up.status).toBe(200);
    const { id, url } = up.data as { id: string; url: string };
    expect(url).toBe(`/api/files/${id}`);
    const e = { ...env, DEV_USER: "alice" };
    const own = await worker.fetch(new Request("http://localhost" + url), e as unknown as Env);
    expect(own.headers.get("content-type")).toBe("image/png");
    expect(new Uint8Array(await own.arrayBuffer())).toEqual(png);
    expect((await call("bob", "GET", url)).status).toBe(404);
  });

  it("refuses non-images", async () => {
    expect((await call("alice", "POST", "/api/files", "hello", { "content-type": "text/plain" })).status).toBe(400);
  });
});
