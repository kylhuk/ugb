#!/usr/bin/env python3
import argparse
import json
import tarfile
from datetime import datetime, timezone
from pathlib import Path


def read_ipdeny_tar(tar_gz_path: Path) -> dict[str, list[str]]:
    out: dict[str, list[str]] = {}
    with tarfile.open(tar_gz_path, "r:gz") as tf:
        for m in tf.getmembers():
            if not m.isfile():
                continue
            name = Path(m.name).name
            if not name.endswith(".zone"):
                continue
            iso2 = name[:-5].lower()  # "cn.zone" -> "cn"
            f = tf.extractfile(m)
            if f is None:
                continue
            lines = f.read().decode("utf-8", errors="replace").splitlines()
            cidrs = [ln.strip() for ln in lines if ln.strip() and not ln.strip().startswith("#")]
            out[iso2] = cidrs
    return out


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--meta", required=True, help="Path to Dhall-generated meta JSON")
    ap.add_argument("--ipdeny4", required=True, help="Path to IPdeny IPv4 all-zones.tar.gz")
    ap.add_argument("--ipdeny6", required=True, help="Path to IPdeny IPv6 ipv6-all-zones.tar.gz")
    ap.add_argument("--out", required=True, help="Output path for db/geo.json")
    ap.add_argument("--allow-missing", action="store_true", help="Do not fail if some ISO2 missing in tarballs")
    args = ap.parse_args()

    meta = json.loads(Path(args.meta).read_text(encoding="utf-8"))
    v4 = read_ipdeny_tar(Path(args.ipdeny4))
    v6 = read_ipdeny_tar(Path(args.ipdeny6))

    missing = []
    for cont in meta["continents"]:
        for c in cont["countries"]:
            iso2 = c["iso2"].lower()
            if iso2 not in v4 and iso2 not in v6:
                missing.append(c["iso2"])

    if missing and not args.allow_missing:
        raise SystemExit(f"Missing ISO2 in IPdeny tarballs: {', '.join(sorted(set(missing)))}")

    out = {
        "version": int(meta.get("version", 1)),
        "generatedAt": datetime.now(timezone.utc).isoformat(),
        "continents": [],
    }

    for cont in meta["continents"]:
        cont_out = {"name": cont["name"], "countries": []}
        for c in cont["countries"]:
            iso2u = c["iso2"]
            iso2 = iso2u.lower()
            cidrs4 = sorted(set(v4.get(iso2, [])))
            cidrs6 = sorted(set(v6.get(iso2, [])))
            cont_out["countries"].append(
                {
                    "name": c["name"],
                    "iso2": iso2u,
                    "aliases": c.get("aliases", []),
                    "cidrs4": cidrs4,
                    "cidrs6": cidrs6,
                }
            )
        out["continents"].append(cont_out)

    Path(args.out).parent.mkdir(parents=True, exist_ok=True)
    Path(args.out).write_text(json.dumps(out, indent=2), encoding="utf-8")


if __name__ == "__main__":
    main()

