#!/usr/bin/env python3
"""French typography for lessons/fr/*.md: narrow no-break space before ? ! ; :, and no-break
spaces inside « ». Idempotent. Usage: python3 -I scripts/fr-typography.py crates/content/lessons/fr/*.md"""
import re, sys

NNBSP, NBSP = " ", " "

def fix(text: str) -> str:
    # ' ?' ' !' ' ;' ' :' (a plain or no-break space before) -> narrow no-break space
    text = re.sub(r"[  ]([?!;:])(?=[\s\"*\])]|$)", NNBSP + r"\1", text, flags=re.M)
    text = re.sub(r"«[   ]*", "«" + NBSP, text)
    text = re.sub(r"[   ]*»", NBSP + "»", text)
    # 4 100,00 · 1 000 $ · 80 % : keep numbers whole and their unit on the same line
    text = re.sub(r"(\d) (\d{3})(?!\d)", "\\1" + NNBSP + "\\2", text)
    text = re.sub(r"(\d) ([$%€])", "\\1" + NBSP + "\\2", text)
    return text

for path in sys.argv[1:]:
    with open(path, encoding="utf-8") as f:
        src = f.read()
    out = fix(src)
    if out != src:
        with open(path, "w", encoding="utf-8") as f:
            f.write(out)
        print("fixed", path)
