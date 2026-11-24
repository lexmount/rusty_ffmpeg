"""Extract the H.264 build source subset from the official FFmpeg release."""

import argparse
import hashlib
import tarfile
from pathlib import Path


ROOT = "ffmpeg-8.1.3"
SHA256 = "7138d28c96d9d3e3af4ee3d8cad72741f8ffb40da90c1112235dea3ecd3178a3"
LIBRARY_TREES = ("compat", "ffbuild", "libavcodec", "libavformat", "libavutil", "libswscale")
MAKEFILE_TREES = ("doc", "fftools", "tests", "tools")


def includes(name: str) -> bool:
    base, _, path = name.partition("/")
    if base != ROOT:
        return False
    if not path or path in ("Makefile", "configure", "VERSION", "RELEASE", "LICENSE.md"):
        return True
    if path.startswith("COPYING."):
        return True
    tree = path.split("/", 1)[0]
    return tree in LIBRARY_TREES or (
        tree in MAKEFILE_TREES
        and (path.endswith(("/Makefile", ".mak")) or path == tree)
    )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("upstream", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    if hashlib.sha256(args.upstream.read_bytes()).hexdigest() != SHA256:
        parser.error("expected the official FFmpeg 8.1.3 release archive")
    with tarfile.open(args.upstream, "r:xz") as source:
        with tarfile.open(args.output, "w:xz") as output:
            for item in source:
                if includes(item.name):
                    output.addfile(item, source.extractfile(item) if item.isfile() else None)


if __name__ == "__main__":
    main()
