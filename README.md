# iondrive: Accelerate your UFO!

`iondrive` wraps the Rust [norad](https://docs.rs/norad) crate in Python
to provide accelerated loading for UFO files using `ufoLib2` or compatible
interfaces.

## Usage

`iondrive` is intended to be used _by_ the UFO library, but to use it
yourself, you need to tell it about the UFO module it should use:

```
import ufoLib2
import iondrive

f = iondrive.load(ufoLib2.objects, filename)
```

This will load a UFO file into `ufoLib2.objects.Font` object. The file is
fully converted to the UFO library's objects, and you can then use them
as normal. It is only the _loading_ process which is accelerated.

## Building

Use `maturin` to build `iondrive`.

```
pip3 install maturin
maturin develop # In a virtualenv
maturin build # Build wheel
```
