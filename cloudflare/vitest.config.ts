import { defineConfig } from "vitest/config";
import { cloudflareTest, readD1Migrations } from "@cloudflare/vitest-plugin";

export default defineConfig(async () => {
  const migrations = await readD1Migrations("./migrations");
  return {
    plugins: [
      cloudflareTest({
        wrangler: { configPath: "./wrangler.toml" },
        miniflare: { bindings: { TEST_MIGRATIONS: migrations, BETTER_AUTH_SECRET: "test-secret-for-vitest-only-0123456789abcdef", OWNER_EMAILS: "owner@example.com" } },
      }),
    ],
    test: { setupFiles: ["./test/setup.ts"] },
  };
});
