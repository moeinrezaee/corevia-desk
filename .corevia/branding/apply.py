#!/usr/bin/env python3
"""Apply .corevia/branding/manifest.json to the working tree (stdlib only, idempotent).

Operations: move, delete, replace (exact multi-line blocks), asset (copy a whole file).
A replace block is skipped when its new text is already present, applied when its old text matches
exactly once, and reported as drift otherwise (upstream changed that spot: resolve by taking upstream's
text and regenerating the manifest entry).
"""
import json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
manifest = json.load(open(os.path.join(HERE, "manifest.json")))
problems = []

def read(path):
    with open(path, "rb") as f:
        return f.read().decode("utf-8", "surrogateescape")

def write(path, text):
    with open(path, "wb") as f:
        f.write(text.encode("utf-8", "surrogateescape"))

for op in manifest["ops"]:
    kind = op["op"]
    if kind == "move":
        src, dst = op["from"], op["to"]
        if os.path.exists(src) and not os.path.exists(dst):
            os.makedirs(os.path.dirname(dst), exist_ok=True)
            shutil.move(src, dst)
        elif not os.path.exists(dst):
            problems.append("move: neither %s nor %s exists" % (src, dst))
    elif kind == "delete":
        if os.path.exists(op["path"]):
            os.remove(op["path"])
    elif kind == "asset":
        data = open(os.path.join(HERE, op["asset"]), "rb").read()
        path = op["path"]
        if not os.path.exists(path) or open(path, "rb").read() != data:
            os.makedirs(os.path.dirname(path) or ".", exist_ok=True)
            with open(path, "wb") as f:
                f.write(data)
    elif kind == "replace":
        path = op["path"]
        if not os.path.exists(path):
            problems.append("replace: %s is missing" % path)
            continue
        text = read(path)
        for hunk in op["hunks"]:
            if hunk["new"] in text:
                continue
            if text.count(hunk["old"]) == 1:
                text = text.replace(hunk["old"], hunk["new"], 1)
            else:
                problems.append("replace: drift in %s (expected block found %d times)" % (path, text.count(hunk["old"])))
        write(path, text)
    else:
        problems.append("unknown op %r" % kind)

if problems:
    sys.stderr.write("\n".join(problems) + "\n")
    sys.exit(1)
