"""
Resolve the >>>>>>> conflict blocks in _posts/2017-09-01-turing-suicide.markdown.

    python3 resolve.py published   # keep current text, bin the proposals
    python3 resolve.py proposed    # take the proposals
    python3 resolve.py check       # just count blocks, change nothing

Writes in place. Nothing here is clever; it exists so you don't hand-delete
fifteen markers and miss one.
"""

import sys
import pathlib

POST = pathlib.Path(__file__).parents[2] / "_posts" / "2017-09-01-turing-suicide.markdown"
OPEN, MID, CLOSE = "<<<<<<< PUBLISHED", "=======", ">>>>>>> PROPOSED"


def resolve(text, side):
    out, mode, blocks = [], "keep", 0
    for line in text.split("\n"):
        if line.startswith(OPEN):
            mode, blocks = "pub", blocks + 1
            continue
        if line == MID and mode == "pub":
            mode = "prop"
            continue
        if line.startswith(CLOSE):
            mode = "keep"
            continue
        if mode == "keep" or mode == side:
            out.append(line)
    return "\n".join(out), blocks


def main():
    side = (sys.argv[1] if len(sys.argv) > 1 else "check").lower()
    text = POST.read_text(encoding="utf-8")

    if side == "check":
        n = text.count(OPEN), text.count("\n" + MID + "\n"), text.count(CLOSE)
        print(f"open {n[0]} / mid {n[1]} / close {n[2]}", "-- balanced" if len(set(n)) == 1 else "-- *** UNBALANCED")
        return
    if side not in ("published", "proposed"):
        sys.exit("usage: resolve.py [published|proposed|check]")

    body, blocks = resolve(text, "pub" if side == "published" else "prop")
    assert OPEN not in body and CLOSE not in body, "markers survived resolution"
    POST.write_text(body, encoding="utf-8")
    print(f"resolved {blocks} blocks to {side.upper()}; markers gone.")
    if side == "published":
        print("note: fn:11 is still an orphan (defined, never referenced) on this side.")


if __name__ == "__main__":
    main()
