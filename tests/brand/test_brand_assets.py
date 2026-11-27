"""The brand assets follow the startr.trade family file set (FEAT-003).

Run from the repository root: python3 -m pytest -v tests/brand/test_brand_assets.py
"""
import struct
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BRAND = ROOT / "assets" / "brand"
P = "koshr"
SVGS = {f"{P}-mark.svg": (160, 160), f"{P}-lockup-light.svg": (720, 200), f"{P}-lockup-dark.svg": (720, 200),
        f"{P}-favicon.svg": (128, 128), f"{P}-app-icon.svg": (180, 180), f"{P}-social-preview.svg": (1280, 640)}
PNGS = {"apple-touch-icon.png": (180, 180), f"{P}-social-preview.png": (1280, 640)}


def png_size(path):
    head = path.read_bytes()[:24]
    assert head[:8] == b"\x89PNG\r\n\x1a\n", f"{path.name} is not a PNG"
    return struct.unpack(">II", head[16:24])


def check_asset_set():
    for name, (w, h) in SVGS.items():
        root = ET.parse(BRAND / name).getroot()
        box = [float(v) for v in root.get("viewBox").split()]
        assert box[2:] == [w, h], f"{name}: viewBox {box}"
        assert root.get("aria-label"), f"{name}: no aria-label"
    for name, size in PNGS.items():
        assert png_size(BRAND / name) == size, f"{name}: size"
    light = (BRAND / f"{P}-lockup-light.svg").read_text(encoding="utf-8")
    dark = (BRAND / f"{P}-lockup-dark.svg").read_text(encoding="utf-8")
    assert "#26317E" in light and "#AEB6E8" in dark, "lockups not in the family palette"
    assert f">{P}<" in light, "the wordmark is not the lower-case name"
    assert (BRAND / "brand.py").exists(), "the generator is missing"


# R-2
def test_brand_assets_are_present():
    check_asset_set()


# R-1
def test_readme_has_the_light_and_dark_lockup_picture():
    readme = (ROOT / "README.md").read_text(encoding="utf-8")
    start = readme.index("<picture>")
    pic = readme[start:readme.index("</picture>", start)]
    assert 'media="(prefers-color-scheme: dark)"' in pic and "koshr-lockup-dark.svg" in pic
    assert 'media="(prefers-color-scheme: light)"' in pic and "koshr-lockup-light.svg" in pic
    assert '<img src="assets/brand/koshr-lockup-light.svg"' in pic and 'alt="koshr' in pic
    assert readme.index("<picture>") < 200, "the README does not open with the lockup"
