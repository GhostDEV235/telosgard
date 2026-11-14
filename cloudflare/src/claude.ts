// Claude through the Worker (Phase 19, ADR 0003). The app builds each prompt (grading, verdicts,
// practice questions, the step tutor) and sends it here; the Worker adds the API key, a fixed model,
// an effort per tier and an output cap, and returns either parsed JSON or plain text. It never
// invents an answer: a refusal, an unreadable reply or an API error comes back as an error code the
// app's bridge already understands.

import Anthropic from "@anthropic-ai/sdk";

/** Claude Haiku 5.5: the lowest-cost current model ($0.10 / $0.50 per million tokens). */
export const MODEL = "claude-haiku-5-5";

/** The app's tiers map onto effort. Haiku 5.5 defaults to medium, so it is always set explicitly. */
const EFFORT = { quick: "low", default: "medium", complex: "high" } as const;
export type Tier = keyof typeof EFFORT;
export type Mode = "json" | "text";

/** Output caps include adaptive thinking; text answers (the tutor) are short by design. */
const MAX_TOKENS: Record<Mode, number> = { json: 16000, text: 4000 };
export const MAX_PROMPT_CHARS = 60000;

const SYSTEM: Record<Mode, string> = {
  json: "You are Professor Aurel, the AI grader and coach inside Telosgard, a course that teaches a complete beginner to trade gold. Follow the user message exactly. Reply with only the JSON it asks for: no prose, no code fences.",
  text: "You are Professor Aurel, the AI tutor inside Telosgard, a course that teaches a complete beginner to trade gold. If asked who you are, say you are Professor Aurel, Telosgard's AI tutor. Follow the user message's rules. Reply in plain text.",
};

export class ClaudeError extends Error {
  constructor(public status: number, public code: string, message: string) {
    super(message);
  }
}

/** The part of the SDK client this module uses, so tests can pass a fake. */
export interface MessagesClient {
  beta: { messages: { create(params: Anthropic.Beta.MessageCreateParamsNonStreaming): Promise<Anthropic.Beta.BetaMessage> } };
}

export function parseTier(t: unknown): Tier {
  return t === "quick" || t === "complex" ? t : "default";
}

/** The first JSON object or array in a reply, tolerating code fences or a stray sentence around it. */
export function extractJson(text: string): unknown {
  const trimmed = text.trim().replace(/^```(?:json)?\s*/i, "").replace(/\s*```$/, "");
  try {
    return JSON.parse(trimmed);
  } catch {}
  const start = trimmed.search(/[[{]/);
  if (start < 0) throw new ClaudeError(502, "invalid_json", "Professor Aurel's reply couldn't be read.");
  const open = trimmed[start];
  const close = open === "{" ? "}" : "]";
  const end = trimmed.lastIndexOf(close);
  if (end <= start) throw new ClaudeError(502, "invalid_json", "Professor Aurel's reply was incomplete.");
  try {
    return JSON.parse(trimmed.slice(start, end + 1));
  } catch {
    throw new ClaudeError(502, "invalid_json", "Professor Aurel's reply couldn't be read.");
  }
}

export interface ClaudeResult {
  value?: unknown;
  text?: string;
  truncated?: boolean;
  usage: { input: number; output: number };
}

export async function askClaude(client: MessagesClient, mode: Mode, prompt: string, tier: Tier): Promise<ClaudeResult> {
  if (!prompt.trim()) throw new ClaudeError(400, "invalid_argument", "Empty prompt.");
  if (prompt.length > MAX_PROMPT_CHARS) throw new ClaudeError(413, "prompt_too_large", "That request is too long.");

  let msg: Anthropic.Beta.BetaMessage;
  try {
    msg = await client.beta.messages.create({
      model: MODEL,
      max_tokens: MAX_TOKENS[mode],
      system: SYSTEM[mode],
      thinking: { type: "adaptive" },
      output_config: { effort: EFFORT[tier] },
      // Haiku 5.5 has no server-side fallback (a `fallbacks` list is a 400): a decline comes back as
      // stop_reason "refusal", handled below.
      messages: [{ role: "user", content: prompt }],
    });
  } catch (e) {
    throw mapApiError(e);
  }

  const usage = { input: msg.usage.input_tokens, output: msg.usage.output_tokens };
  if (msg.stop_reason === "refusal") {
    throw new ClaudeError(422, "refused", "Professor Aurel declined this request.");
  }
  const text = msg.content
    .filter((b): b is Anthropic.Beta.BetaTextBlock => b.type === "text")
    .map((b) => b.text)
    .join("")
    .trim();
  const truncated = msg.stop_reason === "max_tokens";

  if (mode === "text") {
    if (!text) throw new ClaudeError(502, "invalid_json", "Professor Aurel sent back an empty answer.");
    return { text, truncated, usage };
  }
  if (truncated) throw new ClaudeError(502, "invalid_json", "Professor Aurel's reply was cut short.");
  return { value: extractJson(text), usage };
}

/** SDK errors → the bridge's codes. Most specific first. */
export function mapApiError(e: unknown): ClaudeError {
  if (e instanceof ClaudeError) return e;
  if (e instanceof Anthropic.RateLimitError) return new ClaudeError(429, "rate_limited", "Professor Aurel is busy. Try again in a minute.");
  if (e instanceof Anthropic.AuthenticationError || e instanceof Anthropic.PermissionDeniedError) {
    return new ClaudeError(503, "not_granted", "Professor Aurel isn't set up on the server (the API key is missing or invalid).");
  }
  if (e instanceof Anthropic.BadRequestError) return new ClaudeError(400, "invalid_request", "Professor Aurel couldn't accept that request.");
  if (e instanceof Anthropic.APIConnectionError) return new ClaudeError(503, "unavailable", "Couldn't reach Professor Aurel.");
  if (e instanceof Anthropic.APIError) return new ClaudeError(503, "unavailable", "Professor Aurel had a problem. Try again.");
  return new ClaudeError(500, "unavailable", "Something went wrong asking Professor Aurel.");
}
