#!/bin/sh
# Packs the native mod (.hnmod) the Horizon registry takes, into dist/.
set -e
here="$(cd "$(dirname "$0")" && pwd)"
mkdir -p "$here/dist"
python3 -I - "$here" <<'EOF'
import json, sys, zipfile, pathlib, tomllib
root = pathlib.Path(sys.argv[1])
bundle = root / "mixins/fullscreen_fix"
mixin = tomllib.loads((bundle / "horizon-mixin.toml").read_text())
version = mixin["mod"]["version"]
meta = {"game": "26.3", "horizon": mixin["compatibility"]["horizon_revision"], "id": mixin["mod"]["id"],
        "mixins": "5a64af938ec86b00d53d16a408aabc0038fb3e57", "schema": 1, "version": version}
(root / "horizon-native.json").write_text(json.dumps(meta, indent=2, sort_keys=True) + "\n")
out = root / "dist" / f"fullscreen_fix-{version}.hnmod"
with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
    def add(name, data):
        info = zipfile.ZipInfo(name, (2026, 1, 1, 0, 0, 0)); info.compress_type = zipfile.ZIP_DEFLATED
        z.writestr(info, data)
    add("horizon-native.json", (root / "horizon-native.json").read_bytes())
    for f in sorted(bundle.glob("*.rs")) + [bundle / "horizon-mixin.toml"]:
        add(f"mixins/fullscreen_fix/{f.name}", f.read_bytes())
    add("mixins/fullscreen_fix/LICENSE", (root / "LICENSE").read_bytes())
print(out)
EOF
for f in "$here"/dist/fullscreen_fix-*.hnmod; do
  echo "$f  $(shasum -a 256 "$f" | cut -d' ' -f1)"
done
