// Converts Trunk's dist/index.html into the Artifact page format:
// no doctype/html/head/body tags; <title> and the viewport-fit meta first, then head content, then body content.
// Usage: node scripts/artifactify.mjs <dist-dir>
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const dir = process.argv[2];
if (!dir) { console.error("usage: artifactify.mjs <dist-dir>"); process.exit(1); }
const file = join(dir, "index.html");
const html = readFileSync(file, "utf8");

const pick = (re) => (html.match(re) || [, ""])[1];
let head = pick(/<head[^>]*>([\s\S]*?)<\/head>/i);
const body = pick(/<body[^>]*>([\s\S]*?)<\/body>/i);
const title = pick(/<title>([\s\S]*?)<\/title>/i).trim();
if (!title) { console.error("artifactify: no <title> in", file); process.exit(1); }

head = head
  .replace(/<title>[\s\S]*?<\/title>/i, "")
  .replace(/<meta\s+charset[^>]*>/gi, "")
  .replace(/<meta\s+name="viewport"[^>]*>/gi, "")
  .trim();

const out = `<title>${title}</title>\n<meta name="claude:viewport-fit-sides" content="cover">\n${head}\n${body.trim()}\n`;
writeFileSync(file, out);
console.log(`artifactify: wrote ${file} (${out.length} bytes)`);
