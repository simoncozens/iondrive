import iondrive
import ufoLib2

ufo_font = ufoLib2.Font.open("tests/data/MutatorSansBoldCondensed.ufo")
iondrive_font = iondrive.load(ufoLib2.objects, "tests/data/MutatorSansBoldCondensed.ufo")

def test_counts():
	assert len(ufo_font.layers) == len(iondrive_font.layers)
	assert len(ufo_font) == len(iondrive_font)

def test_kerning():
	assert ufo_font.kerning == iondrive_font.kerning

def test_features():
	assert ufo_font.features == iondrive_font.features
