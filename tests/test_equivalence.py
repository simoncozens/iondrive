import math
from pathlib import Path
from typing import Optional, Set

import pytest
import ufoLib2
import ufoLib2.objects

import iondrive

UFOS = [
    Path("tests/data/MutatorSansBoldCondensed.ufo"),
    Path("tests/data/UbuTestData.ufo"),
    Path("tests/data/SourceSans_ExtraLight.ufo"),
    Path("tests/data/NotoSans-Regular.ufo"),
    Path("tests/data/Empty.ufo"),
    Path("tests/data/dataimagetest.ufo"),
    Path("tests/data/identifiers.ufo"),
]


def identifiers_in_glyph(glyph: ufoLib2.objects.Glyph) -> Set[Optional[str]]:
    ids = set()
    ids.update(a.identifier for a in glyph.anchors)
    ids.update(g.identifier for g in glyph.guidelines)
    ids.update(c.identifier for c in glyph.components)
    ids.update(c.identifier for c in glyph.contours)
    ids.update(p.identifier for c in glyph.contours for p in c.points)
    return ids


@pytest.mark.parametrize("path", UFOS, ids=lambda p: p.name)
def test_equivalence(path: Path) -> None:
    font = ufoLib2.Font.open(path)
    id_font = iondrive.load(ufoLib2.objects, path)

    # Compare fields one by one so we can clean up known differences and get smaller diffs.
    assert font.lib == id_font.lib
    assert font.groups == id_font.groups
    assert font.kerning == id_font.kerning
    assert font.features == id_font.features
    assert font.info == id_font.info
    assert font.layers.keys() == id_font.layers.keys()
    for layer_name in font.layers.keys():
        layer = font.layers[layer_name]
        id_layer = id_font.layers[layer_name]
        assert layer.color == id_layer.color
        assert layer.lib == id_layer.lib
        assert layer.keys() == id_layer.keys()
        for glyph_name in layer.keys():
            glyph = layer[glyph_name]
            id_glyph = id_layer[glyph_name]
            assert glyph.name == id_glyph.name
            assert glyph.width == id_glyph.width
            assert glyph.height == id_glyph.height
            assert glyph.unicodes == id_glyph.unicodes
            assert glyph.image == id_glyph.image
            if "public.objectLibs" in glyph.lib:
                # norad does not keep object libs for objects it can't find, so we need
                # to filter them out for the comparison.
                glyph.lib["public.objectLibs"] = {
                    k: v
                    for k, v in glyph.lib["public.objectLibs"].items()
                    if k in identifiers_in_glyph(glyph)
                }
            assert glyph.lib == id_glyph.lib
            assert glyph.note == id_glyph.note
            assert glyph.anchors == id_glyph.anchors
            assert glyph.components == id_glyph.components
            assert glyph.contours == id_glyph.contours
            assert glyph.guidelines == id_glyph.guidelines

    assert font.data == id_font.data
    assert font.images == id_font.images
