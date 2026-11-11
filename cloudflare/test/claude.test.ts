import { env } from "cloudflare:test";
import { describe, expect, it } from "vitest";
import { handle, type Env } from "../src/index";
import { extractJson, MODEL, type MessagesClient } from "../src/claude";

type Reply = { text?: string; stop?: string; error?: unknown };

/** A fake Anthropic client: records each request and returns a canned reply. */
function fake(reply: Reply) {
  const calls: Record<string, unknown>[] = [];
  const client: MessagesClient = {
    beta: {
      messages: {
        async create(params) {
          calls.push(params as unknown as Record<string, unknown>);
          if (reply.error) throw reply.error;
          return {
            content: reply.text === undefined ? [] : [{ type: "text", text: reply.text }],
            stop_reason: reply.stop ?? "end_turn",
            usage: { input_tokens: 120, output_tokens: 30 },
          } as never;
        },
      },
    },
  };
  return { client, calls };
}

async function ask(mode: "json" | "text", reply: Reply, opts: { user?: string | null; key?: string | null; limit?: string; tier?: string } = {}) {
  const f = fake(reply);
  const e = {
    ...env,
    DEV_USER: opts.user === null ? undefined : (opts.user ?? "alice"),
    ANTHROPIC_API_KEY: opts.key === null ? undefined : (opts.key ?? "test-key"),
    CLAUDE_DAILY_LIMIT: opts.limit,
  } as unknown as Env;
  const res = await handle(
    new Request(`http://localhost/api/claude/${mode}`, { method: "POST", body: JSON.stringify({ prompt: "Grade this answer.", tier: opts.tier ?? "default" }) }),
    e,
    () => f.client,
  );
  return { status: res.status, data: (await res.json()) as Record<string, unknown>, calls: f.calls };
}

describe("Claude through the Worker", () => {
  it("needs a signed-in user and a configured key", async () => {
    expect((await ask("json", { text: "{}" }, { user: null })).status).toBe(401);
    const r = await ask("json", { text: "{}" }, { key: null });
    expect(r.status).toBe(503);
    expect(r.data.code).toBe("not_granted");
    expect(r.calls).toHaveLength(0);
  });

  it("sends a fixed model, the tier's effort and server-side fallbacks", async () => {
    const r = await ask("json", { text: '{"grade":"got_it"}' }, { tier: "quick" });
    expect(r.status).toBe(200);
    const p = r.calls[0];
    expect(p.model).toBe(MODEL);
    expect(p.output_config).toEqual({ effort: "low" });
    expect(p.thinking).toEqual({ type: "adaptive" });
    expect(p.fallbacks).toBe("default");
    expect(p.betas).toEqual(["server-side-fallback-2026-07-01"]);
    expect(p.max_tokens).toBe(16000);
    expect((await ask("json", { text: "{}" }, { tier: "anything" })).calls[0].output_config).toEqual({ effort: "medium" });
  });

  it("returns parsed JSON, tolerating code fences", async () => {
    const r = await ask("json", { text: '```json\n{"grade":"almost","gap":"the spread"}\n```' });
    expect(r.data).toEqual({ value: { grade: "almost", gap: "the spread" } });
  });

  it("returns plain text, and flags a cut-short answer", async () => {
    expect((await ask("text", { text: "1R is your planned loss." })).data).toEqual({ text: "1R is your planned loss.", truncated: false });
    expect((await ask("text", { text: "1R is your", stop: "max_tokens" })).data).toEqual({ text: "1R is your", truncated: true });
  });

  it("never invents an answer: refusals, empty or unreadable replies are errors", async () => {
    const refused = await ask("json", { stop: "refusal" });
    expect([refused.status, refused.data.code]).toEqual([422, "refused"]);
    const unreadable = await ask("json", { text: "Sure! The grade is got it." });
    expect([unreadable.status, unreadable.data.code]).toEqual([502, "invalid_json"]);
    const cut = await ask("json", { text: '{"grade":', stop: "max_tokens" });
    expect(cut.data.code).toBe("invalid_json");
    const empty = await ask("text", { text: "   " });
    expect(empty.data.code).toBe("invalid_json");
    const broken = await ask("json", { error: new Error("socket closed") });
    expect([broken.status, broken.data.code]).toEqual([500, "unavailable"]);
  });

  it("caps each user's daily calls and logs token use", async () => {
    const user = "capped-user";
    expect((await ask("text", { text: "a" }, { user, limit: "2" })).status).toBe(200);
    expect((await ask("text", { text: "b" }, { user, limit: "2" })).status).toBe(200);
    const third = await ask("text", { text: "c" }, { user, limit: "2" });
    expect([third.status, third.data.code]).toEqual([429, "rate_limited"]);
    expect(third.calls).toHaveLength(0);
    const row = await (env as unknown as Env).DB.prepare("SELECT COUNT(*) AS n, SUM(input_tokens) AS i FROM claude_calls WHERE user_id = ?1").bind(user).first<{ n: number; i: number }>();
    expect(row).toEqual({ n: 2, i: 240 });
  });
});

describe("extractJson", () => {
  it("finds the JSON inside a reply", () => {
    expect(extractJson('{"a":1}')).toEqual({ a: 1 });
    expect(extractJson('Here you go: {"a":[1,2]} Thanks')).toEqual({ a: [1, 2] });
    expect(extractJson("[1,2]")).toEqual([1, 2]);
    expect(() => extractJson("no json here")).toThrow();
  });
});
