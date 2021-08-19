import math
from pathlib import Path

import pytest
import ufoLib2

import iondrive

UFOS = [
    Path("tests/data/MutatorSansBoldCondensed.ufo"),
    Path("tests/data/UbuTestData.ufo"),
]


def close_enough(a: float, b: float) -> bool:
    """Determine whether norad's f32 decimals are close enough to Python's
    f64 decimals."""
    return math.isclose(a, b, rel_tol=1e-07)


@pytest.mark.parametrize("path", UFOS, ids=lambda p: p.name)
def test_equivalence(path: Path) -> None:
    font = ufoLib2.Font.open(path)
    id_font = iondrive.load(ufoLib2.objects, path)

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
            # Compare fields one by one so we can use approximate equal comparisons.
            # Expand as necessary.
            glyph = layer[glyph_name]
            id_glyph = id_layer[glyph_name]
            assert glyph.name == id_glyph.name
            assert close_enough(glyph.width, id_glyph.width)
            assert close_enough(glyph.height, id_glyph.height)
            assert glyph.unicodes == id_glyph.unicodes
            assert glyph.image == id_glyph.image
            assert glyph.lib == id_glyph.lib
            assert glyph.note == id_glyph.note
            assert glyph.anchors == id_glyph.anchors
            assert glyph.components == id_glyph.components
            assert len(glyph.contours) == len(id_glyph.contours)
            for contour, id_contour in zip(glyph.contours, id_glyph.contours):
                assert contour.identifier == id_contour.identifier
                assert len(contour.points) == len(id_contour.points)
                for point, id_point in zip(contour.points, id_contour.points):
                    assert close_enough(point.x, id_point.x)
                    assert close_enough(point.y, id_point.y)
                    assert point.type == id_point.type
                    assert point.smooth == id_point.smooth
                    assert point.name == id_point.name
                    assert point.identifier == id_point.identifier
            assert glyph.guidelines == id_glyph.guidelines

    # Not supported by norad yet:
    # assert font.data == id_font.data
    # assert font.images == id_font.images
