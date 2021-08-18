from pathlib import Path

import pytest
import ufoLib2

import iondrive

UFOS = [
    Path("tests/data/MutatorSansBoldCondensed.ufo"),
    Path("tests/data/UbuTestData.ufo"),
]


@pytest.mark.parametrize("path", UFOS)
def test_equivalence(path: Path) -> None:
    ufo_font = ufoLib2.Font.open(path)
    iondrive_font = iondrive.load(ufoLib2.objects, path)

    assert len(ufo_font) == len(iondrive_font)
    assert len(ufo_font.layers) == len(iondrive_font.layers)
    assert ufo_font.groups == iondrive_font.groups
    assert ufo_font.kerning == iondrive_font.kerning
    assert ufo_font.features == iondrive_font.features
