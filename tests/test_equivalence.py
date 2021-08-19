from pathlib import Path

import pytest
import ufoLib2

import iondrive

UFOS = [
    Path("tests/data/MutatorSansBoldCondensed.ufo"),
    Path("tests/data/UbuTestData.ufo"),
]


@pytest.mark.parametrize("path", UFOS, ids=lambda p: p.name)
def test_equivalence(path: Path) -> None:
    ufo_font = ufoLib2.Font.open(path)
    iondrive_font = iondrive.load(ufoLib2.objects, path)

    assert ufo_font.lib == iondrive_font.lib
    assert ufo_font.groups == iondrive_font.groups
    assert ufo_font.kerning == iondrive_font.kerning
    assert ufo_font.features == iondrive_font.features
    assert ufo_font.info == iondrive_font.info
    assert len(ufo_font.layers) == len(iondrive_font.layers)
    for layer_name in ufo_font.layers.keys():
        ufo_layer = ufo_font.layers[layer_name]
        iondrive_layer = iondrive_font.layers[layer_name]
        assert ufo_layer.color == iondrive_layer.color
        assert ufo_layer.lib == iondrive_layer.lib
        assert ufo_layer.keys() == iondrive_layer.keys()
        for glyph_name in ufo_layer.keys():
            assert ufo_layer[glyph_name] == iondrive_layer[glyph_name]

    # Not supported by norad yet:
    # assert ufo_font.data == iondrive_font.data
    # assert ufo_font.images == iondrive_font.images
